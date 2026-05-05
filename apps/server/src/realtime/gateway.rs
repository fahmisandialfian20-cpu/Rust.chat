use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::error::AppError;
use crate::permissions::PermissionKey;
use crate::realtime::events::{ErrorData, WsEvent};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsCommand {
    #[serde(rename = "send_message")]
    SendMessage { channel_id: Uuid, content: String },
    #[serde(rename = "join_channel")]
    JoinChannel { channel_id: Uuid },
    #[serde(rename = "leave_channel")]
    LeaveChannel { channel_id: Uuid },
    #[serde(rename = "typing")]
    Typing { channel_id: Uuid, is_typing: bool },
}

pub async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth_user.user_id_uuid()?;

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, user_id, auth_user.session_id)))
}

async fn handle_socket(socket: WebSocket, state: AppState, user_id: Uuid, session_id: String) {
    let (mut sender, mut receiver) = socket.split();
    let hub = state.realtime_hub.clone();

    let hello = WsEvent::Hello(crate::realtime::events::HelloData {
        user_id,
        session_id: session_id.clone(),
    });

    match hello.to_json() {
        Ok(json) => {
            if sender.send(Message::Text(json.into())).await.is_err() {
                return;
            }
        }
        Err(e) => {
            eprintln!("WS serialize error: {}", e);
            return;
        }
    }

    state.presence_service.set_online(user_id).await.ok();

    let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = cmd_rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let receive_task = tokio::spawn(async move {
        let mut joined_channels: HashMap<Uuid, tokio::task::AbortHandle> = HashMap::new();
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(cmd) = serde_json::from_str::<WsCommand>(&text) {
                        match cmd {
                            WsCommand::SendMessage {
                                channel_id,
                                content,
                            } => {
                                match state
                                    .message_service
                                    .create_message(
                                        channel_id,
                                        user_id,
                                        crate::domain::message::CreateMessage {
                                            content,
                                            kind: None,
                                            reply_to_message_id: None,
                                        },
                                    )
                                    .await
                                {
                                    Ok(msg) => {
                                        let event = WsEvent::MessageCreated(
                                            crate::realtime::events::MessageCreatedData {
                                                message: msg,
                                            },
                                        );
                                        match event.to_json() {
                                            Ok(json) => {
                                                hub.publish_to_channel(channel_id, json).await;
                                            }
                                            Err(e) => {
                                                eprintln!("WS serialize error: {}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let err = WsEvent::Error(ErrorData {
                                            code: "forbidden".to_string(),
                                            message: format!("{}", e),
                                        });
                                        match err.to_json() {
                                            Ok(json) => {
                                                if cmd_tx.send(json).is_err() {
                                                    break;
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("WS serialize error: {}", e);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            WsCommand::JoinChannel { channel_id } => {
                                if let std::collections::hash_map::Entry::Vacant(e) =
                                    joined_channels.entry(channel_id)
                                {
                                    let channel =
                                        state.channel_service.get_channel(channel_id).await;
                                    match channel {
                                        Ok(ch) => {
                                            let perm = state
                                                .permission_service
                                                .check(
                                                    user_id,
                                                    PermissionKey::ViewChannel,
                                                    Some(ch.space_id),
                                                    Some(channel_id),
                                                )
                                                .await;
                                            if perm.is_ok() {
                                                let rx = hub.subscribe(channel_id).await;
                                                let forward_tx = cmd_tx.clone();
                                                let handle = tokio::spawn(async move {
                                                    let mut rx = rx;
                                                    while let Ok(msg) = rx.recv().await {
                                                        if forward_tx.send(msg).is_err() {
                                                            break;
                                                        }
                                                    }
                                                });
                                                e.insert(handle.abort_handle());
                                            } else {
                                                let err = WsEvent::Error(ErrorData {
                                                    code: "forbidden".to_string(),
                                                    message: "No permission to view channel"
                                                        .to_string(),
                                                });
                                                match err.to_json() {
                                                    Ok(json) => {
                                                        if cmd_tx.send(json).is_err() {
                                                            break;
                                                        }
                                                    }
                                                    Err(e) => {
                                                        eprintln!("WS serialize error: {}", e);
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            let err = WsEvent::Error(ErrorData {
                                                code: "not_found".to_string(),
                                                message: "Channel not found".to_string(),
                                            });
                                            match err.to_json() {
                                                Ok(json) => {
                                                    if cmd_tx.send(json).is_err() {
                                                        break;
                                                    }
                                                }
                                                Err(e) => {
                                                    eprintln!("WS serialize error: {}", e);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            WsCommand::LeaveChannel { channel_id } => {
                                if let Some(handle) = joined_channels.remove(&channel_id) {
                                    handle.abort();
                                }
                            }
                            WsCommand::Typing {
                                channel_id,
                                is_typing: _,
                            } => {
                                state
                                    .typing_service
                                    .set_typing(channel_id, user_id)
                                    .await
                                    .ok();
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(_) => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_task => {}
    }

    state.presence_service.set_offline(user_id).await.ok();
}

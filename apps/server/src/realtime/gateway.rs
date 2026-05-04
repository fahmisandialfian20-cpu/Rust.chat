use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::error::AppError;
use crate::realtime::events::{WsEvent, HelloData};
use crate::state::AppState;

pub async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth_user.user_id_uuid()?;

    Ok(ws.on_upgrade(move |socket| {
        handle_socket(socket, state, user_id, auth_user.session_id)
    }))
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    user_id: Uuid,
    session_id: String,
) {
    let (mut sender, mut receiver) = socket.split();

    let hello = WsEvent::Hello(HelloData {
        user_id,
        session_id: session_id.clone(),
    });

    if sender.send(Message::Text(hello.to_json().into())).await.is_err() {
        return;
    }

    state.presence_service.set_online(user_id).await.ok();

    let hub = state.realtime_hub.clone();
    let mut rx = hub.subscribe();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let receive_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(event) = serde_json::from_str::<WsEvent>(&text) {
                        hub.publish(event);
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

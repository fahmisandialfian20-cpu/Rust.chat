use axum::{
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::Json,
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::file_object::FileObject;
use crate::error::AppError;
use crate::middleware::rate_limit;
use crate::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct UploadResponse {
    pub file: FileObject,
    pub download_url: String,
}

fn extract_user_id(state: &AppState, headers: &axum::http::HeaderMap) -> Result<Uuid, AppError> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_token(auth_header)?;
    Uuid::parse_str(&claims.claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user".to_string()))
}

#[utoipa::path(
    post,
    path = "/api/v1/files/upload",
    tag = "files",
    responses(
        (status = 200, description = "File uploaded", body = UploadResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn upload_file(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    let user_id = extract_user_id(&state, &headers)?;

    let key = rate_limit::upload_key(&user_id.to_string());
    state
        .rate_limiter
        .check(&key, state.config.rate_limit.file_upload, 60)
        .await?;

    let mut filename = String::new();
    let mut content_type = String::new();
    let mut data = Vec::new();
    let mut space_id: Option<Uuid> = None;
    let mut channel_id: Option<Uuid> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                filename = field.file_name().unwrap_or("unknown").to_string();
                content_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();
                data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?
                    .to_vec();
            }
            "space_id" => {
                let val = field
                    .text()
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?;
                if !val.is_empty() {
                    space_id = Some(
                        Uuid::parse_str(&val)
                            .map_err(|_| AppError::BadRequest("Invalid space_id".to_string()))?,
                    );
                }
            }
            "channel_id" => {
                let val = field
                    .text()
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?;
                if !val.is_empty() {
                    channel_id = Some(
                        Uuid::parse_str(&val)
                            .map_err(|_| AppError::BadRequest("Invalid channel_id".to_string()))?,
                    );
                }
            }
            _ => {}
        }
    }

    if data.is_empty() {
        return Err(AppError::BadRequest("No file data provided".to_string()));
    }

    let file = state
        .file_service
        .upload(space_id, channel_id, user_id, filename, content_type, data)
        .await?;

    state
        .audit_service
        .log(
            crate::services::audit_service::FILE_UPLOAD,
            user_id,
            space_id,
            None,
            None,
            channel_id,
            None,
            None,
        )
        .await?;

    let download_url = state
        .file_service
        .get_download_url(file.id, user_id)
        .await?;

    Ok(Json(UploadResponse { file, download_url }))
}

#[utoipa::path(
    get,
    path = "/api/v1/files/{file_id}",
    tag = "files",
    params(
        ("file_id" = Uuid, Path, description = "File UUID"),
    ),
    responses(
        (status = 200, description = "File metadata", body = FileObject),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "File not found"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn get_file(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(file_id): Path<Uuid>,
) -> Result<Json<FileObject>, AppError> {
    let user_id = extract_user_id(&state, &headers)?;
    let file = state.file_service.get_file(file_id).await?;
    state
        .file_service
        .get_download_url(file_id, user_id)
        .await?;
    Ok(Json(file))
}

#[utoipa::path(
    delete,
    path = "/api/v1/files/{file_id}",
    tag = "files",
    params(
        ("file_id" = Uuid, Path, description = "File UUID"),
    ),
    responses(
        (status = 204, description = "File deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "File not found"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn delete_file(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Path(file_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = extract_user_id(&state, &headers)?;
    state.file_service.delete_file(file_id, user_id).await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::FILE_DELETE,
            user_id,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{delete, get, post};

    axum::Router::new()
        .route("/files/upload", post(upload_file))
        .route("/files/{file_id}", get(get_file))
        .route("/files/{file_id}", delete(delete_file))
}

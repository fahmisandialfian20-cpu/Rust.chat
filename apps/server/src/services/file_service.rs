use std::sync::Arc;
use uuid::Uuid;

use crate::domain::file_object::FileObject;
use crate::error::AppError;
use crate::repositories::file_repository::FileRepository;
use crate::storage::FileStorage;
use crate::permissions::{PermissionKey, PermissionService};

const MIME_ALLOWLIST: &[&str] = &[
    "image/jpeg", "image/png", "image/gif", "image/webp",
    "application/pdf",
    "text/plain",
    "application/zip", "application/x-tar", "application/gzip",
];

#[derive(Clone)]
pub struct FileService {
    repo: FileRepository,
    storage: Arc<dyn FileStorage>,
    permission_service: PermissionService,
    max_upload_bytes: i64,
}

impl FileService {
    pub fn new(
        repo: FileRepository,
        storage: Arc<dyn FileStorage>,
        permission_service: PermissionService,
        max_upload_bytes: i64,
    ) -> Self {
        Self { repo, storage, permission_service, max_upload_bytes }
    }

    pub async fn upload(
        &self,
        space_id: Option<Uuid>,
        channel_id: Option<Uuid>,
        user_id: Uuid,
        filename: String,
        content_type: String,
        data: Vec<u8>,
    ) -> Result<FileObject, AppError> {
        let size = data.len() as i64;

        if size > self.max_upload_bytes {
            return Err(AppError::BadRequest(format!(
                "File too large: {} bytes (max: {})",
                size, self.max_upload_bytes
            )));
        }

        let content_lower = content_type.to_lowercase();
        if !MIME_ALLOWLIST.contains(&content_lower.as_str()) {
            return Err(AppError::BadRequest(format!(
                "Content type not allowed: {}",
                content_type
            )));
        }

        if let Some(cid) = channel_id {
            self.permission_service.check(user_id, PermissionKey::SendFiles, space_id, Some(cid)).await?;
        }

        let storage_key = format!("{}/{}", user_id, Uuid::now_v7());

        self.storage.upload(&storage_key, &data, &content_type)
            .await
            .map_err(|e| AppError::InternalServerError(e))?;

        let file = self.repo.create(
            space_id,
            channel_id,
            user_id,
            filename,
            content_type,
            size,
            storage_key,
        ).await?;

        Ok(file)
    }

    pub async fn get_download_url(&self, file_id: Uuid, user_id: Uuid) -> Result<String, AppError> {
        let file = self.repo.find_by_id(file_id).await?;

        if let Some(cid) = file.channel_id {
            self.permission_service.check_optional(user_id, PermissionKey::ViewChannel, file.space_id, Some(cid)).await?;
        }

        self.storage.download_url(&file.storage_key)
            .await
            .map_err(|e| AppError::InternalServerError(e))
    }

    pub async fn get_file(&self, file_id: Uuid) -> Result<FileObject, AppError> {
        self.repo.find_by_id(file_id).await
    }

    pub async fn delete_file(&self, file_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let file = self.repo.find_by_id(file_id).await?;

        if file.uploader_user_id != user_id {
            return Err(AppError::Forbidden("You can only delete your own files".to_string()));
        }

        self.storage.delete(&file.storage_key)
            .await
            .map_err(|e| AppError::InternalServerError(e))?;

        self.repo.delete(file_id).await
    }
}

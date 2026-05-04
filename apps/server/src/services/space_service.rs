use uuid::Uuid;
use crate::domain::space::{Space, CreateSpace, UpdateSpace};
use crate::domain::membership::{SpaceMembership, AddMember};
use crate::repositories::space_repository::SpaceRepository;
use crate::error::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct SpaceService {
    repository: Arc<SpaceRepository>,
}

impl SpaceService {
    pub fn new(repository: Arc<SpaceRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_space(
        &self,
        user_id: Uuid,
        input: CreateSpace,
    ) -> Result<Space, AppError> {
        let slug = Self::generate_slug(&input.name);
        
        if self.repository.slug_exists(&slug).await? {
            return Err(AppError::Conflict("Space with similar name already exists".to_string()));
        }

        let visibility = input.visibility
            .as_ref()
            .and_then(|v| v.parse().ok())
            .unwrap_or_default();

        let space = self.repository.create(
            input.name,
            slug,
            input.description,
            user_id,
            visibility,
        ).await?;

        self.repository.add_member(space.id, user_id, None).await?;

        Ok(space)
    }

    pub async fn get_space(&self, space_id: Uuid) -> Result<Space, AppError> {
        self.repository.find_by_id(space_id).await
    }

    pub async fn get_space_by_slug(&self, slug: &str) -> Result<Space, AppError> {
        self.repository.find_by_slug(slug).await
    }

    pub async fn list_spaces(&self, limit: i64, offset: i64) -> Result<Vec<Space>, AppError> {
        self.repository.find_all(limit, offset).await
    }

    pub async fn list_user_spaces(&self, user_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Space>, AppError> {
        self.repository.find_by_user(user_id, limit, offset).await
    }

    pub async fn update_space(&self, space_id: Uuid, input: UpdateSpace) -> Result<Space, AppError> {
        self.repository.update(
            space_id,
            input.name,
            input.description,
            input.visibility,
        ).await
    }

    pub async fn delete_space(&self, space_id: Uuid) -> Result<(), AppError> {
        self.repository.delete(space_id).await
    }

    pub async fn add_member(&self, space_id: Uuid, input: AddMember) -> Result<SpaceMembership, AppError> {
        if self.repository.is_member(space_id, input.user_id).await? {
            return Err(AppError::Conflict("User is already a member".to_string()));
        }

        self.repository.add_member(space_id, input.user_id, input.nickname).await
    }

    pub async fn remove_member(&self, space_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.remove_member(space_id, user_id).await
    }

    pub async fn get_members(&self, space_id: Uuid, limit: i64, offset: i64) -> Result<Vec<SpaceMembership>, AppError> {
        self.repository.get_members(space_id, limit, offset).await
    }

    pub async fn get_member(&self, space_id: Uuid, user_id: Uuid) -> Result<SpaceMembership, AppError> {
        self.repository.get_member(space_id, user_id).await
    }

    pub async fn is_member(&self, space_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        self.repository.is_member(space_id, user_id).await
    }

    fn generate_slug(name: &str) -> String {
        let slug = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>();

        let slug = slug.trim_matches('-').to_string();
        
        if slug.is_empty() {
            format!("space-{}", &Uuid::new_v4().to_string()[..8])
        } else {
            slug
        }
    }
}
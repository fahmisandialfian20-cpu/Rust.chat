use std::sync::Arc;

use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::auth::jwt::JwtManager;
use crate::config::AppConfig;
use crate::middleware::rate_limit::RateLimiter;
use crate::permissions::PermissionService;
use crate::realtime::hub::RealtimeHub;
use crate::services::auth_service::AuthService;
use crate::services::file_service::FileService;
use crate::services::space_service::SpaceService;
use crate::services::channel_service::ChannelService;
use crate::services::invite_service::InviteService;
use crate::services::message_service::MessageService;
use crate::services::presence_service::PresenceService;
use crate::services::typing_service::TypingService;
use crate::services::audit_service::AuditService;
use crate::services::role_service::RoleService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub config: AppConfig,
    pub jwt_manager: JwtManager,
    pub auth_service: Arc<AuthService>,
    pub file_service: FileService,
    pub space_service: SpaceService,
    pub channel_service: ChannelService,
    pub invite_service: InviteService,
    pub message_service: MessageService,
    pub presence_service: PresenceService,
    pub typing_service: TypingService,
    pub permission_service: PermissionService,
    pub audit_service: AuditService,
    pub role_service: RoleService,
    pub realtime_hub: Arc<RealtimeHub>,
    pub rate_limiter: RateLimiter,
}
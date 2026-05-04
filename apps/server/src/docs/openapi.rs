use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust.chat API",
        description = "Self-hosted chat platform API. Provides REST endpoints for authentication, spaces, channels, messages, file uploads, media tokens, and admin operations.",
        version = "0.1.0",
        contact(name = "Rust.chat", url = "https://rust.chat"),
    ),
    servers(
        (url = "/", description = "Current server"),
    ),
    paths(
        crate::routes::health::healthz,
        crate::routes::health::readyz,
        crate::handlers::auth::bootstrap_owner,
        crate::handlers::auth::register,
        crate::handlers::auth::login,
        crate::handlers::auth::logout,
        crate::handlers::auth::refresh,
        crate::handlers::auth::me,
        crate::handlers::auth::list_devices,
        crate::handlers::auth::revoke_device,
        crate::handlers::spaces::create_space,
        crate::handlers::spaces::get_space,
        crate::handlers::spaces::get_space_by_slug,
        crate::handlers::spaces::list_spaces,
        crate::handlers::spaces::list_user_spaces,
        crate::handlers::spaces::update_space,
        crate::handlers::spaces::delete_space,
        crate::handlers::spaces::add_member,
        crate::handlers::spaces::remove_member,
        crate::handlers::spaces::list_members,
        crate::handlers::spaces::get_member,
        crate::handlers::channels::create_channel,
        crate::handlers::channels::get_channel,
        crate::handlers::channels::get_channel_by_slug,
        crate::handlers::channels::list_channels,
        crate::handlers::channels::list_visible_channels,
        crate::handlers::channels::update_channel,
        crate::handlers::channels::archive_channel,
        crate::handlers::channels::delete_channel,
        crate::handlers::channels::get_channel_feature_flags,
        crate::handlers::channels::update_channel_feature_flags,
        crate::handlers::channels::add_channel_member,
        crate::handlers::channels::remove_channel_member,
        crate::handlers::messages::create_message,
        crate::handlers::messages::get_message,
        crate::handlers::messages::list_messages,
        crate::handlers::messages::update_message,
        crate::handlers::messages::delete_message,
        crate::handlers::invites::create_invite,
        crate::handlers::invites::get_invite,
        crate::handlers::invites::get_invite_by_code,
        crate::handlers::invites::validate_invite,
        crate::handlers::invites::consume_invite,
        crate::handlers::invites::list_space_invites,
        crate::handlers::invites::delete_invite,
        crate::handlers::files::upload_file,
        crate::handlers::files::get_file,
        crate::handlers::files::delete_file,
        crate::handlers::media::create_media_token,
        crate::handlers::admin::list_audit_logs,
    ),
    components(
        schemas(
            crate::domain::user::User,
            crate::domain::user::UserStatus,
            crate::domain::user::UserProfile,
            crate::domain::user::ClientInfo,
            crate::domain::user::ClientDevice,
            crate::domain::space::Space,
            crate::domain::space::SpaceVisibility,
            crate::domain::space::CreateSpace,
            crate::domain::space::UpdateSpace,
            crate::domain::membership::SpaceMembership,
            crate::domain::membership::AddMember,
            crate::domain::channel::Channel,
            crate::domain::channel::ChannelKind,
            crate::domain::channel::ChannelVisibility,
            crate::domain::channel::ChannelFeatureFlags,
            crate::domain::channel::ChannelFeatureFlagsUpdate,
            crate::domain::channel::CreateChannel,
            crate::domain::channel::UpdateChannel,
            crate::domain::message::Message,
            crate::domain::message::CreateMessage,
            crate::domain::message::UpdateMessage,
            crate::domain::invite::Invite,
            crate::domain::invite::CreateInvite,
            crate::domain::file_object::FileObject,
            crate::domain::audit::AuditEntry,
            crate::services::auth_service::AuthResponse,
            crate::handlers::auth::BootstrapRequest,
            crate::handlers::auth::RegisterRequest,
            crate::handlers::auth::LoginRequest,
            crate::handlers::auth::RefreshRequest,
            crate::handlers::auth::LogoutRequest,
            crate::handlers::files::UploadResponse,
            crate::handlers::media::MediaTokenRequest,
            crate::handlers::media::MediaTokenResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication and account management"),
        (name = "spaces", description = "Space CRUD and membership management"),
        (name = "channels", description = "Channel CRUD, feature flags, and membership"),
        (name = "messages", description = "Message CRUD with cursor-based pagination"),
        (name = "invites", description = "Invite creation, validation, and consumption"),
        (name = "files", description = "File upload, download, and management"),
        (name = "media", description = "LiveKit media token generation for voice/video"),
        (name = "admin", description = "Admin-only operations (audit logs, etc.)"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

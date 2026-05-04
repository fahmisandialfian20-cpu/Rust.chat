use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionKey {
    ManageInstance,
    ManageSpaces,
    ManageRoles,
    ManageMembers,
    ManageChannels,
    ManageInvites,
    ViewAuditLog,

    ViewSpace,

    ViewChannel,
    ReadMessages,
    SendMessages,
    EditOwnMessage,
    DeleteOwnMessage,
    EditAnyMessage,
    DeleteAnyMessage,
    PinMessages,
    MentionEveryone,

    SendFiles,

    CreateThreads,
    ManageThreads,

    AddReactions,

    JoinVoice,
    StartVoice,
    JoinVideo,
    StartVideo,
    ShareScreen,

    KickMembers,
    BanMembers,
    MuteMembers,
    ManageModeration,

    CustomizeOwnProfile,
    CustomizeSpace,
    UseWebhooks,
}

impl PermissionKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionKey::ManageInstance => "manage_instance",
            PermissionKey::ManageSpaces => "manage_spaces",
            PermissionKey::ManageRoles => "manage_roles",
            PermissionKey::ManageMembers => "manage_members",
            PermissionKey::ManageChannels => "manage_channels",
            PermissionKey::ManageInvites => "manage_invites",
            PermissionKey::ViewAuditLog => "view_audit_log",
            PermissionKey::ViewSpace => "view_space",
            PermissionKey::ViewChannel => "view_channel",
            PermissionKey::ReadMessages => "read_messages",
            PermissionKey::SendMessages => "send_messages",
            PermissionKey::EditOwnMessage => "edit_own_message",
            PermissionKey::DeleteOwnMessage => "delete_own_message",
            PermissionKey::EditAnyMessage => "edit_any_message",
            PermissionKey::DeleteAnyMessage => "delete_any_message",
            PermissionKey::PinMessages => "pin_messages",
            PermissionKey::MentionEveryone => "mention_everyone",
            PermissionKey::SendFiles => "send_files",
            PermissionKey::CreateThreads => "create_threads",
            PermissionKey::ManageThreads => "manage_threads",
            PermissionKey::AddReactions => "add_reactions",
            PermissionKey::JoinVoice => "join_voice",
            PermissionKey::StartVoice => "start_voice",
            PermissionKey::JoinVideo => "join_video",
            PermissionKey::StartVideo => "start_video",
            PermissionKey::ShareScreen => "share_screen",
            PermissionKey::KickMembers => "kick_members",
            PermissionKey::BanMembers => "ban_members",
            PermissionKey::MuteMembers => "mute_members",
            PermissionKey::ManageModeration => "manage_moderation",
            PermissionKey::CustomizeOwnProfile => "customize_own_profile",
            PermissionKey::CustomizeSpace => "customize_space",
            PermissionKey::UseWebhooks => "use_webhooks",
        }
    }
}

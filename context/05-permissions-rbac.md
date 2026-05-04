# Permissions and RBAC

The permission model is unchanged by the existence of native clients.

All clients must call the server. The server decides.

## 1. Layers

1. Hoster bypass.
2. Active space membership.
3. Role permissions.
4. Channel overrides.
5. Channel feature flags.
6. Rate limit / moderation state.

## 2. Permission keys

```rust
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
```

## 3. Native-client-specific permissions

Do not create separate permissions such as `send_messages_mobile`.

If a user can send a message, the user can send it from any authorized client.

Platform-specific restrictions should be handled as:

- device trust policy;
- session policy;
- admin security setting;
- not normal chat permissions.

## 4. Feature flag examples

### Send file

```text
has_permission(send_files)
AND channel.send_file_enabled
AND file.size <= limit
```

### Join voice

```text
has_permission(join_voice)
AND channel.voice_group_enabled
AND media provider enabled
```

### Mobile push notification

Push notifications should not reveal private message content unless the user/device notification setting allows it.

## 5. Required tests

- web client cannot bypass server permission;
- desktop client token cannot access private channel without permission;
- mobile bearer token receives same permission result as browser session;
- private attachment URL requires channel access;
- media token requires media permission.

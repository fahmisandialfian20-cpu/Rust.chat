# 01 — Product Scope

## Scope Summary

Rust.chat must become a self-hosted chat app with Discord/Telegram-like concepts, but the current build must stay focused on the MVP core.

The application should eventually feel complete, customizable, and friendly, but the first goal is correctness and permission safety.

---

## Main Product Features

### 1. Lobby / Entry Area

Users can register or log in.

Users may enter the application lobby after authentication.

The lobby is not the same as channel access.

A user in the lobby may still have no space/channel access until invited or granted membership.

### 2. Spaces

The Hoster can create many spaces as long as the server can handle them.

The app starts empty.

No default public room should be assumed.

### 3. Channels

The Hoster can create custom channels inside spaces.

Channels can be:

- public
- private

Public channels are visible to members who have space access and permission.

Private channels require explicit access through role, override, credential, token, or invite flow.

### 4. Roles and Permissions

The Hoster can promote members to admin/moderator roles.

Permissions should be checklist-style.

A role can grant capabilities such as:

- manage space
- manage members
- manage roles
- manage channels
- create invites
- view channel
- read messages
- send messages
- edit own message
- delete own message
- edit any message
- delete any message
- send files
- join voice later
- join video later

### 5. Channel Feature Customization

The Hoster can configure features per channel.

Examples:

- disable voice group
- disable video group
- disable file sending
- restrict message sending
- make channel read-only

Feature flags do not replace permissions.

Correct behavior is:

```text
permission allows action AND channel feature flag allows action
```

### 6. Messaging

Users can exchange real-time text messages in channels they can access.

Message actions must be permission checked.

Message history must be stored durably.

### 7. Personal Customization

Users should eventually be able to customize their personal experience:

- avatar
- display name
- personal theme/skin
- font preference
- UI preference

This is a future user-experience feature, not part of the first backend stabilization task.

### 8. File Storage

The app should eventually support file sharing.

The VPS may have limited disk, so large files should not be stored directly on the VPS forever.

Possible storage directions:

- local storage for development
- S3-compatible storage
- Cloudflare R2
- PikPak adapter using `Quan666/PikPakAPI`
- other pluggable storage provider

For MVP core, keep storage pluggable. Do not hardcode PikPak as the only storage backend.

### 9. Native Clients

The app should eventually have:

- web client
- native desktop client
- mobile client

Current priority is web + backend correctness first.

Desktop and mobile come after stable API contracts.

---

## MVP Core Scope

The MVP core includes:

1. Auth and registration
2. Hoster bootstrap
3. Sessions / tokens
4. Spaces
5. Members
6. Roles
7. Permissions
8. Public/private channels
9. Invites
10. Messages
11. Basic WebSocket realtime
12. PostgreSQL
13. Redis
14. Local dev infrastructure
15. Backend tests

---

## Out of Scope for MVP Core

Do not implement these during MVP core unless the task explicitly says so:

- voice/video
- LiveKit production setup
- mobile app
- desktop app
- advanced file storage
- PikPak integration
- themes and skins
- notification system
- reactions
- threads
- bots
- webhooks
- production deployment automation

---

## Product Quality Direction

The app should feel:

- clean
- natural
- friendly
- not AI-generated
- not overloaded with long text
- easy to understand
- responsive
- suitable for desktop and mobile later

UI quality matters, but UI polish must not happen before core permissions and backend correctness are stable.

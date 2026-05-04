# Product Requirements

## 1. Vision

Rust.chat is a self-hosted chat platform for private communities, teams, families, gaming groups, and organizations. It is inspired by Discord and Telegram, but its authority model is built around the person who deploys the app.

The deployer is called the **Hoster**.

The application must support multiple clients:

- web browser;
- native desktop app;
- native mobile app;
- future bot or CLI clients.

## 2. Product scope

### Core server

The server is the source of truth for:

- identity;
- authentication;
- permissions;
- roles;
- spaces;
- channels;
- messages;
- invites;
- attachments metadata;
- audit logs;
- feature flags;
- media room token authorization.

### Clients

Clients are UI shells over the same server contracts.

A client may cache or hide UI, but it must never become the authority for permissions.

## 3. Actors

### Hoster

The Hoster is the highest authority in one deployment.

Capabilities:

- bootstrap the instance;
- create spaces;
- create channels;
- create roles;
- grant/revoke permissions;
- promote admins/moderators;
- create/revoke invite links;
- control channel feature flags;
- manage storage settings;
- review audit logs.

Constraints:

- normal admins cannot demote/delete the Hoster;
- Hoster bypass must be explicit and auditable;
- MVP supports one primary Hoster.

### Admin

An admin is a member with specific permissions. Admin is not automatically root.

### Moderator

A moderator handles safety and hygiene:

- mute;
- kick;
- ban;
- delete messages;
- lock channels;
- manage reports.

### Member

A registered user who can access only allowed spaces and channels.

### Guest/Pending user

Optional future actor.

## 4. Concepts

### Instance

One deployment of Rust.chat.

### Lobby

The first area after login.

Important rule:

> Lobby access is not channel access.

### Space

A workspace/community/server.

### Channel

A communication room inside a space.

Supported MVP kinds:

- text;
- announcement;
- voice;
- video.

Visibility:

- public;
- private.

Feature flags:

```json
{
  "text_enabled": true,
  "send_file_enabled": true,
  "voice_group_enabled": false,
  "video_group_enabled": false,
  "reactions_enabled": true,
  "threads_enabled": false,
  "mentions_enabled": true,
  "pin_message_enabled": false
}
```

### Invite

Invite can target:

- instance registration;
- a space;
- a private channel.

## 5. MVP requirements

Required:

- Hoster bootstrap;
- auth: register/login/logout;
- invite preview and acceptance;
- spaces;
- public/private channels;
- roles;
- checklist permissions;
- channel permission overrides;
- realtime text chat;
- message history;
- typing indicators;
- presence;
- user profile;
- user theme tokens;
- file storage abstraction;
- local file storage for development;
- audit logs;
- Docker Compose local environment;
- one complete UI client;
- contracts suitable for desktop and mobile.

## 6. Non-goals for first serious MVP

Do not implement:

- custom WebRTC SFU;
- full Discord feature parity;
- federation;
- plugin marketplace;
- arbitrary user CSS;
- complex bot system;
- end-to-end encryption;
- native mobile app before server contracts are stable.

## 7. Recommended product phases

### Phase A: Core server + web/admin client

Build the server and a SvelteKit client first. This proves the product logic.

### Phase B: Desktop client

Package the Svelte UI with Tauri for Windows/macOS/Linux.

### Phase C: Mobile client

Build Flutter mobile app using the same REST/WebSocket/LiveKit contracts.

### Phase D: Advanced features

Add threads, search, push notifications, mobile offline cache, bots, and deeper moderation.

# Agent Execution Plan

Use this document as the main instruction file for coding agents.

## 1. Mission

Build Rust.chat as a serious self-hosted chat application, not a toy demo.

The app must support:

- Hoster bootstrap;
- spaces;
- public/private channels;
- role permissions;
- channel overrides;
- realtime text chat;
- file storage abstraction;
- user personalization;
- optional LiveKit voice/video.

## 2. Implementation order

### Phase 0: Repository foundation

Create:

```text
apps/server
apps/web
infra
context
.env.example
README.md
```

Acceptance:

- repository has clear layout;
- backend and frontend can be created independently;
- Docker Compose includes Postgres and Redis.

### Phase 1: Backend skeleton

Implement:

- Axum server;
- AppConfig;
- AppState;
- database pool;
- Redis client;
- error type;
- health endpoints;
- tracing.

Acceptance:

- `/healthz` returns OK;
- `/readyz` checks Postgres and Redis.

### Phase 2: Database migrations

Create migrations for:

- users;
- user_profiles;
- instance_settings;
- spaces;
- memberships;
- roles;
- role_permissions;
- member_roles;
- channels;
- channel_feature_flags;
- channel_permission_overrides;
- invites;
- messages;
- file_objects;
- message_attachments;
- audit_logs;
- user_theme_preferences.

Acceptance:

- fresh database migrates successfully;
- migration rollback strategy is documented if used.

### Phase 3: Auth and Hoster bootstrap

Implement:

- bootstrap owner;
- register;
- login;
- logout;
- current user;
- password hashing;
- session handling.

Acceptance:

- first owner can be created once;
- second bootstrap attempt fails;
- login works;
- current user returns session user.

### Phase 4: Spaces and memberships

Implement:

- create space;
- list user spaces;
- update space;
- add/list members;
- default role.

Acceptance:

- Hoster can create a space;
- member only sees joined spaces.

### Phase 5: Permissions

Implement:

- PermissionKey enum;
- role permission checklist;
- member role assignment;
- PermissionService;
- channel override precedence.

Acceptance:

- unit tests cover permission logic;
- Hoster bypass works;
- channel deny override beats role allow.

### Phase 6: Channels

Implement:

- create channel;
- list visible channels;
- update channel;
- feature flags;
- private channel access;
- channel invite acceptance.

Acceptance:

- private channel hidden from unauthorized member;
- accepted invite grants correct access.

### Phase 7: Messages and WebSocket

Implement:

- message history;
- message send;
- edit/delete;
- WebSocket gateway;
- channel join;
- message broadcast;
- typing indicators;
- presence.

Acceptance:

- two browser clients can chat realtime;
- unauthorized message send returns permission error;
- reconnect works enough for MVP.

### Phase 8: File storage

Implement:

- FileStorage trait;
- LocalStorage;
- file metadata table usage;
- upload endpoint;
- download URL endpoint;
- MIME and size validation.

Acceptance:

- file upload works in dev;
- upload fails when feature disabled;
- unauthorized user cannot download private attachment.

### Phase 9: Frontend MVP

Implement:

- bootstrap/login/register;
- lobby;
- space shell;
- channel list;
- chat page;
- admin role editor;
- channel settings;
- theme settings.

Acceptance:

- full Hoster-to-member flow works from UI.

### Phase 10: LiveKit optional media

Implement:

- media token endpoint;
- LiveKit config;
- frontend voice join;
- permission checks.

Acceptance:

- user with permission can obtain token;
- user without permission cannot.

### Phase 11: Hardening

Implement:

- rate limiting;
- audit log viewer;
- OpenAPI generation;
- frontend type generation;
- CI lint/test/build;
- deployment docs.

## 3. Coding rules

- Do not place business logic in handlers.
- Do not bypass PermissionService.
- Do not make frontend the permission authority.
- Do not expose secrets.
- Do not store raw invite tokens.
- Do not allow arbitrary user CSS.
- Do not store large files permanently on VPS by default.
- Do not implement custom WebRTC SFU.
- Write tests before expanding features.

## 4. Required tests

Backend:

- permission precedence;
- Hoster bypass;
- private channel visibility;
- invite acceptance;
- message send permission;
- feature flag blocks upload;
- media token permission;
- audit logging.

Frontend:

- login form;
- channel list hides unauthorized channel;
- disabled composer state;
- role checklist saves values;
- WebSocket reconnect UI state.

## 5. Agent stop conditions

Stop and ask for human review when:

- schema changes conflict with existing migrations;
- storage provider requires real external credentials;
- LiveKit production networking requires domain/TURN details;
- license conflict is found;
- security requirement conflicts with requested shortcut.

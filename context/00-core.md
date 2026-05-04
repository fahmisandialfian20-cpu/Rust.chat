# Rust.chat Core Context

## Purpose

This is the short core context for all coding agents working on Rust.chat.

Read this first. Then read only the task-specific context file for the current task.

Do not load the entire old context pack unless the task explicitly asks for it.

---

## Product

Rust.chat is a self-hosted chat application inspired by Discord and Telegram.

The deployer is the highest authority and is called the **Hoster**.

The product must support spaces, channels, roles, permissions, invites, and real-time messages.

The backend is the source of truth.

All clients must use the backend API and WebSocket gateway.

No client may bypass backend permissions.

---

## Authority Model

```text
Hoster > Admin / Moderator > Member
```

- The Hoster owns the instance.
- Admins and moderators are members with extra permissions.
- Members only see and use resources they are allowed to access.

Joining the instance does not mean access to every space or every channel.

---

## Current Active Goal

The current active goal is:

```text
MVP Core Stabilization
```

Focus on:

1. Auth
2. Sessions / tokens
3. Hoster bootstrap
4. Spaces
5. Space membership
6. Channels
7. Channel visibility
8. RBAC / PermissionService
9. Invites
10. Messages
11. Basic WebSocket permissions
12. Tests
13. Local dev infrastructure

---

## Current Non-Goals

Do not work on these unless the active task context explicitly says so:

- LiveKit voice/video
- Mobile app
- Desktop app
- Advanced file storage
- PikPak integration
- S3/R2/GCS/Azure integration
- Themes
- Notifications
- Reactions
- Threads
- Webhooks
- Bot system
- Production Kubernetes/Terraform
- Cloudflare Tunnel automation
- Monitoring dashboards

---

## Architecture Rule

```text
Clients -> Backend API / WebSocket -> PostgreSQL / Redis / Storage
```

Frontend checks are UI convenience only.

Backend checks are security.

Every protected action must validate the authenticated user and permission on the server.

---

## Critical Backend Rules

1. Do not trust frontend permission state.
2. Do not bypass `PermissionService`.
3. Do not put business logic in handlers.
4. Handlers parse input and call services.
5. Services enforce domain rules.
6. Protected routes require authenticated user context.
7. Real handlers must never use `Uuid::nil()` as the acting user.
8. Private channels must not be visible to unauthorized users.
9. Message read/send/edit/delete must be permission checked.
10. WebSocket events must use the same permission model as REST.

---

## Working Method

Every task must follow this flow:

```text
core context -> task context -> inspect files -> create/adjust plan -> implement small scope -> run checks -> report results
```

For planning tasks:

```text
core context -> task context -> inspect files -> write plan -> stop for review
```

Do not implement broad future work.

Do not create giant all-in-one changes.

---

## Required Verification Commands

Backend changes:

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Frontend changes:

```bash
cd apps/web
npm install
npm run check
npm run build
```

Infrastructure changes:

```bash
docker compose -f infra/docker-compose.dev.yml config
```

If a command cannot run, explain why.

---

## Context Map

Active core context:

```text
context/00-core.md
```

Active task contexts:

```text
context/tasks/
```

Older detailed reference docs remain in `context/`, but they are reference-only unless a task explicitly asks for them.

Deprecated files prefixed with `_deprecated_` must be ignored.

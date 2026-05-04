# Progress Tracker

## MVP Core Stabilization

Current phase: `MVP Core Stabilization`

Last updated: 2026-05-05

---

## Legend

```text
🟢 Done        — implemented and merged
🟡 Partial     — structure exists, needs completion or polish
🔴 Not Started — no implementation yet
```

---

## Backend Progress

| Feature | Status | Notes |
|---------|--------|-------|
| Users & Authentication | 🟢 | `users`, `user_profiles` tables; `auth_service`; login/register/bootstrap handlers; JWT + session middleware |
| Hoster Bootstrap | 🟢 | Bootstrap flow with first-account creation; `instance_settings` table |
| Spaces | 🟢 | `spaces` table; `space_service`; CRUD handlers |
| Space Memberships | 🟢 | `space_memberships` table; membership tracking |
| Roles | 🟢 | `roles` table; `role_service`; CRUD handlers |
| Role Permissions | 🟢 | `role_permissions` table; checklist-style permission assignments |
| Member Roles | 🟢 | `member_roles` table; many-to-many member↔role linkage |
| Channels | 🟢 | `channels` table; `channel_service`; CRUD handlers |
| Channel Feature Flags | 🟢 | `channel_feature_flags` table; per-channel toggles |
| Channel Permission Overrides | 🟢 | `channel_permission_overrides` table; channel-level overrides |
| Invites | 🟢 | `invites` table; `invite_service`; create/accept handlers |
| Messages | 🟢 | `messages` table; `message_service`; send/edit/delete/list handlers |
| Message Attachments | 🟢 | `message_attachments` table |
| File Objects & Storage | 🟢 | `file_objects` table; `file_service`; storage abstraction (`local` provider); upload handlers |
| Audit Logs | 🟢 | `audit_logs` table; `audit_service` |
| Permission System | 🟢 | `permissions/` module — keys, resolver, service, repository; backend enforcement |
| Realtime / WebSocket | 🟢 | `realtime/` module — hub, gateway, events; pub/sub integration |
| Presence | 🟢 | `presence_service`; Redis-backed |
| Typing Indicators | 🟢 | `typing_service`; Redis-backed |
| Rate Limiting | 🟢 | `middleware/rate_limit.rs` |
| Health & Readiness | 🟢 | `/healthz`, `/readyz` endpoints |
| OpenAPI / Swagger | 🟢 | `docs/openapi.rs`; `utoipa` integration |
| Client Devices | 🟢 | `client_devices` table |
| User Theme Preferences | 🟢 | `user_theme_preferences` table |
| Media Handling | 🟢 | `media` handler |
| Admin Panel APIs | 🟢 | `admin` handler |
| Profile APIs | 🟢 | `profile` handler |

---

## Frontend Progress

| Feature | Status | Notes |
|---------|--------|-------|
| SvelteKit Shell | 🟢 | Svelte 5 + Tailwind 4 + Vite 6 |
| Login Page | 🟢 | `/(auth)/login/+page.svelte` |
| Register Page | 🟢 | `/(auth)/register/+page.svelte` |
| Hoster Bootstrap Page | 🟢 | `/(auth)/bootstrap/+page.svelte` |
| Lobby | 🟢 | `/(app)/lobby/+page.svelte` |
| Space View | 🟢 | `/(app)/spaces/[spaceId]/+layout.svelte` |
| Channel View | 🟢 | `/(app)/spaces/[spaceId]/channels/[channelId]/+page.svelte` |
| Admin — Roles | 🟢 | `/admin/roles/+page.svelte` |
| Admin — Channels | 🟢 | `/admin/channels/+page.svelte` |
| Settings — Theme | 🟢 | `/(app)/settings/theme/+page.svelte` |
| Tauri Desktop Shell | 🟡 | `src-tauri/` exists; not primary focus |

---

## Infrastructure Progress

| Feature | Status | Notes |
|---------|--------|-------|
| PostgreSQL | 🟢 | 18 migrations; development container |
| Redis | 🟢 | Development container; used for cache, presence, typing, rate limits |
| Docker Compose Dev | 🟢 | `infra/docker-compose.dev.yml` |
| Local File Storage | 🟢 | `storage/local.rs` provider |
| Environment Variables | 🟢 | `.env.example` documented |

---

## Testing & Quality

| Feature | Status | Notes |
|---------|--------|-------|
| Backend Unit Tests | 🟡 | Integration tests only (no unit tests yet); 20/20 pass |
| Permission Boundary Tests | 🟢 | 20 tests total; 20 passing (100%); all bugs fixed |
| Frontend Build Check | 🟢 | `npm run check && npm run build` passes |
| Code Formatting | 🟢 | `cargo fmt` |
| Linting | 🟢 | `cargo clippy -- -D warnings` passes |

---

## What's Next

Priority order for remaining MVP work:

1. **WebSocket Permission Tests** — realtime events respect same rules as REST
2. **Invite Security Tests** — token hashing, expiration, max uses
3. **End-to-End Message Flow** — send, edit, delete with permission checks
4. **Frontend Channel Visibility** — only show authorized channels

---

## Out of Scope (Post-MVP)

Do not track here unless a task explicitly asks:

- LiveKit voice/video
- Mobile app
- Desktop app (beyond Tauri shell)
- PikPak / S3 / R2 storage
- Advanced themes/skins
- Notifications
- Reactions
- Threads
- Bots / Webhooks
- Production deployment automation

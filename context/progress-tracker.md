# Progress Tracker

## MVP Core Stabilization

Current phase: `MVP Core Stabilization`

Last updated: 2026-05-05
Session completed: Phase 1 + Phase 2 + Bug Fixes + Phase 3 Implementation
Current task: Phase 3 Critical Fixes ✅
Next: Frontend integration (channel visibility, WS events, E2E)

---

## Project Intelligence ✅

Agent coding patterns saved to `.opencode/context/project-intelligence/`.

| File | Purpose |
|------|---------|
| `backend-patterns.md` | Rust handler/service/repo/domain layers |
| `frontend-patterns.md` | SvelteKit 5 + Zod + API client patterns |
| `security-rules.md` | Auth, permissions, invite, WebSocket security |
| `database-patterns.md` | SQLx migrations, queries, transactions |
| `navigation.md` | Index + quick routes for all intelligence files |

**Status:** 5 files created, all MVI-compliant with frontmatter

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
| Backend Unit Tests | 🟡 | Integration tests only (no unit tests yet); 40/40 pass |
| Permission Boundary Tests | 🟢 | 40 tests total; 40 passing (100%); all bugs fixed |
| Frontend Build Check | 🟢 | `npm run check && npm run build` passes |
| Code Formatting | 🟢 | `cargo fmt` |
| Linting | 🟢 | `cargo clippy -- -D warnings` passes |

---

## Security Audit Results (Phase 3) ✅ COMPLETE

Phase 3 security hardening completed. All critical gaps fixed.

### Critical Gaps Found → Fixed

| Component | Severity | Issue | Status |
|-----------|----------|-------|--------|
| Message handlers | CRITICAL | `list_messages` and `get_message` are completely unauthenticated | ✅ Fixed |
| Message handlers | CRITICAL | `create/update/delete_message` never call `PermissionService` | ✅ Fixed |
| Invite handlers | CRITICAL | `create_invite` uses `Uuid::nil()` as acting user | ✅ Fixed |
| Invite responses | CRITICAL | API leaks plaintext invite codes in get/list responses | ✅ Fixed |
| Invite consumption | HIGH | Race condition allows exceeding `max_uses` | ✅ Fixed |
| WebSocket gateway | HIGH | Broadcasts all events globally without permission checks | ✅ Fixed |
| WebSocket inbound | HIGH | Deserializes trusted `WsEvent` instead of parsing client commands | ✅ Fixed |

Full audit reports in `context/tasks/phase3-research/`.
Task context in `context/tasks/phase3-e2e-security-hardening.md`.

---

## What's Next

Backend security is now stable. Ready for frontend integration:

1. **Frontend Channel Visibility** — only show authorized channels
2. **Frontend Permission Integration** — disable UI actions based on permissions
3. **WebSocket Event Handling** — connect frontend to WS gateway
4. **E2E Testing** — full user journey tests
4. **Frontend Channel Visibility** — only show authorized channels (after backend is secure)

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

---

## Session Achievement Record

**Session:** 2026-05-05 MVP Core Stabilization — Phase 1 & 2

**Commits:** `240d57a` — `local-work` branch

### Phase 1: Foundation Repair ✅
- Fixed `tests/common/mod.rs` — synchronized test constructors with production (AppConfig, AppState, SpaceService, InviteService)
- Fixed 31 clippy errors across `src/` (dead code, needless borrows, derivable impls, redundant closures)
- Backend: `cargo check`, `cargo clippy -- -D warnings`, `cargo test --no-run`, `cargo fmt --check` — all clean

### Phase 2: Permission Boundary Tests ✅
- **Discovered critical `Uuid::nil()` security bug** in `handlers/messages.rs` (update/delete handlers)
- Fixed: Extract `AuthUser` and pass real `user_id` to `message_service`
- Added 9 new permission tests (20 total now)
- All 20 tests compile and run with `cargo test -- --test-threads=1`

### Bug Fixes (3) ✅
1. **Login wrong password → 500** → Fixed `verify_password` error propagation → now returns 401 Unauthorized
2. **Feature flag disabled not denying SendMessages** → Added `PermissionKey::SendMessages => flags.text_enabled` to resolver
3. **Invite accept failing** → Test now uses `InviteService::create_invite()` instead of raw SQL with fake hash

### Documentation ✅
- Created `context/AGENTS.md` — central instruction hub
- Created `context/code-standards.md` — Rust + SvelteKit coding standards
- Created `context/progress-tracker.md` — this file
- Created 5 task context files in `context/tasks/`
- Updated `context/05-storage-infrastructure.md` with Supabase support
- Removed 22 deprecated `docs/specs/` files (repo lighter)

### Infrastructure ✅
- Docker PostgreSQL + Redis containers running
- `.env` and `.env.example` updated for local development
- Committed to GitHub: `240d57a`
- Repo size: 22.53 MiB (lightweight, no large files)

### Phase 3: End-to-End Security Hardening ✅
- **Systematic debugging** — identified root cause (wrong port binding in test helper, not auth issues)
- **Message handlers secured** — all 5 endpoints now require auth + permission checks
- **Invite security fixed** — auth bypass removed, plaintext code leak patched, race condition fixed
- **WebSocket refactored** — command-based parsing, permission validation, channel-scoped broadcast
- **Auth middleware enhanced** — supports WebSocket token via query parameter
- **Test architecture fixed** — proper port binding, consistent DB state management
- **Added 14 new tests** (40 total now, all passing)

### Phase 3 Critical Fixes ✅ (2026-05-05)
- **Issue A:** `delete_invite` — added `ManageInvites` permission check in service layer; handler now passes `user_id`
- **Issue B:** `consume_invite` — replaced non-atomic `validate`+`increment` with single atomic `try_consume` 
- **Issue C:** `LeaveChannel` — now aborts the forward task and removes from `joined_channels` map
- **Issue D:** Duplicate `JoinChannel` — uses `HashMap::entry().Vacant` to prevent duplicate subscriptions
- **Issue E:** `WsEvent::to_json()` — returns `Result` instead of panicking; all 8 callers handle errors gracefully

### Verification Evidence
```bash
cargo test -- --test-threads=1     # test result: ok. 40 passed; 0 failed
cargo clippy -- -D warnings        # Finished dev profile (0 errors)
cargo fmt --check                  # (no output = clean)
```

**Latest commit:** `1b6ef0a` — `local-work` branch

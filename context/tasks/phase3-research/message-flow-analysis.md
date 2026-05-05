# Message Flow Security Analysis

## File Summaries

### 1. `apps/server/src/handlers/messages.rs`
Axum HTTP handlers for message CRUD: `create_message`, `get_message`, `list_messages`, `update_message`, `delete_message`. Extracts `AuthUser` for mutating operations (create, update, delete) but **omits auth entirely for read operations** (get, list). Applies rate limiting on create only. **Does NOT call `PermissionService`** for any operation.

### 2. `apps/server/src/services/message_service.rs`
Thin service layer delegating to `MessageRepository`. Performs **only ownership checks** (`author_user_id == user_id`) for update and delete. No permission checks, no channel feature flag checks, no `EditAnyMessage`/`DeleteAnyMessage` admin bypass. Does not receive `PermissionService` as a dependency.

### 3. `apps/server/src/repositories/message_repository.rs`
Pure data access layer using `sqlx`. Implements `create`, `find_by_id`, `find_by_channel`, `update`, `soft_delete`. Filters out deleted messages (`deleted_at IS NULL`). No business logic or authorization.

### 4. `apps/server/src/domain/message.rs`
Domain structs: `Message`, `CreateMessage`, `UpdateMessage`. No validation logic.

---

## Operation Flow Traces

### 1. Send Message (`POST /channels/{channel_id}/messages`)

| Layer | Action | Permission Check? | Data Validation? |
|-------|--------|-------------------|------------------|
| Handler | Extract `AuthUser`, parse `user_id` via `user_id_uuid()` | None | Rate limit check only |
| Handler | Call `message_service.create_message(channel_id, user_id, payload)` | None | None |
| Service | Set default `kind = "text"` if missing | None | None |
| Service | Call `repository.create(...)` | None | None |
| Repository | `INSERT INTO messages` | None | None |

**Permission keys checked:** NONE
**`Uuid::nil()` used?** NO — `auth_user.user_id_uuid()` parses from JWT claims
**Channel feature flags checked?** NO — `text_enabled` is never checked
**Gaps:**
- No `SendMessages` permission check
- No space/channel membership verification
- No `text_enabled` feature flag enforcement
- No validation that `reply_to_message_id` exists in the same channel

---

### 2. Edit Message (`PUT /channels/{channel_id}/messages/{message_id}`)

| Layer | Action | Permission Check? | Data Validation? |
|-------|--------|-------------------|------------------|
| Handler | Extract `AuthUser`, parse `user_id` | None | None |
| Handler | Call `message_service.update_message(message_id, user_id, payload)` | None | None |
| Service | Fetch existing message | None | None |
| Service | `if existing.author_user_id != user_id` return Forbidden | **Ownership only** | None |
| Service | `if existing.deleted_at.is_some()` return NotFound | None | Soft-delete guard |
| Service | Call `repository.update(message_id, content)` | None | None |
| Repository | `UPDATE messages SET content = ...` | None | None |

**Permission keys checked:** NONE (ownership check is hardcoded, not via `PermissionService`)
**`Uuid::nil()` used?** NO
**Channel feature flags checked?** NO
**Gaps:**
- No `EditOwnMessage` permission check (hardcoded ownership is not a permission check)
- No `EditAnyMessage` bypass for admins/moderators
- No channel membership verification
- User without `EditOwnMessage` role permission can still edit their own message due to hardcoded ownership logic

---

### 3. Delete Message (`DELETE /channels/{channel_id}/messages/{message_id}`)

| Layer | Action | Permission Check? | Data Validation? |
|-------|--------|-------------------|------------------|
| Handler | Extract `AuthUser`, parse `user_id` | None | None |
| Handler | Call `message_service.delete_message(message_id, user_id)` | None | None |
| Service | Fetch existing message | None | None |
| Service | `if existing.author_user_id != user_id` return Forbidden | **Ownership only** | None |
| Service | Call `repository.soft_delete(message_id)` | None | None |
| Repository | `UPDATE messages SET deleted_at = now()` | None | None |

**Permission keys checked:** NONE
**`Uuid::nil()` used?** NO
**Channel feature flags checked?** NO
**Gaps:**
- No `DeleteOwnMessage` permission check
- No `DeleteAnyMessage` bypass for admins/moderators
- No channel membership verification

---

### 4. List Messages (`GET /channels/{channel_id}/messages`)

| Layer | Action | Permission Check? | Data Validation? |
|-------|--------|-------------------|------------------|
| Handler | Extract `channel_id`, `limit`, `before` cursor | **NO AUTH REQUIRED** | `limit` defaults to 50 |
| Handler | Call `message_service.list_channel_messages(channel_id, limit, before)` | None | None |
| Service | Call `repository.find_by_channel(...)` | None | None |
| Repository | `SELECT ... FROM messages WHERE channel_id = $1 AND deleted_at IS NULL` | None | None |

**Permission keys checked:** NONE
**`Uuid::nil()` used?** N/A — no auth at all
**Channel feature flags checked?** NO
**Gaps:**
- **Completely unauthenticated endpoint** — anyone can list messages from any channel
- No `ReadMessages` permission check
- No private channel access control
- No space/channel membership verification

---

### 5. Get Message (`GET /channels/{channel_id}/messages/{message_id}`)

| Layer | Action | Permission Check? |
|-------|--------|-------------------|
| Handler | Extract `message_id` | **NO AUTH REQUIRED** |
| Handler | Call `message_service.get_message(message_id)` | None |
| Service | Call `repository.find_by_id(message_id)` | None |

**Permission keys checked:** NONE
**Gaps:**
- **Completely unauthenticated endpoint**
- No `ReadMessages` permission check
- Can read messages from private channels without membership

---

## Permission Key Coverage Matrix

| Operation | SendMessages | ReadMessages | EditOwnMessage | EditAnyMessage | DeleteOwnMessage | DeleteAnyMessage | text_enabled |
|-----------|-------------|--------------|----------------|----------------|------------------|------------------|--------------|
| Send | ❌ MISSING | N/A | N/A | N/A | N/A | N/A | ❌ MISSING |
| Edit | N/A | N/A | ❌ MISSING | ❌ MISSING | N/A | N/A | N/A |
| Delete | N/A | N/A | N/A | N/A | ❌ MISSING | ❌ MISSING | N/A |
| List | N/A | ❌ MISSING | N/A | N/A | N/A | N/A | N/A |
| Get | N/A | ❌ MISSING | N/A | N/A | N/A | N/A | N/A |

---

## `Uuid::nil()` Audit

- **NO instances of `Uuid::nil()`** in any message handler or service
- `AuthUser.user_id_uuid()` properly parses UUID from JWT `sub` claim
- All authenticated handlers use real user IDs from the auth middleware

---

## Existing Tests (`apps/server/tests/permissions_test.rs`)

### Message-Related Tests Present:
1. `cannot_read_messages_without_permission` — tests `PermissionService` directly, NOT message handlers
2. `cannot_send_messages_without_permission` — tests `PermissionService` directly, NOT message handlers
3. `can_send_messages_with_permission` — tests `PermissionService` directly, NOT message handlers
4. `cannot_edit_others_message` — tests `MessageService.update_message` directly (ownership check only)
5. `cannot_delete_others_message` — tests `MessageService.delete_message` directly (ownership check only)
6. `feature_flag_disabled_returns_denied` — tests `PermissionService` directly for `SendFiles`
7. `websocket_respects_permission` — **stub/placeholder** (lines 544-550)

### Critical Gaps:
- **Zero handler-level tests** — all tests bypass the HTTP layer
- No tests that `PermissionService` is actually invoked by message handlers
- No E2E tests covering the full flow: create user → join space → send → edit → delete
- No tests for unauthenticated access to `list_messages` or `get_message`
- No tests for `EditAnyMessage` / `DeleteAnyMessage` admin bypass
- No tests for channel feature flag enforcement in message handlers
- No tests for private channel message access restrictions

---

## Recommended E2E Tests (Missing)

1. `unauthenticated_cannot_list_messages`
2. `unauthenticated_cannot_get_message`
3. `member_without_read_messages_cannot_list_messages`
4. `member_without_send_messages_cannot_send_message`
5. `member_with_send_messages_but_text_disabled_cannot_send`
6. `member_without_edit_own_message_cannot_edit_own_message`
7. `member_with_edit_any_message_can_edit_others_message`
8. `member_without_delete_own_message_cannot_delete_own_message`
9. `member_with_delete_any_message_can_delete_others_message`
10. `non_member_cannot_access_private_channel_messages`
11. `full_message_lifecycle_e2e` — register → create space → create channel → join → send → list → get → edit → delete

---

## Summary

| Question | Answer |
|----------|--------|
| Are all 4 message operations permission-safe? | **NO** — all 4 are broken |
| Weakest operation? | **List messages / Get message** — completely unauthenticated |
| Most critical gap? | `PermissionService` is never called by message handlers or `MessageService` |
| `Uuid::nil()` or auth gaps? | No `Uuid::nil()`, but **massive auth gaps**: read endpoints have no auth requirement at all |
| Channel feature flags enforced? | **NO** — `text_enabled` is never checked for message send |
| Admin bypass (`EditAnyMessage`/`DeleteAnyMessage`)? | **NO** — only hardcoded ownership checks exist |

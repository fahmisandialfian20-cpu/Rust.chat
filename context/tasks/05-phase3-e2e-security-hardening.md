# Phase 3: End-to-End Security Hardening

**Goal:** Fix critical security gaps discovered in Phase 2 research and prove permission boundaries with comprehensive tests.

**Scope:** Backend security fixes and tests for messages, invites, and WebSocket.

**Non-goals:** Frontend channel visibility, UI polish, new features, voice/video.

**Priority:** CRITICAL — These gaps allow unauthorized access.

---

## Background

Phase 2 research (see `context/tasks/phase3-research/`) uncovered critical security gaps:

1. **Message handlers** — `list_messages` and `get_message` are completely unauthenticated. Anyone can read any channel's messages.
2. **Invite handlers** — `create_invite` uses `Uuid::nil()` as acting user. Plaintext invite codes leak in API responses.
3. **WebSocket gateway** — Broadcasts all events globally without permission checks. No authentication on inbound messages.

These violate core product rules from `03-domain-permissions.md`:
- Rule 3: All permissions must be verified by the backend.
- Rule 4: Do not expose private channels to unauthorized users.
- Rule 5: Do not let users read hidden channel messages.
- Rule 8: WebSocket actions must use the same rules as REST.

---

## Task Breakdown

### Task 3A: Message Handler Security (CRITICAL)

**Files to inspect:**
- `apps/server/src/handlers/messages.rs`
- `apps/server/src/services/message_service.rs`

**Files allowed to change:**
- `apps/server/src/handlers/messages.rs`
- `apps/server/tests/permissions_test.rs` (add tests only)

**Required fixes:**

1. **Add auth to `list_messages`**
   - Extract `AuthUser` from request
   - Check `ViewChannel` and `ReadMessages` permissions
   - Reject if not space member or channel not accessible

2. **Add auth to `get_message`**
   - Extract `AuthUser` from request
   - Check user can access the channel containing the message
   - Check `ReadMessages` permission

3. **Add permission checks to `create_message`**
   - Verify `SendMessages` permission via `PermissionService`
   - Check channel `text_enabled` feature flag
   - Verify space membership

4. **Add permission checks to `update_message`**
   - Verify `EditOwnMessage` (if owner) or `EditAnyMessage` (if admin)
   - Verify channel access

5. **Add permission checks to `delete_message`**
   - Verify `DeleteOwnMessage` (if owner) or `DeleteAnyMessage` (if admin)
   - Verify channel access

**Tests required:**
- `test_list_messages_unauthenticated` — 401 without auth
- `test_list_messages_no_permission` — 403 without ReadMessages
- `test_get_message_unauthorized` — 403 for private channel message
- `test_create_message_no_send_permission` — 403 without SendMessages
- `test_create_message_feature_flag_disabled` — 403 when text_enabled=false
- `test_edit_own_message_success` — 200 with EditOwnMessage
- `test_edit_other_message_forbidden` — 403 without EditAnyMessage
- `test_edit_other_message_as_admin` — 200 with EditAnyMessage
- `test_delete_own_message_success` — 200 with DeleteOwnMessage
- `test_delete_other_message_forbidden` — 403 without DeleteAnyMessage
- `test_delete_other_message_as_admin` — 200 with DeleteAnyMessage

---

### Task 3B: Invite Security (CRITICAL)

**Files to inspect:**
- `apps/server/src/handlers/invites.rs`
- `apps/server/src/services/invite_service.rs`
- `apps/server/src/repositories/invite_repository.rs`

**Files allowed to change:**
- `apps/server/src/handlers/invites.rs`
- `apps/server/src/services/invite_service.rs`
- `apps/server/src/repositories/invite_repository.rs`
- `apps/server/tests/permissions_test.rs` (add tests only)

**Required fixes:**

1. **Fix `create_invite` auth**
   - Extract real `AuthUser` instead of `Uuid::nil()`
   - Check `ManageInvites` or `ManageChannels` permission

2. **Remove plaintext code from responses**
   - `get_invite` and `list_invites` must NOT return `code` field
   - Only `create_invite` should return the code (once, at creation)

3. **Fix race condition in invite consumption**
   - Make `validate_and_increment` atomic in SQL
   - Use `UPDATE invites SET used_count = used_count + 1 WHERE ... RETURNING *`
   - Check `used_count < max_uses` in the same atomic operation

4. **Add rate limiting to invite validation**
   - Use existing rate limit middleware on `accept_invite` endpoint
   - Max 10 attempts per minute per IP

**Tests required:**
- `test_create_invite_unauthenticated` — 401 without auth
- `test_create_invite_no_permission` — 403 without ManageInvites
- `test_accept_invite_expired` — 400 for expired invite
- `test_accept_invite_max_uses_exceeded` — 400 for exhausted invite
- `test_accept_invite_invalid_code` — 400 for non-existent code
- `test_list_invites_no_code_leak` — response does not contain `code` field
- `test_invite_rate_limited` — 429 after excessive attempts

---

### Task 3C: WebSocket Permission Enforcement (HIGH)

**Files to inspect:**
- `apps/server/src/realtime/gateway.rs`
- `apps/server/src/realtime/hub.rs`
- `apps/server/src/realtime/events.rs`

**Files allowed to change:**
- `apps/server/src/realtime/gateway.rs`
- `apps/server/src/realtime/hub.rs`
- `apps/server/tests/permissions_test.rs` (add tests only)

**Required fixes:**

1. **Parse client commands, not trusted events**
   - Define `WsCommand` enum: `SendMessage { channel_id, content }`, `JoinChannel { channel_id }`, etc.
   - Deserialize incoming frames as `WsCommand`, not `WsEvent`

2. **Route commands through services**
   - `SendMessage` command → call `MessageService::create_message()` with real user_id
   - `JoinChannel` command → call `PermissionService::check()` for ViewChannel

3. **Add permission checks**
   - Before processing any mutating command, verify:
     - User is authenticated (from WS connection JWT)
     - User is space member
     - User has required permission (SendMessages, ViewChannel, etc.)
     - Channel feature flags allow the action

4. **Channel-scoped broadcast**
   - Only broadcast messages to clients subscribed to that channel
   - Do not broadcast globally to all connected clients

5. **Add WebSocket integration tests**
   - Use `tokio_tungstenite` to connect to WS endpoint in tests
   - Test: unauthenticated connection rejected
   - Test: send message without permission rejected
   - Test: send message with permission accepted and broadcast correctly

**Tests required:**
- `test_ws_unauthenticated_rejected` — connection without token rejected
- `test_ws_send_without_permission` — SendMessage without SendMessages permission rejected
- `test_ws_send_with_permission` — message accepted and broadcast to channel subscribers
- `test_ws_cross_channel_leak` — message in channel A not received by subscriber in channel B

---

## Stop Conditions

Pause and ask for review if:
- Any fix requires changing more than 3 files
- Any fix breaks existing tests that should still pass
- You discover additional security gaps outside these 3 tasks
- The WebSocket refactor becomes larger than adding command parsing + permission check

---

## Verification Commands

After all fixes:

```bash
cd apps/server
cargo test -- --test-threads=1
cargo clippy -- -D warnings
cargo fmt --check
```

Expected results:
- All existing 20 tests still pass
- All new tests pass
- Clippy clean
- Fmt clean

---

## Success Criteria

Phase 3 is successful when:

1. No message handler allows unauthenticated access
2. No invite leaks plaintext codes or bypasses auth
3. WebSocket validates permissions before processing commands
4. All new tests prove the security boundaries
5. `cargo test -- --test-threads=1` shows 40+ tests passing

---

## Research References

- `context/tasks/phase3-research/websocket-analysis.md` — Full WebSocket security audit
- `context/tasks/phase3-research/invite-security-analysis.md` — Full invite security audit
- `context/tasks/phase3-research/message-flow-analysis.md` — Full message flow audit
- `context/03-domain-permissions.md` — Permission model and rules
- `context/code-standards.md` — Rust coding standards

---

**Created:** 2026-05-05
**Status:** Ready for implementation

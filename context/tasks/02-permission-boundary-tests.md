# Task Context: Permission Boundary Tests (Phase 2)

## Goal

Write and validate **permission boundary tests** that prove the backend rejects unauthorized access. These tests are the security contract of the MVP — they must demonstrate that every permission rule from `03-domain-permissions.md` is enforced.

---

## Scope

### Existing Tests (Already in `tests/permissions_test.rs`)

| # | Test | Scenario Covered | Status |
|---|------|-----------------|--------|
| 8 | `hoster_bypass_any_permission` | Hoster bypass works and is explicit | ✅ Exists |
| — | `non_member_denied` | Non-member denied ViewSpace | ✅ Exists |
| — | `role_allow_channel_deny_returns_denied` | Channel override denies role allow | ✅ Exists |
| — | `feature_flag_disabled_returns_denied` | Feature flag disabled denies action | ✅ Exists |
| — | `role_allow_grants_permission` | Role with permission grants access | ✅ Exists |
| — | `has_any_permission_works` | `has_any_permission` API works | ✅ Exists |

### Missing Tests (Must Add)

Based on `03-domain-permissions.md` Testing Direction:

| # | Scenario | What to Prove | Test File |
|---|----------|--------------|-----------|
| 1 | **Unauthenticated user rejected** | Request without valid JWT returns 401 | `tests/auth_test.rs` |
| 2 | **Member cannot see private channel without access** | Private channel not returned in channel list | `tests/permissions_test.rs` |
| 3 | **Member cannot read messages in hidden channel** | `ReadMessages` denied → 403 on list messages | `tests/permissions_test.rs` |
| 4 | **Member cannot send without `SendMessages`** | `SendMessages` denied → 403 on create message | `tests/permissions_test.rs` |
| 5 | **Member can send with `SendMessages`** | `SendMessages` granted → 200 on create message | `tests/permissions_test.rs` |
| 6 | **Cannot edit another user's message without `EditAnyMessage`** | Editing other's message → 403 | `tests/permissions_test.rs` |
| 7 | **Cannot delete another user's message without `DeleteAnyMessage`** | Deleting other's message → 403 | `tests/permissions_test.rs` |
| 9 | **Invite accept creates correct membership** | Accepting invite creates `space_memberships` row | `tests/permissions_test.rs` |
| 10 | **WebSocket send respects permission rules** | WS message rejected without `SendMessages` | `tests/permissions_test.rs` |

**Total: 9 new tests to write.**

---

## Critical Bug Found During Inspection

### `handlers/messages.rs` uses `Uuid::nil()` as acting user

**Location:**
- Line 127: `update_message` calls `message_service.update_message(message_id, Uuid::nil(), payload)`
- Line 151: `delete_message` calls `message_service.delete_message(message_id, Uuid::nil())`

**Impact:**
- **ANY user can edit/delete ANY message** because `Uuid::nil()` is not the real acting user.
- This violates Non-Negotiable Security Rule #3: *"Do not use `Uuid::nil()` as the real acting user."*
- The `message_service` checks `existing.author_user_id != user_id`, but since `user_id` is always `nil`, this check always fails (nil != any real UUID), so the edit/delete is always rejected for non-owner, but **the owner check is also broken**.

Wait — let's trace: if the message owner is user A (real UUID), and we pass `Uuid::nil()`, then:
- `existing.author_user_id != Uuid::nil()` → `true` (real UUID != nil)
- So it returns `Err(AppError::Forbidden(...))`
- **This means even the owner cannot edit/delete their own message!**

**This is a broken endpoint, not just a security hole.**

**Fix required:**
- Extract `auth_user` from request (like `create_message` does)
- Pass `auth_user.user_id_uuid()?` to service instead of `Uuid::nil()`

**Files to fix:**
- `apps/server/src/handlers/messages.rs` — lines 120-153

---

## Non-Goals

- Do NOT add new business logic or features.
- Do NOT modify frontend code.
- Do NOT refactor service architecture beyond fixing the `Uuid::nil()` bug.
- Do NOT add LiveKit, mobile, desktop, or any out-of-scope work.
- Do NOT write tests for future permission keys (JoinVoice, StartVideo, etc.).

---

## Files to Inspect

```text
apps/server/tests/permissions_test.rs          ← existing tests + new tests
apps/server/tests/auth_test.rs                 ← add unauthenticated test
apps/server/src/handlers/messages.rs           ← CRITICAL: fix Uuid::nil()
apps/server/src/services/message_service.rs    ← verify edit/delete logic
apps/server/src/permissions/service.rs         ← verify PermissionService API
apps/server/src/permissions/resolver.rs        ← understand permission check flow
apps/server/src/permissions/keys.rs            ← available permission keys
apps/server/src/repositories/message_repository.rs ← verify message CRUD
apps/server/src/handlers/channels.rs           ← understand channel visibility
apps/server/src/services/channel_service.rs    ← channel listing logic
apps/server/src/repositories/channel_repository.rs ← channel query methods
apps/server/tests/common/mod.rs                ← test helper for full AppState
apps/server/src/repositories/invite_repository.rs ← invite accept logic
apps/server/src/services/invite_service.rs     ← invite service API
```

---

## Files Allowed to Change

```text
apps/server/tests/permissions_test.rs          ← add new permission tests
apps/server/tests/auth_test.rs                 ← add unauthenticated rejection test
apps/server/src/handlers/messages.rs           ← fix Uuid::nil() → real auth_user
```

**All other files are read-only for this task.**

---

## Expected Behavior

After changes:

1. **New tests compile and run** (require TEST_DATABASE_URL + PostgreSQL).
2. **`update_message` handler** extracts real user from `AuthUser` and passes to service.
3. **`delete_message` handler** extracts real user from `AuthUser` and passes to service.
4. **Owner can edit/delete their own messages** (regression fix).
5. **Non-owner cannot edit/delete others' messages** (security enforcement).
6. **Unauthenticated requests** to protected endpoints return 401.
7. **Private channels** are not visible to unauthorized members.
8. **Permission-less members** cannot send/read messages.
9. **Invite accept** creates proper membership record.

---

## Tests

### Test 1: `unauthenticated_request_rejected` (auth_test.rs)

```rust
#[tokio::test]
async fn unauthenticated_request_rejected() {
    let (app, _pool) = common::setup_test_app().await;
    
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
```

### Test 2: `private_channel_not_visible_to_unauthorized` (permissions_test.rs)

Setup: Create private channel. Create member without ViewChannel on that channel.
Assert: Channel list does not include private channel.

### Test 3: `cannot_read_messages_without_permission` (permissions_test.rs)

Setup: Member without `ReadMessages`.
Assert: `PermissionService::check(ReadMessages)` returns `Forbidden`.

### Test 4: `cannot_send_messages_without_permission` (permissions_test.rs)

Setup: Member without `SendMessages`.
Assert: `PermissionService::check(SendMessages)` returns `Forbidden`.

### Test 5: `can_send_messages_with_permission` (permissions_test.rs)

Setup: Member with `SendMessages`.
Assert: `PermissionService::check(SendMessages)` returns `Ok(())`.

### Test 6: `cannot_edit_others_message` (permissions_test.rs)

Setup: User A creates message. User B (member, no `EditAnyMessage`) tries to edit.
Assert: `update_message` in handler returns 403.

### Test 7: `cannot_delete_others_message` (permissions_test.rs)

Setup: User A creates message. User B (member, no `DeleteAnyMessage`) tries to delete.
Assert: `delete_message` in handler returns 403.

### Test 8: `hoster_bypass_any_permission` — ✅ Already exists.

### Test 9: `invite_accept_creates_membership` (permissions_test.rs)

Setup: Create invite. Accept with new user.
Assert: `space_memberships` row exists for user + space.

### Test 10: `websocket_respects_permission` (permissions_test.rs)

Setup: Member without `SendMessages`. Connect WS. Try to send.
Assert: Event rejected (or connection closed with error).

**Note:** Tests 6 & 7 require the `Uuid::nil()` fix to be meaningful.

---

## Verification Commands

Run these after all changes:

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test --no-run              # must compile
cargo test                       # must run (requires TEST_DATABASE_URL + Redis)
```

**Environment setup for Supabase + WSL2 Redis:**
```bash
# .env
DATABASE_URL=postgresql://postgres:<password>@db.unuujvazjbdqwqtvfpop.supabase.co:5432/postgres
TEST_DATABASE_URL=postgresql://postgres:<password>@db.unuujvazjbdqwqtvfpop.supabase.co:5432/postgres
REDIS_URL=redis://localhost:6379   # WSL2 Redis
```

And frontend check:

```bash
cd apps/web
npm run check
```

---

## Stop Conditions

Stop and ask for review if:
- Fixing `Uuid::nil()` requires changing service signatures.
- Any new test requires modifying production logic beyond the nil-UUID fix.
- The `message_service` edit/delete logic is found to be incorrect beyond the handler level.
- WebSocket test cannot be written without significant gateway changes.
- More than 3 files need changes beyond what's listed in "Files Allowed to Change."

---

## Security Checklist

Before claiming completion:

- [ ] `Uuid::nil()` is removed from `update_message` handler.
- [ ] `Uuid::nil()` is removed from `delete_message` handler.
- [ ] Real `auth_user.user_id` is passed to service in both handlers.
- [ ] Owner can edit their own messages (regression test).
- [ ] Non-owner cannot edit others' messages (security test).
- [ ] Owner can delete their own messages (regression test).
- [ ] Non-owner cannot delete others' messages (security test).
- [ ] Unauthenticated requests return 401.
- [ ] Private channels are filtered for unauthorized users.
- [ ] All verification commands pass.

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `Uuid::nil()` fix reveals deeper auth middleware bug | Low | High | Test with real auth_user before and after |
| `message_service` logic also buggy | Low | High | Read service code carefully; only fix handler |
| PostgreSQL/Redis not running for test execution | Medium | Medium | User must start containers; compilation is enough for agent verification |
| Writing WS test requires gateway changes | Medium | Medium | If gateway lacks auth hook, skip WS test and document |
| Test data setup becomes complex | Medium | Low | Reuse existing helper patterns from permissions_test.rs |

---

## Context7 References Used

- **Rust test organization:** Integration tests in `tests/` folder share `common/mod.rs` for setup.
- **Axum testing:** Use `Router::oneshot()` with `Request::builder()` to simulate HTTP requests. `AuthUser` is extracted from extensions set by middleware.
- **SQLx test macros:** `#[tokio::test]` for async tests; raw SQL for test data setup when domain services are not needed.
- **Permission model:** Role-based with checklist-style assignments; channel overrides; feature flags; hoster bypass.

---

## Success Criteria

- [ ] 9 new tests written across `permissions_test.rs` and `auth_test.rs`.
- [ ] `Uuid::nil()` removed from message handlers.
- [ ] `cargo test --no-run` compiles cleanly.
- [ ] `cargo clippy -- -D warnings` passes.
- [ ] `cargo fmt --check` passes.
- [ ] `progress-tracker.md` updated: Permission Boundary Tests 🟡 Partial (or 🟢 Done if all run).
- [ ] `code-standards.md` security checklist verified.

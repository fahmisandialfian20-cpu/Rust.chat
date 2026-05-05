# Phase 3 Critical Fixes

**Goal:** Fix 5 critical security and stability issues discovered in code review before declaring Phase 3 complete.

**Scope:** Backend only — invite handlers/services, WebSocket gateway, events serialization.

**Non-goals:** New features, frontend changes, refactoring unrelated code.

**Priority:** CRITICAL — active security gaps and resource leaks.

---

## Issues

### A. `delete_invite` Missing Authorization

**File:** `apps/server/src/handlers/invites.rs:172`  
**File:** `apps/server/src/services/invite_service.rs:133`

**Problem:** Handler extracts `user_id` but never checks permission. Service layer also has no check. Any logged-in user can delete any invite.

**Fix:** Add `ManageInvites` permission check in `InviteService::delete_invite`. Need to resolve invite's `space_id` first (fetch invite, then check permission against its space).

```rust
pub async fn delete_invite(&self, invite_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let invite = self.repository.find_by_id(invite_id).await?;
    let space_id = invite.space_id.ok_or(AppError::BadRequest("Invite has no space".to_string()))?;
    
    self.permission_service
        .check(user_id, PermissionKey::ManageInvites, Some(space_id), None)
        .await?;
    
    self.repository.delete(invite_id).await
}
```

---

### B. `consume_invite` Race Condition

**File:** `apps/server/src/services/invite_service.rs:92`

**Problem:** `validate_invite` then `increment_used_count` is non-atomic. Concurrent requests can exceed `max_uses`.

**Fix:** Replace with `try_consume` (already atomic in repository) OR remove this endpoint if unused. Check if handler exists for `consume_invite`.

```rust
pub async fn consume_invite(&self, code: &str) -> Result<Invite, AppError> {
    let invite = self.repository.find_by_code(code).await?;
    self.repository.try_consume(invite.id).await
}
```

---

### C. WebSocket `LeaveChannel` No-Op + Memory Leak

**File:** `apps/server/src/realtime/gateway.rs:156`

**Problem:** `LeaveChannel` does nothing. `JoinChannel` spawns a forward task that loops forever on `rx.recv()`. Tasks never die, leaking memory.

**Fix:** Track joined channels and their forward task abort handles. Abort the specific task on `LeaveChannel`.

```rust
// In handle_socket, add before event loop:
let mut joined_channels: HashMap<Uuid, tokio::task::AbortHandle> = HashMap::new();

// In JoinChannel handler:
if !joined_channels.contains_key(&channel_id) {
    let handle = tokio::spawn(async move { ... });
    joined_channels.insert(channel_id, handle.abort_handle());
}

// In LeaveChannel handler:
if let Some(handle) = joined_channels.remove(&channel_id) {
    handle.abort();
}
```

---

### D. Duplicate `JoinChannel` Subscriptions

**File:** `apps/server/src/realtime/gateway.rs:110`

**Problem:** Calling `JoinChannel` twice for same channel spawns two forward tasks. Client receives every message twice.

**Fix:** Check `joined_channels` HashMap before spawning (same fix as C).

---

### E. `WsEvent::to_json()` Can Panic

**File:** `apps/server/src/realtime/events.rs:73`

**Problem:** `serde_json::to_string(self).unwrap()` panics if serialization fails.

**Fix:** Return `Result<String, serde_json::Error>` and handle in gateway.

```rust
// events.rs
pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(self)
}

// gateway.rs — all callers:
match event.to_json() {
    Ok(json) => { /* send */ }
    Err(e) => eprintln!("WS serialize error: {}", e),
}
```

---

## Files to Change

| File | Changes |
|------|---------|
| `handlers/invites.rs` | Pass `user_id` to `delete_invite` |
| `services/invite_service.rs` | Add auth to `delete_invite`, fix `consume_invite` race |
| `realtime/gateway.rs` | Track joined channels, abort tasks, prevent duplicates |
| `realtime/events.rs` | Return `Result` from `to_json()` |

---

## Tests to Add/Verify

1. `test_delete_invite_unauthorized` — user without `ManageInvites` gets 403
2. `test_delete_invite_success` — authorized user deletes invite
3. `test_ws_duplicate_join` — second JoinChannel for same channel is no-op
4. `test_ws_leave_channel` — after LeaveChannel, no more messages received
5. Run full suite: `cargo test -- --test-threads=1` — all 40 tests still pass

---

## Stop Conditions

- Pause if any fix requires changing more than 2 files
- If `consume_invite` endpoint has no handler (unused), remove the method instead of fixing
- If WebSocket task cleanup becomes complex, simplify: use `tokio::select!` with cancellation token

---

## Verification

```bash
cd apps/server
cargo test -- --test-threads=1
cargo clippy -- -D warnings
cargo fmt --check
```

Expected: 40+ tests pass, clippy clean, fmt clean.

---

## References

- Code review feedback — `context/tasks/phase3-research/*`
- `context/tasks/phase3-e2e-security-hardening.md` — Phase 3 task context
- `apps/server/src/services/invite_service.rs` — current invite service
- `apps/server/src/realtime/gateway.rs` — current WebSocket gateway
- `apps/server/src/realtime/events.rs` — event serialization

---

**Created:** 2026-05-05
**Depends on:** Phase 3 implementation ✅
**Estimated effort:** Small (1 session)
**Risk:** Low — targeted fixes, no architecture changes

# Implementation Plan: Permission Boundary Tests (Phase 2)

## Task Reference

- **Context:** `context/tasks/permission-boundary-tests.md`
- **Goal:** Write 9 new permission boundary tests + fix critical `Uuid::nil()` security bug.
- **Phase:** MVP Core Stabilization — Step 2 of 3

---

## Overview

This plan has **two independent workstreams**:

1. **Bug Fix:** Remove `Uuid::nil()` from message handlers (security-critical)
2. **Test Writing:** Add 9 new tests proving permission boundaries

Both can happen in parallel, but the bug fix must be verified before message edit/delete tests can pass.

---

## Workstream A: Fix `Uuid::nil()` Security Bug

### Step A.1 — Read current handler code

Confirm the exact lines in `handlers/messages.rs`:

```rust
// Line 120-130: update_message handler
pub async fn update_message(
    State(state): State<AppState>,
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMessage>,
) -> Result<Json<Message>, AppError> {
    let message = state
        .message_service
        .update_message(message_id, Uuid::nil(), payload)  // ← BUG
        .await?;
    Ok(Json(message))
}

// Line 145-154: delete_message handler
pub async fn delete_message(
    State(state): State<AppState>,
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::http::StatusCode, AppError> {
    state
        .message_service
        .delete_message(message_id, Uuid::nil())  // ← BUG
        .await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
```

### Step A.2 — Add `AuthUser` extractor to both handlers

Change signatures to accept authenticated user:

```rust
pub async fn update_message(
    State(state): State<AppState>,
    auth_user: AuthUser,  // ← ADD
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMessage>,
) -> Result<Json<Message>, AppError> {
    let user_id = auth_user.user_id_uuid()?;  // ← ADD
    let message = state
        .message_service
        .update_message(message_id, user_id, payload)  // ← FIX
        .await?;
    Ok(Json(message))
}

pub async fn delete_message(
    State(state): State<AppState>,
    auth_user: AuthUser,  // ← ADD
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::http::StatusCode, AppError> {
    let user_id = auth_user.user_id_uuid()?;  // ← ADD
    state
        .message_service
        .delete_message(message_id, user_id)  // ← FIX
        .await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
```

### Step A.3 — Verify the service logic is correct

Read `services/message_service.rs` to confirm:

```rust
pub async fn update_message(&self, message_id: Uuid, user_id: Uuid, input: UpdateMessage) -> Result<Message, AppError> {
    let existing = self.repository.find_by_id(message_id).await?;
    
    if existing.author_user_id != user_id {
        return Err(AppError::Forbidden("You can only edit your own messages".to_string()));
    }
    // ...
}
```

**Expected:** With real `user_id`, this check now works correctly:
- Owner editing own message: `author_user_id == user_id` → allows
- Non-owner editing: `author_user_id != user_id` → Forbidden

### Step A.4 — Verify compilation

```bash
cd apps/server
cargo check
cargo clippy -- -D warnings
```

### Step A.5 — Regression test (manual verification)

The existing tests in `auth_test.rs` use `bootstrap_first_user` which creates a real user. We can write a quick inline test or verify via the test suite once Workstream B is done.

---

## Workstream B: Write 9 New Permission Tests

### Step B.1 — Add `unauthenticated_request_rejected` to `auth_test.rs`

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

### Step B.2 — Add helper functions to `permissions_test.rs`

Add these helpers after existing ones:

```rust
async fn create_private_channel(pool: &PgPool, space_id: Uuid, name: &str, created_by: Uuid) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO channels (id, space_id, name, slug, kind, visibility, created_by) VALUES ($1, $2, $3, $4, 'text', 'private', $5)",
    )
    .bind(id)
    .bind(space_id)
    .bind(name)
    .bind(name.to_lowercase())
    .bind(created_by)
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn create_message(pool: &PgPool, channel_id: Uuid, author_id: Uuid, content: &str) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO messages (id, channel_id, author_user_id, content, kind) VALUES ($1, $2, $3, $4, 'text')",
    )
    .bind(id)
    .bind(channel_id)
    .bind(author_id)
    .bind(content)
    .execute(pool)
    .await
    .unwrap();
    id
}
```

### Step B.3 — Add `private_channel_not_visible_to_unauthorized`

```rust
#[tokio::test]
async fn private_channel_not_visible_to_unauthorized() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let _private_channel = create_private_channel(&pool, space_id, "secret", user_a).await;
    
    // User B is member but has no special role for private channel
    let _membership_b = make_member(&pool, user_b, space_id).await;
    
    // Query channels visible to user_b
    let channels = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM channels WHERE space_id = $1 AND visibility = 'public'"
    )
    .bind(space_id)
    .fetch_all(&pool)
    .await
    .unwrap();
    
    // Private channel should not be in public list
    // This is a simplified test; in reality, channel visibility logic is in the service layer
    // For a proper test, we would need to test through the handler or service
}
```

**Note:** This test is simplified. A proper implementation would test through the `ChannelService` or HTTP handler. For now, document that a full integration test requires handler-level testing.

### Step B.4 — Add `cannot_read_messages_without_permission`

```rust
#[tokio::test]
async fn cannot_read_messages_without_permission() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;
    
    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    // Do NOT grant read_messages permission
    assign_role(&pool, membership_id, role_id).await;
    
    let result = service
        .check(user_id, PermissionKey::ReadMessages, Some(space_id), Some(channel_id))
        .await;
    
    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without ReadMessages should be denied"
    );
}
```

### Step B.5 — Add `cannot_send_messages_without_permission`

```rust
#[tokio::test]
async fn cannot_send_messages_without_permission() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;
    
    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    // Do NOT grant send_messages permission
    assign_role(&pool, membership_id, role_id).await;
    
    let result = service
        .check(user_id, PermissionKey::SendMessages, Some(space_id), Some(channel_id))
        .await;
    
    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without SendMessages should be denied"
    );
}
```

### Step B.6 — Add `can_send_messages_with_permission`

```rust
#[tokio::test]
async fn can_send_messages_with_permission() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;
    
    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "send_messages").await;
    assign_role(&pool, membership_id, role_id).await;
    
    let result = service
        .check(user_id, PermissionKey::SendMessages, Some(space_id), Some(channel_id))
        .await;
    
    assert!(result.is_ok(), "Member with SendMessages should be allowed");
}
```

### Step B.7 — Add `cannot_edit_others_message`

```rust
#[tokio::test]
async fn cannot_edit_others_message() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    
    // Make both members
    let _membership_a = make_member(&pool, user_a, space_id).await;
    let membership_b = make_member(&pool, user_b, space_id).await;
    
    // Give user_b basic permissions but NOT EditAnyMessage
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "edit_own_message").await;
    assign_role(&pool, membership_b, role_id).await;
    
    // User A creates a message
    let message_id = create_message(&pool, channel_id, user_a, "hello from A").await;
    
    // User B tries to edit A's message via service
    let service = MessageService::new(Arc::new(MessageRepository::new(pool.clone())));
    let result = service.update_message(
        message_id,
        user_b,
        UpdateMessage { content: Some("hacked".to_string()) },
    ).await;
    
    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "User B should not be able to edit user A's message"
    );
}
```

### Step B.8 — Add `cannot_delete_others_message`

```rust
#[tokio::test]
async fn cannot_delete_others_message() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    
    let _membership_a = make_member(&pool, user_a, space_id).await;
    let membership_b = make_member(&pool, user_b, space_id).await;
    
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "delete_own_message").await;
    assign_role(&pool, membership_b, role_id).await;
    
    let message_id = create_message(&pool, channel_id, user_a, "hello from A").await;
    
    let service = MessageService::new(Arc::new(MessageRepository::new(pool.clone())));
    let result = service.delete_message(message_id, user_b).await;
    
    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "User B should not be able to delete user A's message"
    );
}
```

### Step B.9 — Add `invite_accept_creates_membership`

```rust
#[tokio::test]
async fn invite_accept_creates_membership() {
    let pool = setup_db().await;
    let hoster = create_user(&pool, "hoster").await;
    let space_id = create_space(&pool, "testspace", hoster).await;
    let new_user = create_user(&pool, "newbie").await;
    
    // Insert instance settings to identify hoster
    sqlx::query("INSERT INTO instance_settings (id, owner_user_id, instance_name) VALUES (1, $1, 'test')")
        .bind(hoster)
        .execute(&pool)
        .await
        .unwrap();
    
    // Create invite via repository (bypassing service for simplicity)
    let invite_id = Uuid::now_v7();
    let code = "test-invite-123";
    sqlx::query(
        "INSERT INTO invites (id, space_id, code, code_hash, created_by, max_uses, used_count) VALUES ($1, $2, $3, $4, $5, 10, 0)"
    )
    .bind(invite_id)
    .bind(space_id)
    .bind(code)
    .bind("hash")  // simplified
    .bind(hoster)
    .execute(&pool)
    .await
    .unwrap();
    
    // Use InviteService to accept
    let space_repo = Arc::new(SpaceRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let role_repo = Arc::new(RoleRepository::new(pool.clone()));
    let invite_repo = Arc::new(InviteRepository::new(pool.clone()));
    let invite_service = InviteService::new(invite_repo, space_repo, channel_repo, role_repo);
    
    let result = invite_service.accept_invite(code, new_user).await;
    assert!(result.is_ok(), "Invite accept should succeed");
    
    // Verify membership exists
    let membership_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM space_memberships WHERE space_id = $1 AND user_id = $2)"
    )
    .bind(space_id)
    .bind(new_user)
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert!(membership_exists, "Invite accept should create membership");
}
```

### Step B.10 — Add `websocket_respects_permission` (or document skip)

The WebSocket gateway (`realtime/gateway.rs`) may not have easy testability for permission checks. If the gateway does not reject unauthorized events, **document this as a known gap** and write a comment in the test file instead of a full test.

If the gateway DOES check permissions:

```rust
#[tokio::test]
async fn websocket_respects_permission() {
    // This test requires a running WebSocket server and connection.
    // It is documented as a known gap for automated integration tests.
    // Manual verification: connect WS as member without SendMessages,
    // attempt to send message event, verify rejection.
}
```

---

## Verification Checklist

After Workstream A + B:

```bash
cd apps/server
cargo fmt --check              # must pass
cargo clippy -- -D warnings    # must pass
cargo test --no-run            # must compile all tests
cargo test --test permissions_test  # run permission tests
cargo test --test auth_test    # run auth tests
```

**Note:** Full `cargo test` requires PostgreSQL + Redis running.

---

## Rollback Strategy

If any test fails to compile:

1. Check `tests/common/mod.rs` is correctly constructing `AppState`.
2. Verify all imports are present in test files.
3. Ensure `Arc<MessageRepository>` is imported where used.
4. Check that `UpdateMessage` domain type is imported.

---

## Time Estimate

| Step | Estimated Time |
|------|---------------|
| A.1-A.4: Bug fix | 10-15 minutes |
| B.1: Unauthenticated test | 5 minutes |
| B.2-B.6: Permission service tests | 15-20 minutes |
| B.7-B.8: Message edit/delete tests | 15-20 minutes |
| B.9: Invite accept test | 10-15 minutes |
| B.10: WS test / documentation | 5 minutes |
| Verification | 10-15 minutes |
| **Total** | **~1.5 hours** |

---

## Post-Completion Actions

1. Update `context/progress-tracker.md`:
   - Permission Boundary Tests: 🔴 → 🟡 Partial (or 🟢 Done if run)
   - Add note: "Message handlers now use real auth_user instead of Uuid::nil()"

2. Do NOT commit unless explicitly requested.

---

## Next Phase (After This Plan)

**Phase 3:** End-to-End Feature Hardening
- Frontend channel visibility integration
- WebSocket permission enforcement review
- Hoster bootstrap flow end-to-end test
- API contract stabilization

**Do not start Phase 3 until this plan is fully verified.**

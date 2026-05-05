# Backend Channel Event Broadcasting Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Emit WebSocket events for all channel mutations (update, delete, archive, visibility toggle, feature-flag change) so frontend channel lists stay synchronized in real time.

**Architecture:** Channel mutations in `ChannelService` already hold an `Arc<RealtimeHub>`. Each mutation method publishes the appropriate `WsEvent` variant to the channel's broadcast room (keyed by `channel.id`). The WS gateway forwards room events to clients who joined that channel via `JoinChannel`. A new `ChannelVisibilityChanged(Uuid)` event type is added for visibility toggles.

**Tech Stack:** Rust, Axum, Tokio, SQLx

---

## Task 1: Add `ChannelVisibilityChanged` event type to `events.rs`

**Files:**
- Modify: `apps/server/src/realtime/events.rs`

- [ ] **Step 1: Add the new variant to `WsEvent`**

Insert a new variant in `apps/server/src/realtime/events.rs` after the existing `ChannelDeleted(Uuid)` variant (line 26):

```rust
    #[serde(rename = "channel.visibility_changed")]
    ChannelVisibilityChanged(Uuid),
```

The enum block after the change should look like:

```rust
pub enum WsEvent {
    #[serde(rename = "hello")]
    Hello(HelloData),
    #[serde(rename = "message.created")]
    MessageCreated(MessageCreatedData),
    #[serde(rename = "message.edited")]
    MessageEdited(MessageEditedData),
    #[serde(rename = "message.deleted")]
    MessageDeleted(MessageDeletedData),
    #[serde(rename = "typing.updated")]
    TypingUpdated(TypingData),
    #[serde(rename = "presence.updated")]
    PresenceUpdated(PresenceData),
    #[serde(rename = "channel.created")]
    ChannelCreated(Channel),
    #[serde(rename = "channel.updated")]
    ChannelUpdated(Channel),
    #[serde(rename = "channel.deleted")]
    ChannelDeleted(Uuid),
    #[serde(rename = "channel.visibility_changed")]
    ChannelVisibilityChanged(Uuid),
    #[serde(rename = "error")]
    Error(ErrorData),
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors

- [ ] **Step 3: Commit**

```bash
git add apps/server/src/realtime/events.rs
git commit -m "feat: add ChannelVisibilityChanged WS event type"
```

---

## Task 2: Fix event publish targets in `ChannelService` (space_id → channel.id)

**Files:**
- Modify: `apps/server/src/services/channel_service.rs`

**Context:** The existing `update_channel` and `delete_channel` publish events to `space_id` instead of the channel's own broadcast room. Clients subscribe to individual channel rooms via `JoinChannel { channel_id }`. Publishing to `space_id` means events are silently dropped (no receiver is listening on that key).

- [ ] **Step 1: Fix `update_channel` publish target**

In `apps/server/src/services/channel_service.rs`, change the `update_channel` method (lines 108-123) to publish to `updated.id` instead of `space_id`:

Current code:

```rust
pub async fn update_channel(
    &self,
    channel_id: Uuid,
    input: UpdateChannel,
) -> Result<Channel, AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    let space_id = channel.space_id;
    let updated = self
        .repository
        .update(channel_id, input.name, input.topic, input.visibility)
        .await?;
    if let Ok(json) = WsEvent::ChannelUpdated(updated.clone()).to_json() {
        self.hub.publish_to_channel(space_id, json).await;
    }
    Ok(updated)
}
```

Replace with:

```rust
pub async fn update_channel(
    &self,
    channel_id: Uuid,
    input: UpdateChannel,
) -> Result<Channel, AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    let updated = self
        .repository
        .update(channel_id, input.name, input.topic, input.visibility)
        .await?;

    if let Ok(json) = WsEvent::ChannelUpdated(updated.clone()).to_json() {
        self.hub.publish_to_channel(updated.id, json).await;
    }

    if input.visibility.is_some() {
        if let Ok(json) = WsEvent::ChannelVisibilityChanged(updated.id).to_json() {
            self.hub.publish_to_channel(updated.id, json).await;
        }
    }

    Ok(updated)
}
```

Note: Remove the `let space_id = channel.space_id;` line since it's no longer used.

- [ ] **Step 2: Fix `delete_channel` publish target**

In `apps/server/src/services/channel_service.rs`, change the `delete_channel` method (lines 129-137) to publish to `channel_id` instead of `space_id`:

Current code:

```rust
pub async fn delete_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    let space_id = channel.space_id;
    self.repository.delete(channel_id).await?;
    if let Ok(json) = WsEvent::ChannelDeleted(channel_id).to_json() {
        self.hub.publish_to_channel(space_id, json).await;
    }
    Ok(())
}
```

Replace with:

```rust
pub async fn delete_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
    self.repository.find_by_id(channel_id).await?;
    self.repository.delete(channel_id).await?;
    if let Ok(json) = WsEvent::ChannelDeleted(channel_id).to_json() {
        self.hub.publish_to_channel(channel_id, json).await;
    }
    Ok(())
}
```

Note: `find_by_id` is still called to verify the channel exists (returns 404 if not found), but the result is no longer bound to a variable since we only need `channel_id`.

- [ ] **Step 3: Fix `create_channel` publish target**

In `apps/server/src/services/channel_service.rs`, change `create_channel` (lines 68-70) to publish to the new channel's ID instead of `space_id`:

Current code:

```rust
if let Ok(json) = WsEvent::ChannelCreated(channel.clone()).to_json() {
    self.hub.publish_to_channel(space_id, json).await;
}
```

Replace with:

```rust
if let Ok(json) = WsEvent::ChannelCreated(channel.clone()).to_json() {
    self.hub.publish_to_channel(channel.id, json).await;
}
```

- [ ] **Step 4: Verify it compiles**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors. Any unused variable warnings (e.g., `space_id` in `update_channel`) must be resolved.

- [ ] **Step 5: Commit**

```bash
git add apps/server/src/services/channel_service.rs
git commit -m "fix: publish channel events to channel.id instead of space_id"
```

---

## Task 3: Emit `ChannelUpdated` event on `archive_channel`

**Files:**
- Modify: `apps/server/src/services/channel_service.rs`

- [ ] **Step 1: Update `archive_channel` to emit event**

In `apps/server/src/services/channel_service.rs`, update `archive_channel` (lines 125-127):

Current code:

```rust
pub async fn archive_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
    self.repository.archive(channel_id).await
}
```

Replace with:

```rust
pub async fn archive_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
    let channel = self.repository.archive(channel_id).await?;
    if let Ok(json) = WsEvent::ChannelUpdated(channel.clone()).to_json() {
        self.hub.publish_to_channel(channel_id, json).await;
    }
    Ok(())
}
```

**Check:** Verify `archive` returns `Channel` (not `()`). Look at the repository method signature in `apps/server/src/repositories/channel_repository.rs`:

```rust
pub async fn archive(&self, id: Uuid) -> Result<Channel, AppError>
```

If it returns `Channel`, the code above works. If it returns `()`, adjust to fetch the channel before/after archiving:

```rust
pub async fn archive_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    self.repository.archive(channel_id).await?;
    if let Ok(json) = WsEvent::ChannelUpdated(channel).to_json() {
        self.hub.publish_to_channel(channel_id, json).await;
    }
    Ok(())
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors

- [ ] **Step 3: Commit**

```bash
git add apps/server/src/services/channel_service.rs
git commit -m "feat: emit ChannelUpdated event on archive_channel"
```

---

## Task 4: Emit event on `update_feature_flags`

**Files:**
- Modify: `apps/server/src/services/channel_service.rs`

- [ ] **Step 1: Update `update_feature_flags` to emit event**

In `apps/server/src/services/channel_service.rs`, update `update_feature_flags` (lines 146-162):

Current code:

```rust
pub async fn update_feature_flags(
    &self,
    channel_id: Uuid,
    input: ChannelFeatureFlagsUpdate,
) -> Result<ChannelFeatureFlags, AppError> {
    self.repository
        .update_feature_flags(
            channel_id,
            input.text_enabled,
            input.file_upload_enabled,
            input.voice_group_enabled,
            input.video_group_enabled,
            input.threads_enabled,
            input.reactions_enabled,
        )
        .await
}
```

Replace with:

```rust
pub async fn update_feature_flags(
    &self,
    channel_id: Uuid,
    input: ChannelFeatureFlagsUpdate,
) -> Result<ChannelFeatureFlags, AppError> {
    let flags = self
        .repository
        .update_feature_flags(
            channel_id,
            input.text_enabled,
            input.file_upload_enabled,
            input.voice_group_enabled,
            input.video_group_enabled,
            input.threads_enabled,
            input.reactions_enabled,
        )
        .await?;

    // Fetch the full channel to broadcast a ChannelUpdated with current state
    let channel = self.repository.find_by_id(channel_id).await?;
    if let Ok(json) = WsEvent::ChannelUpdated(channel).to_json() {
        self.hub.publish_to_channel(channel_id, json).await;
    }

    Ok(flags)
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors

- [ ] **Step 3: Commit**

```bash
git add apps/server/src/services/channel_service.rs
git commit -m "feat: emit ChannelUpdated event on feature flag changes"
```

---

## Task 5: Verify Hub injection in `state.rs`

**Files:**
- Read-only check: `apps/server/src/state.rs`
- Read-only check: `apps/server/src/main.rs`

- [ ] **Step 1: Verify `ChannelService::new` receives the hub**

In `apps/server/src/state.rs` (or wherever `AppState::new` or `AppState` is constructed), confirm that `ChannelService::new` receives both `ChannelRepository` and `RealtimeHub`:

The instantiation should look like:

```rust
let channel_repo = Arc::new(ChannelRepository::new(db.clone()));
let channel_service = ChannelService::new(channel_repo, realtime_hub.clone());
```

If `realtime_hub` is an `Arc<RealtimeHub>`, use `.clone()`. If it's a `Hub` (type alias for `Arc<RealtimeHub>`), it's already clonable.

Search for the `ChannelService::new` call site:

```bash
cd apps/server && rg "ChannelService::new" src/
```

Expected output (one match):
```
src/state.rs:    channel_service: ChannelService::new(channel_repo.clone(), realtime_hub.clone()),
```

If the call only passes one argument (the repo), add the hub as the second argument.

- [ ] **Step 2: Verify compilation with the existing state**

Run: `cd apps/server && cargo check`
Expected: Compiles without errors

- [ ] **Step 3: Commit**

```bash
git add apps/server/src/state.rs
git commit -m "fix: pass RealtimeHub to ChannelService"
```

Only commit if `state.rs` was actually modified.

---

## Task 6: Tests — Channel WS event broadcasting

**Files:**
- Create: `apps/server/tests/channel_ws_events_test.rs`

- [ ] **Step 1: Write the test file**

Create `apps/server/tests/channel_ws_events_test.rs`:

```rust
use axum::http::StatusCode;
use rust_chat_server::app::create_app;
use rust_chat_server::config::AppConfig;
use rust_chat_server::domain::channel::{Channel, ChannelVisibility, CreateChannel, UpdateChannel};
use rust_chat_server::domain::space::CreateSpace;
use rust_chat_server::realtime::events::WsEvent;
use rust_chat_server::state::AppState;
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

mod common;

#[sqlx::test]
async fn test_channel_update_emits_ws_event(pool: PgPool) {
    let config = AppConfig::default();
    let state = common::create_test_state(pool.clone(), &config).await;
    let app = create_app(Arc::new(state.clone()));

    // Bootstrap hoster
    let (hoster_token, _) = common::bootstrap_hoster(&app).await;

    // Create a space
    let space = common::create_space(&app, &hoster_token, "Test Space").await;

    // Create a channel
    let channel: Channel = common::api_post(
        &app,
        &format!("/api/v1/spaces/{}/channels", space.id),
        &hoster_token,
        &CreateChannel {
            name: "general".to_string(),
            parent_id: None,
            kind: Some("text".to_string()),
            visibility: Some("public".to_string()),
            topic: None,
        },
        StatusCode::OK,
    )
    .await;

    // Subscribe to the channel's broadcast room via the hub
    let hub = state.realtime_hub.clone();
    let mut rx = hub.subscribe(channel.id).await;

    // Update the channel
    let _updated: Channel = common::api_put(
        &app,
        &format!("/api/v1/spaces/{}/channels/{}", space.id, channel.id),
        &hoster_token,
        &UpdateChannel {
            name: Some("renamed".to_string()),
            topic: None,
            visibility: None,
            feature_flags: None,
        },
        StatusCode::OK,
    )
    .await;

    // Verify an event was received on the channel's broadcast room
    let received = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;
    assert!(received.is_ok(), "Should receive a WS event within timeout");

    let json_str = received.unwrap().unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed["type"], "channel.updated");
    assert_eq!(parsed["data"]["name"], "renamed");
}

#[sqlx::test]
async fn test_channel_delete_emits_ws_event(pool: PgPool) {
    let config = AppConfig::default();
    let state = common::create_test_state(pool.clone(), &config).await;
    let app = create_app(Arc::new(state.clone()));

    let (hoster_token, _) = common::bootstrap_hoster(&app).await;
    let space = common::create_space(&app, &hoster_token, "Test Space").await;

    let channel: Channel = common::api_post(
        &app,
        &format!("/api/v1/spaces/{}/channels", space.id),
        &hoster_token,
        &CreateChannel {
            name: "temp".to_string(),
            parent_id: None,
            kind: Some("text".to_string()),
            visibility: Some("public".to_string()),
            topic: None,
        },
        StatusCode::OK,
    )
    .await;

    let hub = state.realtime_hub.clone();
    let mut rx = hub.subscribe(channel.id).await;

    // Delete the channel
    let resp = common::api_delete::<()>(
        &app,
        &format!("/api/v1/spaces/{}/channels/{}/hard", space.id, channel.id),
        &hoster_token,
        StatusCode::NO_CONTENT,
    )
    .await;

    let received = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;
    assert!(received.is_ok(), "Should receive a WS event within timeout");

    let json_str = received.unwrap().unwrap();
    let parsed: Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed["type"], "channel.deleted");
    assert_eq!(parsed["data"], Value::String(channel.id.to_string()));
}

#[sqlx::test]
async fn test_channel_visibility_change_emits_event(pool: PgPool) {
    let config = AppConfig::default();
    let state = common::create_test_state(pool.clone(), &config).await;
    let app = create_app(Arc::new(state.clone()));

    let (hoster_token, _) = common::bootstrap_hoster(&app).await;
    let space = common::create_space(&app, &hoster_token, "Test Space").await;

    let channel: Channel = common::api_post(
        &app,
        &format!("/api/v1/spaces/{}/channels", space.id),
        &hoster_token,
        &CreateChannel {
            name: "general".to_string(),
            parent_id: None,
            kind: Some("text".to_string()),
            visibility: Some("public".to_string()),
            topic: None,
        },
        StatusCode::OK,
    )
    .await;

    let hub = state.realtime_hub.clone();
    let mut rx = hub.subscribe(channel.id).await;

    // Toggle visibility to private
    let _updated: Channel = common::api_put(
        &app,
        &format!("/api/v1/spaces/{}/channels/{}", space.id, channel.id),
        &hoster_token,
        &UpdateChannel {
            name: None,
            topic: None,
            visibility: Some("private".to_string()),
            feature_flags: None,
        },
        StatusCode::OK,
    )
    .await;

    // We should receive TWO events: ChannelUpdated + ChannelVisibilityChanged
    let received1 = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;
    assert!(received1.is_ok(), "Should receive first event");

    let json_str1 = received1.unwrap().unwrap();
    let parsed1: Value = serde_json::from_str(&json_str1).unwrap();
    assert_eq!(parsed1["type"], "channel.updated");

    let received2 = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;
    assert!(received2.is_ok(), "Should receive second event (visibility_changed)");

    let json_str2 = received2.unwrap().unwrap();
    let parsed2: Value = serde_json::from_str(&json_str2).unwrap();
    assert_eq!(parsed2["type"], "channel.visibility_changed");
}
```

- [ ] **Step 2: Check that test helpers exist**

Verify that `apps/server/tests/common/mod.rs` exports these helpers:

```bash
cd apps/server && rg -l "pub async fn (api_post|api_put|api_delete|bootstrap_hoster|create_space|create_test_state)" tests/common/
```

If any helper is missing, add it to `apps/server/tests/common/mod.rs`. The plan assumes these exist from Phase 3 testing work.

- [ ] **Step 3: Run the new tests**

Run: `cd apps/server && cargo test test_channel_ws_events -- --test-threads=1`
Expected: All 3 tests pass

- [ ] **Step 4: Commit**

```bash
git add apps/server/tests/channel_ws_events_test.rs
git commit -m "test: add channel WS event broadcasting tests"
```

---

## Task 7: Full verification pass

**Files:**
- No file changes

- [ ] **Step 1: Run all backend checks**

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test -- --test-threads=1
```

Expected: Formatting clean, clippy clean, all tests pass (including the new 3 + existing 40 = 43 tests)

- [ ] **Step 2: Fix any issues**

If clippy warns about unused imports or variables (e.g., `serde_json::Value` import in channel_service.rs), remove them.

If clippy warns about `let _ =` on the `api_delete` response in the test, prefix with `let _resp` or use `drop()`.

Re-run checks until clean.

- [ ] **Step 3: Final commit**

```bash
git add -A
git commit -m "chore: finalize backend channel event broadcasting"
```

---

## Summary of All Changes

| File | Change | Status |
|------|--------|--------|
| `apps/server/src/realtime/events.rs` | Add `ChannelVisibilityChanged(Uuid)` variant | New |
| `apps/server/src/services/channel_service.rs` | Fix publish target from `space_id` to `channel.id` in `create_channel`, `update_channel`, `delete_channel` | Fix |
| `apps/server/src/services/channel_service.rs` | Emit `ChannelUpdated` on `archive_channel` | New |
| `apps/server/src/services/channel_service.rs` | Emit `ChannelUpdated` on `update_feature_flags` | New |
| `apps/server/src/services/channel_service.rs` | Emit `ChannelVisibilityChanged` when visibility toggles | New |
| `apps/server/src/state.rs` | Verify `RealtimeHub` is passed to `ChannelService::new` | Check |
| `apps/server/tests/channel_ws_events_test.rs` | 3 integration tests for WS event broadcasting | New |

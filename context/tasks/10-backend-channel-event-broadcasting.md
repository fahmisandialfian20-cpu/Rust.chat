# Backend Channel Event Broadcasting

**Goal:** Emit WebSocket events when channels are updated or deleted so frontend channel lists stay synchronized.

**Scope:** Backend only — add event publishing to channel service methods.

**Non-goals:** New WS event types, frontend changes, new handlers.

**Priority:** High — required for frontend channel visibility to work end-to-end.

---

## Current State

WS event types already defined in `events.rs`:
```rust
ChannelCreated(Channel),
ChannelUpdated(Channel),
ChannelDeleted(Uuid),
```

Frontend already subscribes to these events (or will after `frontend-channel-visibility-complete`).

**Missing:** Backend never emits these events. Only `channel.created` is emitted indirectly through existing flow.

---

## Required Changes

### 1. Inject RealtimeHub into ChannelService

Current:
```rust
pub struct ChannelService {
    repository: Arc<ChannelRepository>,
}
```

New:
```rust
pub struct ChannelService {
    repository: Arc<ChannelRepository>,
    hub: Arc<RealtimeHub>,
}
```

Update constructor in `AppState`:
```rust
channel_service: ChannelService::new(channel_repo.clone(), realtime_hub.clone()),
```

### 2. Publish Events on Mutations

In `ChannelService::update_channel()`:
```rust
pub async fn update_channel(&self, id: Uuid, data: UpdateChannel) -> Result<Channel, AppError> {
    let channel = self.repository.update(id, data).await?;
    
    self.hub.publish_to_channel(
        channel.id,
        WsEvent::ChannelUpdated(channel.clone()).to_json()?
    ).await;
    
    Ok(channel)
}
```

In `ChannelService::delete_channel()`:
```rust
pub async fn delete_channel(&self, id: Uuid) -> Result<(), AppError> {
    let channel = self.repository.find_by_id(id).await?;
    
    self.repository.delete(id).await?;
    
    self.hub.publish_to_channel(
        id,
        WsEvent::ChannelDeleted(id).to_json()?
    ).await;
    
    Ok(())
}
```

### 3. Handle Visibility Changes

When channel visibility toggles between Public/Private:
```rust
// After visibility update
self.hub.publish_to_channel(
    channel.id,
    WsEvent::ChannelVisibilityChanged(channel.id).to_json()?
).await;
```

Note: May need new event type `ChannelVisibilityChanged(Uuid)` if not already defined.

---

## Files to Change

| File | Change |
|------|--------|
| `services/channel_service.rs` | Add `hub` field; publish events in update/delete |
| `state.rs` | Pass `realtime_hub` to `ChannelService::new()` |
| `realtime/events.rs` | Add `ChannelVisibilityChanged` if missing |

---

## Acceptance Criteria

1. Admin renames channel → WS event `channel.updated` broadcast to subscribers
2. Admin deletes channel → WS event `channel.deleted` broadcast
3. Admin changes visibility → WS event triggers frontend re-fetch
4. Events only sent to clients subscribed to that space's channels

---

## Verification

```bash
cd apps/server
cargo test -- --test-threads=1
cargo clippy -- -D warnings
cargo fmt --check
```

Integration test: Connect WS client, trigger channel update via API, verify event received.

---

## References

- `context/tasks/gap-analysis-mvp-completion.md` — Item #3
- `apps/server/src/services/channel_service.rs` — current service
- `apps/server/src/realtime/hub.rs` — publish methods
- `apps/server/src/realtime/events.rs` — event types

---

**Created:** 2026-05-05
**Depends on:** Phase 3 Critical Fixes ✅
**Estimated effort:** Small (0.5 session)
**Risk:** Very low — adding calls to existing infrastructure

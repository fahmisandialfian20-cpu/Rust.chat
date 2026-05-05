# Frontend Channel Visibility Completion — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete real-time channel list synchronization so all channel mutations (create, update, delete, visibility change) are reflected in every user's sidebar within 2 seconds, with permission-based action visibility.

**Architecture:** The space layout (`+layout.svelte`) connects to the WebSocket hub and subscribes to `channel.*` events. Event payloads mutate a reactive `channels: Channel[]` array. The `ChannelList` component derives `canManageChannels` from a `permissions` string array prop. The `MessageList` component shows a read-only badge when the user lacks `SendMessages`.

**Tech Stack:** SvelteKit 5 (runes), TypeScript, Zod, Tailwind CSS, Rust Axum backend

---

## Task 1: Backend — Add `channel.visibility_changed` event type + broadcast

**Files:**
- Modify: `apps/server/src/realtime/events.rs`
- Modify: `apps/server/src/services/channel_service.rs`

- [ ] **Step 1: Add `ChannelVisibilityChanged` variant to `WsEvent` enum**

In `apps/server/src/realtime/events.rs`, add after the `ChannelDeleted(Uuid)` variant:

```rust
#[serde(rename = "channel.visibility_changed")]
ChannelVisibilityChanged(Uuid),
```

- [ ] **Step 2: Inject `RealtimeHub` into `ChannelService`**

In `apps/server/src/services/channel_service.rs`, add the import and Hub field:

```rust
use crate::realtime::events::WsEvent;
use crate::realtime::Hub;
use std::sync::Arc;

#[derive(Clone)]
pub struct ChannelService {
    repository: Arc<ChannelRepository>,
    hub: Arc<Hub>,
}

impl ChannelService {
    pub fn new(repository: Arc<ChannelRepository>, hub: Arc<Hub>) -> Self {
        Self { repository, hub }
    }

    pub async fn update_channel_visibility(
        &self,
        channel_id: Uuid,
        visibility: Visibility,
    ) -> Result<Channel, AppError> {
        let channel = self.repository.find_by_id(channel_id).await?;
        let space_id = channel.space_id;
        let updated = self.repository.update_visibility(channel_id, visibility).await?;
        let _ = self
            .hub
            .publish_to_channel(
                space_id,
                WsEvent::ChannelVisibilityChanged(channel_id).to_json()?,
            )
            .await;
        Ok(updated)
    }
}
```

- [ ] **Step 3: Update `AppState` to pass hub to `ChannelService`**

Find the `ChannelService::new(...)` call in `apps/server/src/state.rs` (or `main.rs`) and update:

```rust
channel_service: ChannelService::new(
    Arc::new(ChannelRepository::new(db.clone())),
    realtime_hub.clone(),
),
```

- [ ] **Step 4: Broadcast on existing `update_channel` and `delete_channel`**

In `update_channel`, broadcast `WsEvent::ChannelUpdated`:
```rust
pub async fn update_channel(
    &self,
    channel_id: Uuid,
    input: UpdateChannel,
) -> Result<Channel, AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    let space_id = channel.space_id;
    let updated = self.repository.update(channel_id, input).await?;
    let _ = self
        .hub
        .publish_to_channel(space_id, WsEvent::ChannelUpdated(updated.clone()).to_json()?)
        .await;
    Ok(updated)
}
```

In `delete_channel`, broadcast `WsEvent::ChannelDeleted`:
```rust
pub async fn delete_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
    let channel = self.repository.find_by_id(channel_id).await?;
    let space_id = channel.space_id;
    self.repository.delete(channel_id).await?;
    let _ = self
        .hub
        .publish_to_channel(space_id, WsEvent::ChannelDeleted(channel_id).to_json()?)
        .await;
    Ok(())
}
```

- [ ] **Step 5: Verify it compiles**

```bash
cd apps/server
cargo check
```
Expected: Compiles without errors

---

## Task 2: Frontend — Add `channel.visibility_changed` to WS schema

**Files:**
- Modify: `apps/web/src/lib/schemas/websocket.ts`

- [ ] **Step 1: Add event type string to `WS_EVENT_TYPES`**

In `apps/web/src/lib/schemas/websocket.ts`, add `'channel.visibility_changed'` after `'channel.deleted'` in the array:

```typescript
export const WS_EVENT_TYPES = [
  'hello.ok',
  'message.created',
  'message.updated',
  'message.deleted',
  'typing.updated',
  'presence.updated',
  'channel.created',
  'channel.updated',
  'channel.deleted',
  'permission.updated',
  'member.joined',
  'member.left',
  'notification.created',
  'media.room.updated',
  'channel.visibility_changed',
  'error',
] as const;
```

- [ ] **Step 2: Add payload schema**

Add after `channelDeletedPayloadSchema`:

```typescript
export const channelVisibilityChangedPayloadSchema = z.object({
  channel_id: z.string().min(1),
});
export type ChannelVisibilityChangedPayload = z.infer<typeof channelVisibilityChangedPayloadSchema>;
```

- [ ] **Step 3: Add to `EventPayloadMap`**

Add:
```typescript
  'channel.visibility_changed': ChannelVisibilityChangedPayload;
```

- [ ] **Step 4: Add to `payloadSchemaByType`**

Add:
```typescript
  'channel.visibility_changed': channelVisibilityChangedPayloadSchema,
```

- [ ] **Step 5: Verify it compiles**

```bash
cd apps/web
npm run check
```
Expected: No errors

---

## Task 3: Frontend — Add `channel.visibility_changed` subscription in layout

**Files:**
- Modify: `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte`

- [ ] **Step 1: Add subscription inside `onMount`**

After the `unsubDeleted` subscription, add:

```typescript
const unsubVisibilityChanged = realtime.subscribe('channel.visibility_changed', () => {
  listVisibleChannels(spaceId).then(result => channels = result);
});
```

- [ ] **Step 2: Add cleanup**

Add `unsubVisibilityChanged()` to the return statement:

```typescript
return () => {
  unsubCreated();
  unsubUpdated();
  unsubDeleted();
  unsubVisibilityChanged();
};
```

The complete `onMount` block now looks like:

```typescript
onMount(() => {
  const token = getAccessToken();
  if (!token) {
    goto('/login');
    return;
  }

  realtime.connect(token);
  load();

  const unsubCreated = realtime.subscribe('channel.created', (payload) => {
    const ch = payload as Channel;
    if (ch.space_id === spaceId) {
      channels = [...channels, ch];
    }
  });

  const unsubUpdated = realtime.subscribe('channel.updated', (payload) => {
    const ch = payload as Channel;
    if (ch.space_id === spaceId) {
      channels = channels.map(c => c.id === ch.id ? ch : c);
    }
  });

  const unsubDeleted = realtime.subscribe('channel.deleted', (payload) => {
    const p = payload as { channel_id: string };
    channels = channels.filter(c => c.id !== p.channel_id);
  });

  const unsubVisibilityChanged = realtime.subscribe('channel.visibility_changed', () => {
    listVisibleChannels(spaceId).then(result => channels = result);
  });

  return () => {
    unsubCreated();
    unsubUpdated();
    unsubDeleted();
    unsubVisibilityChanged();
  };
});
```

- [ ] **Step 2: Verify it compiles**

```bash
cd apps/web
npm run check
```
Expected: No errors

---

## Task 4: Frontend — Verify permission controls in ChannelList

**No code changes needed.** Verify the existing `ChannelList.svelte` already has:

| Requirement | Status | Location |
|---|---|---|
| Accepts `permissions` prop | ✅ | `ChannelList.svelte:6` |
| `canManageChannels` derived | ✅ | `ChannelList.svelte:10` |
| Empty state with CTA for admin | ✅ | `ChannelList.svelte:15-17` |
| Empty state info for non-admin | ✅ | `ChannelList.svelte:19-20` |
| "+" Add Channel button gated on `canManageChannels` | ✅ | `ChannelList.svelte:44-54` |
| Channels sorted by `position` | ✅ | `ChannelList.svelte:8` |
| Active channel highlight | ✅ | `ChannelList.svelte:30-32` |

- [ ] **Step 1: Confirm permissions are fetched in layout**

Verify `+layout.svelte` already does:
```typescript
const perms = await getMyPermissions(spaceId).catch(() => []);
permissions = perms;
```

- [ ] **Step 2: Confirm permissions are passed to ChannelList**

Verify `+layout.svelte` already does:
```svelte
<ChannelList {channels} {permissions} />
```

---

## Task 5: Frontend — Verify send-message permission controls

**No code changes needed.** Verify the existing channel page already has:

| Requirement | Status | Location |
|---|---|---|
| Fetches permissions in `load()` | ✅ | `+page.svelte:98` |
| `canSendMessages` derived | ✅ | `+page.svelte:35` |
| `composerDisabled` checks `canSendMessages` | ✅ | `+page.svelte:43` |
| `composerDisabledReason` shows permission message | ✅ | `+page.svelte:50` |
| MessageList `readOnly` prop | ✅ | `MessageList.svelte:11` |
| Read-only badge rendered | ✅ | `MessageList.svelte:56-61` |
| Channel page passes `readOnly={!canSendMessages}` | ✅ | `+page.svelte:224` |

- [ ] **Step 1: Verify the disabled composer shows reason**

When user lacks `send_messages`, the composer should show:
> "You do not have permission to send messages."

And the textarea should be disabled.

---

## Task 6: Verification

- [ ] **Step 1: Run backend checks**

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Expected: Formatting clean, clippy clean, all tests pass

- [ ] **Step 2: Run frontend checks**

```bash
cd apps/web
npm run check
npm run build
```

Expected: Both commands succeed (0 errors, 0 warnings)

- [ ] **Step 3: Fix any issues found**

If any verification fails, fix and re-run until clean.

---

## Acceptance Criteria Checklist

| # | Criterion | How Verified |
|---|-----------|-------------|
| 1 | Admin renames channel → both users see new name within 2s | WS `channel.updated` subscription in layout; broadcast in `ChannelService::update_channel()` |
| 2 | Admin deletes channel → removed from all users' lists | WS `channel.deleted` subscription in layout; broadcast in `ChannelService::delete_channel()` |
| 3 | Admin changes visibility → authorized users see it appear/disappear | WS `channel.visibility_changed` triggers `listVisibleChannels()` re-fetch |
| 4 | User with `ManageChannels` sees "+" button; user without does not | `canManageChannels` derived from `permissions` prop in `ChannelList.svelte` |
| 5 | Empty state shows CTA only to users with `ManageChannels` | Conditional rendering in `ChannelList.svelte` lines 14-21 |

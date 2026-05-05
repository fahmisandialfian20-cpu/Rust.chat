# Frontend Channel Visibility Design

**Date:** 2026-05-05
**Status:** Approved
**Depends on:** Phase 3 (backend security) ✅

---

## Summary

Integrate backend permission system into frontend channel list so users only see channels they are authorized to view, with real-time updates via WebSocket.

---

## Backend Changes

### 1. PermissionService — `list_user_permissions`

Add method to `PermissionService` that checks all `PermissionKey` variants for a given user in a space:

```rust
pub async fn list_user_permissions(
    &self,
    user_id: Uuid,
    space_id: Uuid,
) -> Result<Vec<String>, AppError>
```

Iterates all PermissionKey variants, calls `check_optional` for each with `channel_id: None`, returns array of allowed permission key strings.

### 2. New handler — `get_my_permissions`

New file `handlers/permissions.rs`:

```
GET /api/v1/spaces/{spaceId}/my-permissions
Auth: Bearer token required
Response: Json<Vec<String>>
```

Returns the list of permission keys the authenticated user has in the given space.

### 3. WebSocket events — add channel events

Add to `realtime/events.rs`:

```rust
#[serde(rename = "channel.created")]
ChannelCreated(Channel),

#[serde(rename = "channel.updated")]
ChannelUpdated(Channel),

#[serde(rename = "channel.deleted")]
ChannelDeleted(Uuid),
```

### 4. Broadcast from channel_service

When a channel is created/updated/deleted, broadcast the corresponding WS event to the space's channel.

---

## Frontend Changes

### 1. API — `getMyPermissions`

Add to `$lib/api/channels.ts`:

```typescript
export async function getMyPermissions(spaceId: string): Promise<string[]>;
// GET /api/v1/spaces/{spaceId}/my-permissions
```

### 2. WebSocket — add channel.deleted event type

Add to `$lib/schemas/websocket.ts`:
- `'channel.deleted'` to WS_EVENT_TYPES
- Add `channelDeletedPayloadSchema` (shape: `{ channel_id: string }`)
- Add to payloadSchemaByType mapping
- Add to EventPayloadMap

### 3. Space layout — real-time channel sync

In `+layout.svelte`:

- Call `realtime.connect(token)` in onMount
- Subscribe to `channel.created` — append new channel if space_id matches
- Subscribe to `channel.updated` — replace in channels array
- Subscribe to `channel.deleted` — remove from channels array
- Fetch permissions via `getMyPermissions(spaceId)` on mount
- Pass both `channels` and `permissions` to ChannelList
- Pass `hasPermission` callback to child routes (for MessageComposer/MessageList)

### 4. ChannelList — permission-gated UI

- Accept `permissions: string[]` prop
- Show "+" create-channel button only if `"manage_channels"` in permissions
- Show empty state based on permissions:
  - No channels + has `manage_channels` → "Create your first channel" CTA
  - No channels + no permission → "No channels available. Ask an admin."

### 5. MessageComposer — disabled based on permissions

- Accept `canSendMessages: boolean` prop (determined in page from permissions)
- Show disabled state with tooltip when no SendMessages permission

### 6. MessageList — "Read-only" badge

- Accept `readOnly: boolean` prop
- Show badge at top when user lacks SendMessages

### 7. Error handling

- 403 on specific channel → "You don't have access" in main content area (already partially done)
- Network failure → auto-retry with exponential backoff max 3 attempts (for channel list loading)

---

## Acceptance Criteria

1. User A removed from private channel → channel disappears from list within 2s (WS event)
2. Admin creates channel → authorized members see it immediately (WS event)
3. User without SendMessages sees disabled composer with reason
4. User without ManageChannels sees no "Create channel" button
5. All changes compile: `npm run check && npm run build`

---

## Files Changed

| File | Change |
|------|--------|
| `server/src/permissions/service.rs` | Add `list_user_permissions()` |
| `server/src/handlers/permissions.rs` | New: `get_my_permissions` handler |
| `server/src/handlers/mod.rs` | Add `permissions` module |
| `server/src/routes/mod.rs` | Register route |
| `server/src/realtime/events.rs` | Add channel WS events |
| `server/src/services/channel_service.rs` | Broadcast WS events |
| `web/src/lib/api/channels.ts` | Add `getMyPermissions()` |
| `web/src/lib/schemas/websocket.ts` | Add `channel.deleted` event |
| `web/src/routes/(app)/spaces/[spaceId]/+layout.svelte` | WS subscribe, permissions |
| `web/src/lib/components/channels/ChannelList.svelte` | Permissions prop, CTA |
| `web/src/lib/components/chat/MessageComposer.svelte` | Permissions-gated disable |
| `web/src/lib/components/chat/MessageList.svelte` | Read-only badge |

---

## Verification

```bash
cd apps/server && cargo clippy -- -D warnings && cargo test
cd apps/web && npm run check && npm run build
```

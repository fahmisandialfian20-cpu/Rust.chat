# Frontend Channel Visibility

**Goal:** Integrate backend permission system into frontend channel list so users only see channels they are authorized to view, with real-time updates.

**Scope:** Frontend only. Backend already secured in Phase 3.

**Non-goals:** New features (voice/video), admin panel redesign, mobile layout.

**Priority:** High — security feature gap between backend enforcement and frontend UX.

---

## Current State

Backend (`Phase 3 ✅`):
- `GET /api/v1/spaces/{id}/channels/visible` — returns only authorized channels
- Permission checks enforced server-side (ViewChannel, space membership)
- WebSocket broadcasts `channel.created`, `channel.updated`, `channel.deleted` events

Frontend (`Partial 🟡`):
- `listVisibleChannels()` API exists → `$lib/api/channels.ts`
- `ChannelList.svelte` renders channels passed as props
- Space layout loads channels once on `onMount`

**Gap:** No real-time sync. UI stale after backend permission changes.

---

## Required Changes

### 1. Real-Time Channel List Sync

Subscribe to WebSocket events in space layout. Update `channels` array reactively.

```typescript
// In space layout
import { realtime } from '$lib/stores/realtime.svelte';

onMount(() => {
  const unsub = realtime.on('channel.created', (ch) => {
    if (ch.space_id === spaceId) channels = [...channels, ch];
  });
  return unsub;
});
```

Events to handle:
- `channel.created` — append if space matches
- `channel.updated` — replace in array
- `channel.deleted` — remove from array
- `channel.visibility_changed` — re-fetch via `listVisibleChannels()`

### 2. Permission-Based UI Controls

Hide/show actions based on user's permissions in current space.

| Action | Permission Key | UI Element |
|--------|---------------|------------|
| Create channel | `ManageChannels` | "+" button in sidebar |
| Edit channel | `ManageChannels` | Context menu item |
| Delete channel | `ManageChannels` | Context menu item |
| Send message | `SendMessages` | Message composer |
| Upload file | `SendFiles` | Attach button |

Fetch permissions on space load:
```typescript
const perms = await getMyPermissions(spaceId); // need new API
```

### 3. Empty States

Current: "No visible channels" (generic)

Improve:
- No channels + has `ManageChannels` → "Create your first channel" CTA
- No channels + no permission → "No channels available. Ask an admin."
- Private space + not member → Redirect to lobby with toast

### 4. Error Handling

Current: Redirects to login on 401, shows "forbidden" alert on 403.

Add:
- 403 on specific channel → show "You don't have access" in main content area
- Network failure → auto-retry with exponential backoff (max 3 attempts)

---

## Files to Change

| File | Change |
|------|--------|
| `$lib/api/channels.ts` | Add `getMyPermissions()` helper |
| `$lib/stores/realtime.svelte.ts` | Add channel event handlers |
| `$routes/(app)/spaces/[spaceId]/+layout.svelte` | Subscribe to WS, pass perms to children |
| `$lib/components/channels/ChannelList.svelte` | Accept `permissions` prop, show/hide actions |
| `$lib/components/chat/MessageComposer.svelte` | Disable send if no `SendMessages` |
| `$lib/components/chat/MessageList.svelte` | Show "Read-only" badge if no `SendMessages` |

---

## API Additions Needed

```typescript
// GET /api/v1/spaces/{spaceId}/my-permissions
// Returns: PermissionKey[]
export async function getMyPermissions(spaceId: string): Promise<string[]>;
```

Backend already has `PermissionService::list_user_permissions()` — expose via handler.

---

## Acceptance Criteria

1. User A removes User B from private channel → User B's channel list removes it within 2 seconds
2. Admin creates new channel → members with ViewChannel see it immediately
3. User without `SendMessages` sees disabled composer with tooltip
4. User without `ManageChannels` sees no "Create channel" button
5. All changes compile: `npm run check && npm run build`

---

## Verification

```bash
cd apps/web
npm run check
npm run build
```

Manual test:
1. Open space with 2 users in different browsers
2. Admin creates private channel, assigns User A
3. User A sees new channel appear; User B does not

---

## References

- `context/03-domain-permissions.md` — Permission model
- `context/tasks/phase3-e2e-security-hardening.md` — Backend security completed
- `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte` — Current space layout
- `apps/web/src/lib/components/channels/ChannelList.svelte` — Current channel list
- `apps/server/src/handlers/channels.rs:160` — `list_visible_channels` handler

---

**Created:** 2026-05-05
**Depends on:** Phase 3 (backend security) ✅
**Estimated effort:** Small (1-2 sessions)
**Risk:** Low — frontend-only, backend already stable

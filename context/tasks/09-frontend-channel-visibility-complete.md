# Complete Frontend Channel Visibility

**Goal:** Finish real-time channel list synchronization so users see accurate channel lists with live updates from WebSocket events.

**Scope:** Frontend SvelteKit components and stores. Backend WS events already defined.

**Non-goals:** Permission-based UI controls (separate task), message composer changes, admin panel.

**Priority:** High — completes MVP criteria #6 (members only see authorized channels).

---

## Current State

Agent coder implemented base:
- `getMyPermissions()` API client ✅
- Realtime WS connection in space layout ✅
- `channel.created` subscription ✅
- Retry logic with exponential backoff ✅

**Missing:**
- `channel.updated` — rename, position change not reflected
- `channel.deleted` — deleted channels remain in list
- `channel.visibility_changed` — visibility changes not handled
- Permission-based action buttons (+ button, context menu)
- Context-aware empty states

---

## Required Changes

### 1. Handle All Channel Events

In `+layout.svelte` (space layout), extend WS subscription:

```typescript
// channel.updated
realtime.subscribe('channel.updated', (payload) => {
  const ch = payload as Channel;
  channels = channels.map(c => c.id === ch.id ? ch : c);
});

// channel.deleted
realtime.subscribe('channel.deleted', (payload) => {
  const id = payload as string;
  channels = channels.filter(c => c.id !== id);
});

// channel.visibility_changed
realtime.subscribe('channel.visibility_changed', () => {
  // Re-fetch visible channels from backend
  listVisibleChannels(spaceId).then(result => channels = result);
});
```

### 2. Permission-Based Actions in ChannelList

Pass `permissions` prop to `ChannelList`:

```svelte
<ChannelList {channels} permissions={permissions} />
```

Show/hide based on `ManageChannels`:
- "+" button (create channel) — only if has `ManageChannels`
- Context menu items (edit, delete) — only if has `ManageChannels`

### 3. Empty States

Replace generic "No visible channels":

- No channels + has `ManageChannels`:
  ```
  No channels yet. Create your first channel to get started.
  [Create Channel] ← button
  ```

- No channels + no permission:
  ```
  No channels available. Ask a space admin to create one.
  ```

### 4. Active Channel Highlight

Current: highlight works via `activeChannelId` derived from `$page.params`

Verify: works correctly after channel list updates (re-sort, rename, etc.)

---

## Files to Change

| File | Change |
|------|--------|
| `routes/(app)/spaces/[spaceId]/+layout.svelte` | Add 3 more WS subscriptions; pass permissions to ChannelList |
| `lib/components/channels/ChannelList.svelte` | Accept `permissions` prop; conditionally render actions |
| `lib/stores/realtime.svelte.ts` | Ensure `subscribe()` supports all channel event types |

---

## Acceptance Criteria

1. Admin renames channel → both users see new name within 2 seconds
2. Admin deletes channel → channel removed from all users' lists
3. Admin changes channel visibility → authorized users see it appear/disappear
4. User with `ManageChannels` sees "+" button; user without does not
5. Empty state shows CTA only to users with `ManageChannels`

---

## Verification

```bash
cd apps/web
npm run check
npm run build
```

Manual test (2 browser tabs):
1. Tab A: admin view, Tab B: member view
2. Admin creates channel → member sees it appear
3. Admin renames channel → member sees new name
4. Admin deletes channel → member sees it disappear

---

## References

- `context/tasks/gap-analysis-mvp-completion.md` — Item #1
- `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte` — current layout
- `apps/web/src/lib/components/channels/ChannelList.svelte` — current list
- `apps/server/src/realtime/events.rs` — WS event types

---

**Created:** 2026-05-05
**Depends on:** Phase 3 Critical Fixes ✅, Frontend Channel Visibility (base) 🟡
**Estimated effort:** Small (1 session)
**Risk:** Low — event types already defined, patterns established

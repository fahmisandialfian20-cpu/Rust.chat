# Frontend Permission Integration

**Goal:** Disable or hide UI actions based on user's permissions so users without rights cannot attempt forbidden actions.

**Scope:** Frontend components only — message composer, message list, channel controls.

**Non-goals:** Backend changes (already enforced), new features, admin panel redesign.

**Priority:** High — completes MVP criteria #7 (members R/W/E/D messages only when permitted) on frontend side.

---

## Current State

Backend ✅: All endpoints check permissions and return 403.

Frontend 🔴: UI always shows all controls. User can click "Send" and get 403 error after attempt. Poor UX.

**Examples of current bad UX:**
- User without `SendMessages` sees active send button → clicks → 403 error
- User without `ManageChannels` sees "+" button → clicks → 403 error
- User without `SendFiles` sees attach button → clicks → 403 error

---

## Required Changes

### 1. Message Composer

File: `$lib/components/chat/MessageComposer.svelte`

Props needed: `permissions: string[]`

Behavior:
```svelte
<script>
  let { permissions = [] } = $props();
  let canSend = $derived(permissions.includes('SendMessages'));
</script>

{#if canSend}
  <textarea bind:value={content} placeholder="Type a message..."></textarea>
  <button onclick={send}>Send</button>
{:else}
  <div class="read-only-notice">
    <Lock class="size-4" />
    You don't have permission to send messages in this channel.
  </div>
{/if}
```

### 2. Message List — Read-Only Badge

File: `$lib/components/chat/MessageList.svelte`

Show badge when user lacks `SendMessages`:
```svelte
{#if !permissions.includes('SendMessages')}
  <div class="badge">Read-only</div>
{/if}
```

### 3. Message Item — Edit/Delete Controls

File: `$lib/components/chat/MessageItem.svelte`

Logic:
```typescript
let isOwnMessage = $derived(message.author_id === currentUserId);
let canEdit = $derived(
  (isOwnMessage && permissions.includes('EditOwnMessage')) ||
  permissions.includes('EditAnyMessage')
);
let canDelete = $derived(
  (isOwnMessage && permissions.includes('DeleteOwnMessage')) ||
  permissions.includes('DeleteAnyMessage')
);
```

Show edit/delete buttons only when `canEdit` / `canDelete` is true.

### 4. Channel List — Create/Edit/Delete

File: `$lib/components/channels/ChannelList.svelte`

Already covered in `frontend-channel-visibility-complete.md`, but verify:
- "+" button: requires `ManageChannels`
- Edit/delete in context menu: requires `ManageChannels`

### 5. Permission Prop Drilling

Space layout fetches permissions once:
```typescript
const perms = await getMyPermissions(spaceId);
permissions = perms;
```

Pass down through component tree:
```svelte
<ChannelList {channels} {permissions} />
<MessageComposer {permissions} />
<MessageList {permissions} />
```

---

## Files to Change

| File | Change |
|------|--------|
| `routes/(app)/spaces/[spaceId]/+layout.svelte` | Fetch and pass `permissions` |
| `routes/(app)/spaces/[spaceId]/channels/[channelId]/+page.svelte` | Pass `permissions` to children |
| `lib/components/chat/MessageComposer.svelte` | Disable/hide when no `SendMessages` |
| `lib/components/chat/MessageList.svelte` | Show "Read-only" badge |
| `lib/components/chat/MessageItem.svelte` | Conditionally show edit/delete |
| `lib/components/channels/ChannelList.svelte` | Conditionally show create/edit/delete |

---

## Acceptance Criteria

1. User without `SendMessages` sees "Read-only" badge and disabled composer
2. User without `ManageChannels` sees no "+" button in channel list
3. User can edit own message only with `EditOwnMessage`
4. User can edit any message only with `EditAnyMessage`
5. Same pattern for delete permissions
6. All permission checks are reactive (update when permissions change)

---

## Verification

```bash
cd apps/web
npm run check
npm run build
```

Manual test:
1. Create role "Viewer" with only `ReadMessages` and `ViewChannel`
2. Assign to User B
3. User B opens channel → sees "Read-only", no send button
4. Admin gives User B `SendMessages` → send button appears

---

## References

- `context/tasks/gap-analysis-mvp-completion.md` — Item #2
- `context/03-domain-permissions.md` — Permission keys and rules
- `apps/web/src/lib/components/chat/MessageComposer.svelte` — composer
- `apps/web/src/lib/components/chat/MessageItem.svelte` — message item

---

**Created:** 2026-05-05
**Depends on:** Frontend Channel Visibility (base) 🟡
**Estimated effort:** Small (1 session)
**Risk:** Low — prop drilling pattern, no new APIs needed

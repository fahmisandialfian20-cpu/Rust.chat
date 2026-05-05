# Frontend Permission Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Wire backend permissions into the frontend UI so unauthorized actions are visually hidden/disabled, preventing users from seeing controls they cannot use.

**Architecture:** Add `updateMessage`/`deleteMessage` API functions, then thread `permissions` and `currentUserId` props from the channel page through `MessageList` into `MessageItem`. `MessageItem` uses `$derived` to compute `canEdit`/`canDelete` based on ownership + permission key presence. Edit/delete actions call the API directly and update local state.

**Tech Stack:** SvelteKit 5 (runes), TypeScript (zod), Tailwind CSS, lucide-svelte

---

## Prerequisite Check

Current state of files targeted for modification:

| File | Status |
|------|--------|
| `apps/web/src/lib/api/messages.ts` | Missing `updateMessage` and `deleteMessage` |
| `apps/web/src/routes/(app)/spaces/[spaceId]/channels/[channelId]/+page.svelte` | Has `permissions` state, `currentUserId`, needs edit/delete handlers |
| `apps/web/src/lib/components/chat/MessageList.svelte` | Has `readOnly` prop, needs `permissions` + `currentUserId` pass-through |
| `apps/web/src/lib/components/chat/MessageItem.svelte` | Only has `message`, needs `currentUserId` + `permissions` + edit/delete UI |
| `apps/web/src/lib/components/chat/MessageComposer.svelte` | Already uses `disabled`/`disabledReason` props — no changes needed |
| `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte` | Already fetches + passes `permissions` — no changes needed |
| `apps/web/src/lib/components/channels/ChannelList.svelte` | Already uses `permissions.includes('manage_channels')` — no changes needed |

---

### Task 1: Add `updateMessage` and `deleteMessage` to API client

**Files:**
- Modify: `apps/web/src/lib/api/messages.ts`

- [ ] **Step 1: Add `updateMessage` export**

Append after `sendMessage` in `apps/web/src/lib/api/messages.ts`:

```typescript
export async function updateMessage(
  channelId: string,
  messageId: string,
  content: string,
): Promise<Message> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(
    apiUrl(`/api/v1/channels/${channelId}/messages/${messageId}`),
    {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({ content }),
    },
  );

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }

  if (response.status === 204) {
    return await response.json();
  }

  const data: unknown = await response.json();
  const result = MessageSchema.safeParse(data);
  if (!result.success) {
    throw { status: 500, message: 'Invalid response from server' } satisfies ApiError;
  }

  return result.data;
}
```

- [ ] **Step 2: Add `deleteMessage` export**

Append after `updateMessage` in `apps/web/src/lib/api/messages.ts`:

```typescript
export async function deleteMessage(
  channelId: string,
  messageId: string,
): Promise<void> {
  const token = getAccessToken();
  if (!token) {
    throw { status: 401, message: 'Not authenticated' } satisfies ApiError;
  }

  const response = await fetch(
    apiUrl(`/api/v1/channels/${channelId}/messages/${messageId}`),
    {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${token}`,
      },
    },
  );

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw { status: response.status, message: body.message ?? 'Request failed' } satisfies ApiError;
  }
}
```

- [ ] **Step 3: Run TypeScript check**

```bash
cd apps/web; npm run check
```

Expected: No type errors. `updateMessage` and `deleteMessage` are properly exported.

---

### Task 2: Thread `permissions` and `currentUserId` through MessageList

**Files:**
- Modify: `apps/web/src/lib/components/chat/MessageList.svelte`

- [ ] **Step 1: Add `permissions` and `currentUserId` props to MessageList**

Replace the `let { ... } = $props()` block in `apps/web/src/lib/components/chat/MessageList.svelte`:

```svelte
<script lang="ts">
  import { tick } from 'svelte';
  import MessageItem from './MessageItem.svelte';
  import type { Message } from '$lib/schemas/messages';

  let {
    messages = [],
    loading = false,
    hasMore = false,
    loadingMore = false,
    readOnly = false,
    permissions = [],
    currentUserId = '',
    onloadMore,
  }: {
    messages: Message[];
    loading?: boolean;
    hasMore?: boolean;
    loadingMore?: boolean;
    readOnly?: boolean;
    permissions?: string[];
    currentUserId?: string;
    onloadMore?: () => void;
  } = $props();
```

- [ ] **Step 2: Pass `permissions` and `currentUserId` to MessageItem**

Replace the `MessageItem` invocation line:
```svelte
      {#each messages as message (message.id)}
        <MessageItem {message} {permissions} {currentUserId} />
      {/each}
```

- [ ] **Step 3: Run TypeScript check**

```bash
cd apps/web; npm run check
```

Expected: No errors. `MessageItem` will initially complain about unknown props — resolved in Task 3.

---

### Task 3: Add edit/delete buttons with permission checks to MessageItem

**Files:**
- Modify: `apps/web/src/lib/components/chat/MessageItem.svelte`

- [ ] **Step 1: Replace the script block with full permission-aware logic**

Replace the entire `<script lang="ts">` block in `apps/web/src/lib/components/chat/MessageItem.svelte`:

```svelte
<script lang="ts">
  import { Pencil, Trash2, X, Check } from 'lucide-svelte';
  import type { Message } from '$lib/schemas/messages';
  import { getPresence } from '$lib/stores/presence.svelte';
  import PresenceDot from '$lib/components/presence/PresenceDot.svelte';
  import { updateMessage, deleteMessage } from '$lib/api/messages';

  let {
    message,
    permissions = [],
    currentUserId = '',
  }: {
    message: Message;
    permissions?: string[];
    currentUserId?: string;
  } = $props();

  let editing = $state(false);
  let editText = $state(message.content);
  let deleting = $state(false);

  let presenceStatus = $derived(getPresence(message.author_user_id));
  let isOwnMessage = $derived(message.author_user_id === currentUserId);
  let canEdit = $derived(
    (isOwnMessage && permissions.includes('edit_own_message')) ||
    permissions.includes('edit_any_message'),
  );
  let canDelete = $derived(
    (isOwnMessage && permissions.includes('delete_own_message')) ||
    permissions.includes('delete_any_message'),
  );

  function formatTime(iso: string): string {
    const date = new Date(iso);
    const now = new Date();
    const isToday = date.toDateString() === now.toDateString();
    if (isToday) {
      return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
    }
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  let timestamp = $derived(formatTime(message.created_at));
  let authorLabel = $derived(message.author_user_id.slice(0, 8));

  function startEdit() {
    editText = message.content;
    editing = true;
  }

  function cancelEdit() {
    editing = false;
    editText = message.content;
  }

  async function saveEdit() {
    const trimmed = editText.trim();
    if (trimmed.length === 0 || trimmed === message.content) {
      editing = false;
      return;
    }
    try {
      const updated = await updateMessage(message.channel_id, message.id, trimmed);
      message.content = updated.content;
      message.edited_at = updated.edited_at;
    } catch {
      // silently fail — backend will reject if unauthorized
    }
    editing = false;
  }

  async function handleDelete() {
    if (deleting) return;
    deleting = true;
    try {
      await deleteMessage(message.channel_id, message.id);
      message.content = '';
      message.deleted_at = new Date().toISOString();
    } catch {
      // silently fail
    } finally {
      deleting = false;
    }
  }

  function handleEditKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      saveEdit();
    }
    if (e.key === 'Escape') {
      cancelEdit();
    }
  }
</script>
```

- [ ] **Step 2: Replace the template with permission-aware rendering**

Replace the entire template (everything after `</script>`) in `apps/web/src/lib/components/chat/MessageItem.svelte`:

```svelte
<div class="group flex gap-3 px-4 py-2 transition hover:bg-white/[0.02]" role="listitem">
  <div class="flex size-8 shrink-0 items-center justify-center rounded-full bg-brand-500/20 text-xs font-semibold text-brand-300">
    {authorLabel.slice(0, 2).toUpperCase()}
  </div>
  <div class="min-w-0 flex-1">
    <div class="flex items-baseline gap-2">
      <span class="flex items-center gap-1.5">
        <PresenceDot status={presenceStatus} size="sm" />
        <span class="text-sm font-medium text-rc-200">{authorLabel}</span>
      </span>
      <time class="text-xs text-rc-500" datetime={message.created_at}>{timestamp}</time>
      {#if message.edited_at}
        <span class="text-[10px] text-rc-600">(edited)</span>
      {/if}
      {#if (canEdit || canDelete) && !editing && !message.deleted_at}
        <div class="ml-auto flex items-center gap-1 opacity-0 transition group-hover:opacity-100">
          {#if canEdit}
            <button
              onclick={startEdit}
              aria-label="Edit message"
              class="flex size-6 items-center justify-center rounded text-rc-400 transition hover:bg-white/10 hover:text-rc-200"
            >
              <Pencil class="size-3.5" aria-hidden="true" />
            </button>
          {/if}
          {#if canDelete}
            <button
              onclick={handleDelete}
              disabled={deleting}
              aria-label="Delete message"
              class="flex size-6 items-center justify-center rounded text-rc-400 transition hover:bg-red-500/20 hover:text-red-400 disabled:opacity-50"
            >
              <Trash2 class="size-3.5" aria-hidden="true" />
            </button>
          {/if}
        </div>
      {/if}
    </div>

    {#if message.deleted_at}
      <p class="mt-0.5 text-xs italic text-rc-600">Message deleted</p>
    {:else if editing}
      <div class="mt-1">
        <textarea
          bind:value={editText}
          onkeydown={handleEditKeydown}
          rows="2"
          class="w-full resize-none rounded-lg border border-white/10 bg-rc-900 px-3 py-2 text-sm text-rc-100 placeholder-rc-500 transition focus:border-brand-500/50 focus:outline-none focus:ring-1 focus:ring-brand-500/30"
        ></textarea>
        <div class="mt-1 flex gap-2">
          <button
            onclick={saveEdit}
            class="flex items-center gap-1 rounded-md bg-brand-600 px-3 py-1 text-xs font-medium text-white transition hover:bg-brand-500"
          >
            <Check class="size-3" aria-hidden="true" />
            Save
          </button>
          <button
            onclick={cancelEdit}
            class="flex items-center gap-1 rounded-md bg-white/10 px-3 py-1 text-xs font-medium text-rc-200 transition hover:bg-white/20"
          >
            <X class="size-3" aria-hidden="true" />
            Cancel
          </button>
        </div>
      </div>
    {:else}
      <p class="mt-0.5 whitespace-pre-wrap break-words text-sm text-rc-100">{message.content}</p>
    {/if}
  </div>
</div>
```

- [ ] **Step 3: Run TypeScript check**

```bash
cd apps/web; npm run check
```

Expected: No errors. `Pencil`, `Trash2`, `X`, `Check` icons resolve from `lucide-svelte`.

---

### Task 4: Wire up channel page to pass new props and handle edit/delete

**Files:**
- Modify: `apps/web/src/routes/(app)/spaces/[spaceId]/channels/[channelId]/+page.svelte`

- [ ] **Step 1: Pass `permissions` and `currentUserId` to `MessageList`**

Find the `MessageList` invocation and add the two new props:

```svelte
    <MessageList
      {messages}
      loading={false}
      {hasMore}
      {loadingMore}
      readOnly={!canSendMessages}
      {permissions}
      currentUserId={currentUserId}
      onloadMore={loadMore}
    />
```

- [ ] **Step 2: Run TypeScript check**

```bash
cd apps/web; npm run check
```

Expected: No errors.

---

### Task 5: Final verification

- [ ] **Step 1: Run full check**

```bash
cd apps/web; npm run check
```

Expected: `svelte-check` exits with 0, no warnings.

- [ ] **Step 2: Run production build**

```bash
cd apps/web; npm run build
```

Expected: `build` completes with no errors, output in `build/` directory.

- [ ] **Step 3: Verify acceptance criteria manually**

| Scenario | Expected behavior |
|----------|-------------------|
| User lacks `send_messages` | MessageList shows "Read-only" badge, composer shows disabled notice |
| User lacks `manage_channels` | No "+" button in ChannelList |
| User owns message + has `edit_own_message` | Edit button appears on hover, opens inline editor |
| User owns message + lacks `edit_own_message` | No edit button |
| User lacks `edit_any_message` | Cannot edit other users' messages |
| User owns message + has `delete_own_message` | Delete button appears on hover |
| User lacks `delete_any_message` | No delete button on other users' messages |
| User clicks Delete | Message marked as deleted in UI |

---

## Summary of All Changes

| File | Change |
|------|--------|
| `apps/web/src/lib/api/messages.ts` | Add `updateMessage()` and `deleteMessage()` exports |
| `apps/web/src/lib/components/chat/MessageList.svelte` | Add `permissions` + `currentUserId` props, pass to `MessageItem` |
| `apps/web/src/lib/components/chat/MessageItem.svelte` | Add `permissions` + `currentUserId` props, edit/delete buttons, inline editor, permission-derived visibility |
| `apps/web/src/routes/(app)/spaces/[spaceId]/channels/[channelId]/+page.svelte` | Pass `permissions` and `currentUserId` to `MessageList` |

Files confirmed **no change needed** (already correct):
- `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte`
- `apps/web/src/lib/components/chat/MessageComposer.svelte`
- `apps/web/src/lib/components/channels/ChannelList.svelte`

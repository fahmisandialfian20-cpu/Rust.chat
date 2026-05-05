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

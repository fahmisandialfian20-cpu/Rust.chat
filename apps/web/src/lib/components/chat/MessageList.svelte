<script lang="ts">
  import { tick } from 'svelte';
  import MessageItem from './MessageItem.svelte';
  import type { Message } from '$lib/schemas/messages';

  let {
    messages = [],
    loading = false,
    hasMore = false,
    loadingMore = false,
    onloadMore,
  }: {
    messages: Message[];
    loading?: boolean;
    hasMore?: boolean;
    loadingMore?: boolean;
    onloadMore?: () => void;
  } = $props();

  let listEl = $state<HTMLDivElement | null>(null);
  let autoScrolled = $state(false);

  $effect(() => {
    if (messages.length > 0 && !autoScrolled) {
      tick().then(() => {
        if (listEl) {
          listEl.scrollTop = listEl.scrollHeight;
        }
        autoScrolled = true;
      });
    }
  });
</script>

<div bind:this={listEl} class="flex flex-1 flex-col overflow-y-auto" role="log" aria-label="Messages" aria-live="polite">
  {#if loading}
    <div class="flex flex-1 flex-col items-center justify-center gap-4 px-4">
      {#each Array(5) as _, i}
        <div class="flex w-full max-w-lg gap-3 px-4">
          <div class="size-8 shrink-0 animate-pulse rounded-full bg-rc-800"></div>
          <div class="min-w-0 flex-1 space-y-2">
            <div class="h-3 w-24 animate-pulse rounded bg-rc-800"></div>
            <div class="h-4 w-full animate-pulse rounded bg-rc-800"></div>
          </div>
        </div>
      {/each}
    </div>
  {:else if messages.length === 0}
    <div class="flex flex-1 items-center justify-center p-8">
      <p class="text-sm text-rc-500">No messages yet</p>
    </div>
  {:else}
    <div class="flex flex-col">
      {#if hasMore}
        <div class="flex justify-center px-4 py-3">
          <button
            onclick={onloadMore}
            disabled={loadingMore}
            class="rounded-lg border border-white/10 px-4 py-1.5 text-xs font-medium text-rc-300 transition hover:bg-white/5 hover:text-white disabled:opacity-50"
          >
            {loadingMore ? 'Loading...' : 'Load older'}
          </button>
        </div>
      {/if}

      {#each messages as message (message.id)}
        <MessageItem {message} />
      {/each}
    </div>
  {/if}
</div>

<script lang="ts">
  import { page } from '$app/state';
  import { MessageSquare, Mic, Video } from 'lucide-svelte';
  import type { Channel } from '$lib/schemas/channels';

  let { channels = [] }: { channels: Channel[] } = $props();

  let sorted = $derived([...channels].sort((a, b) => a.position - b.position));
  let activeChannelId = $derived((page.params as Record<string, string>).channelId);
</script>

{#if sorted.length === 0}
  <div class="flex flex-col items-center justify-center px-4 py-12 text-center">
    <p class="text-sm text-rc-500">No visible channels</p>
  </div>
{:else}
  <nav class="space-y-0.5 px-2 py-2" aria-label="Channel list">
    {#each sorted as channel (channel.id)}
      <a
        href="/spaces/{channel.space_id}/channels/{channel.id}"
        aria-label="{channel.name}, {channel.kind} channel"
        class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm transition
          {activeChannelId === channel.id
            ? 'bg-brand-500/10 text-brand-200'
            : 'text-rc-400 hover:bg-white/5 hover:text-rc-200'}"
      >
        {#if channel.kind === 'Voice'}
          <Mic class="size-4 shrink-0 text-rc-500" aria-hidden="true" />
        {:else if channel.kind === 'Video'}
          <Video class="size-4 shrink-0 text-rc-500" aria-hidden="true" />
        {:else}
          <MessageSquare class="size-4 shrink-0 text-rc-500" aria-hidden="true" />
        {/if}
        <span class="truncate"># {channel.name}</span>
      </a>
    {/each}
  </nav>
{/if}

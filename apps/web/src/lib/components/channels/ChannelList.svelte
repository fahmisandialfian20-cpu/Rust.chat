<script lang="ts">
  import { page } from '$app/state';
  import { MessageSquare, Mic, Video } from 'lucide-svelte';
  import type { Channel } from '$lib/schemas/channels';

  let { channels = [], permissions = [] }: { channels: Channel[]; permissions: string[] } = $props();

  let sorted = $derived([...channels].sort((a, b) => a.position - b.position));
  let activeChannelId = $derived((page.params as Record<string, string>).channelId);
  let canManageChannels = $derived(permissions.includes('manage_channels'));
</script>

{#if sorted.length === 0}
  <div class="flex flex-col items-center justify-center px-4 py-12 text-center">
    {#if canManageChannels}
      <p class="text-sm text-rc-500">No channels yet.</p>
      <p class="mt-1 text-xs text-rc-600">Create your first channel to get started.</p>
    {:else}
      <p class="text-sm text-rc-500">No visible channels</p>
      <p class="mt-1 text-xs text-rc-600">Ask an admin to grant you access.</p>
    {/if}
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
    {#if canManageChannels}
      <div class="px-2 pt-2">
        <a
          href="/spaces/{page.params.spaceId}/admin/channels"
          class="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm text-rc-400 transition hover:bg-white/5 hover:text-rc-200"
        >
          <span class="text-lg leading-none">+</span>
          <span>Add Channel</span>
        </a>
      </div>
    {/if}
  </nav>
{/if}

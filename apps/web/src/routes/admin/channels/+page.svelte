<script lang="ts">
  import { onMount } from 'svelte';
  import { listSpaces } from '$lib/api/spaces';
  import { listChannels } from '$lib/api/channels';
  import type { Space } from '$lib/schemas/spaces';
  import type { Channel } from '$lib/schemas/channels';
  import ChannelSettingsForm from '$lib/components/admin/ChannelSettingsForm.svelte';

  let spaces = $state<Space[]>([]);
  let channels = $state<Channel[]>([]);
  let selectedSpaceId = $state<string>('');
  let selectedChannelId = $state<string>('');
  let loadingSpaces = $state(true);
  let loadingChannels = $state(false);
  let error = $state<string | null>(null);

  const selectedChannel = $derived(
    channels.find((c) => c.id === selectedChannelId) ?? null
  );

  onMount(() => {
    loadSpaces();
  });

  async function loadSpaces() {
    loadingSpaces = true;
    error = null;
    try {
      spaces = await listSpaces();
    } catch (e) {
      error = e instanceof Object && 'message' in e ? (e as { message: string }).message : 'Failed to load spaces';
    } finally {
      loadingSpaces = false;
    }
  }

  async function handleSpaceChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    selectedSpaceId = target.value;
    selectedChannelId = '';
    channels = [];
    if (selectedSpaceId) {
      await loadChannels();
    }
  }

  async function loadChannels() {
    if (!selectedSpaceId) return;
    loadingChannels = true;
    error = null;
    try {
      channels = await listChannels(selectedSpaceId);
    } catch (e) {
      error = e instanceof Object && 'message' in e ? (e as { message: string }).message : 'Failed to load channels';
    } finally {
      loadingChannels = false;
    }
  }

  function handleChannelChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    selectedChannelId = target.value;
  }

  function handleSaveSuccess() {
    error = null;
  }
</script>

<svelte:head>
  <title>Admin &mdash; Channel Settings &mdash; Rust.chat</title>
</svelte:head>

<div class="mx-auto max-w-3xl space-y-8 px-6 py-8">
  <div class="space-y-2">
    <h1 class="text-2xl font-bold text-white">Channel Settings</h1>
    <p class="text-sm text-rc-300">
      Manage channel metadata and feature flags.
    </p>
  </div>

  <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
    <div class="space-y-2">
      <label for="space-select" class="text-sm font-medium text-rc-200">
        Space
      </label>
      {#if loadingSpaces}
        <div class="rounded-lg border border-white/10 bg-rc-900 px-4 py-3 text-sm text-rc-400">
          Loading spaces...
        </div>
      {:else}
        <select
          id="space-select"
          value={selectedSpaceId}
          onchange={handleSpaceChange}
          class="w-full rounded-lg border border-white/10 bg-rc-900 px-4 py-2.5 text-sm text-white focus:border-brand-400 focus:outline-none focus:ring-1 focus:ring-brand-400"
        >
          <option value="">Select a space</option>
          {#each spaces as space}
            <option value={space.id}>{space.name}</option>
          {/each}
        </select>
      {/if}
    </div>

    <div class="space-y-2">
      <label for="channel-select" class="text-sm font-medium text-rc-200">
        Channel
      </label>
      {#if !selectedSpaceId}
        <div class="rounded-lg border border-white/10 bg-rc-900 px-4 py-3 text-sm text-rc-400">
          Select a space first
        </div>
      {:else if loadingChannels}
        <div class="rounded-lg border border-white/10 bg-rc-900 px-4 py-3 text-sm text-rc-400">
          Loading channels...
        </div>
      {:else if channels.length === 0}
        <div class="rounded-lg border border-white/10 bg-rc-900 px-4 py-3 text-sm text-rc-400">
          No channels in this space
        </div>
      {:else}
        <select
          id="channel-select"
          value={selectedChannelId}
          onchange={handleChannelChange}
          class="w-full rounded-lg border border-white/10 bg-rc-900 px-4 py-2.5 text-sm text-white focus:border-brand-400 focus:outline-none focus:ring-1 focus:ring-brand-400"
        >
          <option value="">Select a channel</option>
          {#each channels as channel}
            <option value={channel.id}>#{channel.name}</option>
          {/each}
        </select>
      {/if}
    </div>
  </div>

  {#if error}
    <div class="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300" role="alert">
      {error}
    </div>
  {/if}

  <div class="rounded-xl border border-white/10 bg-rc-900/50 p-6">
    <ChannelSettingsForm
      spaceId={selectedSpaceId}
      channel={selectedChannel}
      onSave={handleSaveSuccess}
    />
  </div>
</div>

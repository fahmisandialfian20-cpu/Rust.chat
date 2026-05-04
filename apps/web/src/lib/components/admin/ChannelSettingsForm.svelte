<script lang="ts">
  import type { Channel } from '$lib/schemas/channels';
  import type { FeatureFlags } from '$lib/api/channels';
  import { updateChannel, updateChannelFlags, getChannelFlags } from '$lib/api/channels';
  import FeatureFlagToggles from './FeatureFlagToggles.svelte';

  let {
    spaceId,
    channel,
    onSave = () => {},
  }: {
    spaceId: string;
    channel: Channel | null;
    onSave?: () => void;
  } = $props();

  let name = $state('');
  let topic = $state('');
  let visibility = $state<'Public' | 'Private'>('Public');
  let flags = $state<FeatureFlags>({
    text_enabled: true,
    file_upload_enabled: true,
    voice_group_enabled: true,
    video_group_enabled: true,
    threads_enabled: true,
    reactions_enabled: true,
  });
  let saving = $state(false);
  let loadingFlags = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    if (channel) {
      name = channel.name;
      topic = channel.topic ?? '';
      visibility = channel.visibility;
    }
  });

  $effect(() => {
    if (channel && spaceId) {
      loadFlags();
    }
  });

  async function loadFlags() {
    if (!channel || !spaceId) return;
    loadingFlags = true;
    error = null;
    try {
      flags = await getChannelFlags(spaceId, channel.id);
    } catch (e) {
      error = e instanceof Object && 'message' in e ? (e as { message: string }).message : 'Failed to load feature flags';
    } finally {
      loadingFlags = false;
    }
  }

  const isDirty = $derived(
    channel !== null &&
    (name !== channel.name ||
      topic !== (channel.topic ?? '') ||
      visibility !== channel.visibility)
  );

  function handleToggle(key: keyof FeatureFlags, value: boolean) {
    flags = { ...flags, [key]: value };
  }

  async function handleSave() {
    if (!channel || !spaceId) return;
    saving = true;
    error = null;
    try {
      const trimmedName = name.trim();
      if (!trimmedName) {
        throw new Error('Channel name is required');
      }

      await updateChannel(spaceId, channel.id, {
        name: trimmedName,
        topic: topic || null,
        visibility,
      });

      await updateChannelFlags(spaceId, channel.id, flags);

      onSave();
    } catch (e) {
      error = e instanceof Object && 'message' in e ? (e as { message: string }).message : 'Failed to save channel settings';
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-6">
  {#if !channel}
    <div class="rounded-lg border border-white/10 bg-rc-900/50 px-5 py-8 text-center">
      <p class="text-sm text-rc-400">Select a channel to edit its settings.</p>
    </div>
  {:else}
    <div class="space-y-2">
      <label for="channel-name" class="text-sm font-medium text-rc-200">
        Channel name
      </label>
      <input
        id="channel-name"
        type="text"
        bind:value={name}
        placeholder="e.g. general"
        class="w-full rounded-lg border border-white/10 bg-rc-900 px-4 py-2.5 text-sm text-white placeholder:text-rc-400 focus:border-brand-400 focus:outline-none focus:ring-1 focus:ring-brand-400"
        maxlength={64}
      />
    </div>

    <div class="space-y-2">
      <label for="channel-topic" class="text-sm font-medium text-rc-200">
        Topic
      </label>
      <textarea
        id="channel-topic"
        bind:value={topic}
        placeholder="Optional channel topic"
        class="w-full rounded-lg border border-white/10 bg-rc-900 px-4 py-2.5 text-sm text-white placeholder:text-rc-400 focus:border-brand-400 focus:outline-none focus:ring-1 focus:ring-brand-400"
        maxlength={512}
        rows={3}
      ></textarea>
    </div>

    <div class="space-y-2">
      <label for="channel-visibility" class="text-sm font-medium text-rc-200">
        Visibility
      </label>
      <select
        id="channel-visibility"
        bind:value={visibility}
        class="w-full rounded-lg border border-white/10 bg-rc-900 px-4 py-2.5 text-sm text-white focus:border-brand-400 focus:outline-none focus:ring-1 focus:ring-brand-400"
      >
        <option value="Public">Public</option>
        <option value="Private">Private</option>
      </select>
    </div>

    <div class="space-y-2">
      <h3 class="text-sm font-medium text-rc-200">Feature flags</h3>
      {#if loadingFlags}
        <div class="rounded-lg border border-white/10 bg-rc-900 px-4 py-3 text-sm text-rc-400">
          Loading feature flags...
        </div>
      {:else}
        <FeatureFlagToggles {flags} onToggle={handleToggle} />
      {/if}
    </div>
  {/if}

  {#if error}
    <div class="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300" role="alert">
      {error}
    </div>
  {/if}

  {#if channel}
    <div class="flex items-center justify-between border-t border-white/10 pt-4">
      <div class="text-sm text-rc-400">
        {channel.name}
      </div>
      <button
        onclick={handleSave}
        disabled={!channel || saving || loadingFlags}
        class="inline-flex items-center gap-2 rounded-lg bg-brand-600 px-5 py-2.5 text-sm font-medium text-white transition hover:bg-brand-500 disabled:cursor-not-allowed disabled:opacity-50"
      >
        {saving ? 'Saving...' : 'Save changes'}
      </button>
    </div>
  {/if}
</div>

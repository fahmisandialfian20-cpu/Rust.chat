<script lang="ts">
  import type { FeatureFlags } from '$lib/api/channels';

  const FLAG_LABELS: Record<keyof FeatureFlags, string> = {
    text_enabled: 'Text messages',
    file_upload_enabled: 'File upload',
    voice_group_enabled: 'Voice channels',
    video_group_enabled: 'Video channels',
    threads_enabled: 'Threads',
    reactions_enabled: 'Reactions',
  };

  const FLAG_KEYS = Object.keys(FLAG_LABELS) as (keyof FeatureFlags)[];

  let {
    flags,
    onToggle,
  }: {
    flags: FeatureFlags;
    onToggle: (key: keyof FeatureFlags, value: boolean) => void;
  } = $props();
</script>

<div class="space-y-3" role="group" aria-label="Feature flags">
  {#each FLAG_KEYS as key}
    <label
      class="flex cursor-pointer items-center justify-between rounded-lg border border-white/10 bg-rc-900 px-4 py-3 transition hover:bg-white/5"
    >
      <span class="text-sm font-medium text-rc-200">{FLAG_LABELS[key]}</span>
      <button
        role="switch"
        type="button"
        aria-checked={flags[key]}
        aria-label={FLAG_LABELS[key]}
        onclick={() => onToggle(key, !flags[key])}
        class="relative inline-flex h-6 w-11 shrink-0 items-center rounded-full border border-white/10 transition-colors {flags[key] ? 'bg-brand-500' : 'bg-rc-700'}"
      >
        <span
          class="inline-block size-4 rounded-full bg-white shadow transition-transform {flags[key] ? 'translate-x-[22px]' : 'translate-x-[3px]'}"
        ></span>
      </button>
    </label>
  {/each}
</div>

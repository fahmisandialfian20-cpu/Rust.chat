<script lang="ts">
  import {
    getThemePreferences,
    setMode,
    setAccent,
    setDensity,
    setMessageDisplay,
    resetToDefaults,
  } from '$lib/stores/theme.svelte';
  import {
    MODE_OPTIONS,
    ACCENT_OPTIONS,
    DENSITY_OPTIONS,
    MESSAGE_DISPLAY_OPTIONS,
    ACCENT_LABELS,
  } from '$lib/schemas/theme';

  const ACCENT_SWATCHES: Record<string, string> = {
    purple: 'oklch(0.585 0.233 277.117)',
    blue: 'oklch(0.58 0.22 250)',
    green: 'oklch(0.58 0.18 150)',
    orange: 'oklch(0.68 0.18 50)',
    pink: 'oklch(0.62 0.2 340)',
  };

  const prefs = $derived(getThemePreferences());
</script>

<div class="space-y-8">
  <fieldset class="space-y-3">
    <legend class="text-sm font-semibold text-white">Mode</legend>
    <div class="flex flex-wrap gap-3">
      {#each MODE_OPTIONS as mode}
        <label
          class="flex cursor-pointer items-center gap-2 rounded-lg border px-4 py-2.5 text-sm transition
            {prefs.mode === mode
              ? 'border-brand-500/50 bg-brand-500/10 text-brand-200'
              : 'border-white/10 bg-white/[0.03] text-rc-300 hover:border-white/20'}"
        >
          <input
            type="radio"
            name="mode"
            value={mode}
            checked={prefs.mode === mode}
            onchange={() => setMode(mode)}
            class="sr-only"
          />
          <span class="size-2 rounded-full {prefs.mode === mode ? 'bg-brand-400' : 'bg-rc-600'}" aria-hidden="true"></span>
          {mode.charAt(0).toUpperCase() + mode.slice(1)}
        </label>
      {/each}
    </div>
  </fieldset>

  <fieldset class="space-y-3">
    <legend class="text-sm font-semibold text-white">Accent color</legend>
    <div class="flex flex-wrap gap-3">
      {#each ACCENT_OPTIONS as accent}
        <label
          class="flex cursor-pointer items-center gap-2 rounded-lg border px-4 py-2.5 text-sm transition
            {prefs.accent === accent
              ? 'border-brand-500/50 bg-brand-500/10 text-brand-200'
              : 'border-white/10 bg-white/[0.03] text-rc-300 hover:border-white/20'}"
        >
          <input
            type="radio"
            name="accent"
            value={accent}
            checked={prefs.accent === accent}
            onchange={() => setAccent(accent)}
            class="sr-only"
          />
          <span
            class="size-3.5 rounded-full border border-white/20"
            style="background: {ACCENT_SWATCHES[accent]}"
            aria-hidden="true"
          ></span>
          {ACCENT_LABELS[accent]}
        </label>
      {/each}
    </div>
  </fieldset>

  <fieldset class="space-y-3">
    <legend class="text-sm font-semibold text-white">Density</legend>
    <div class="flex flex-wrap gap-3">
      {#each DENSITY_OPTIONS as density}
        <label
          class="flex cursor-pointer items-center gap-2 rounded-lg border px-4 py-2.5 text-sm transition
            {prefs.density === density
              ? 'border-brand-500/50 bg-brand-500/10 text-brand-200'
              : 'border-white/10 bg-white/[0.03] text-rc-300 hover:border-white/20'}"
        >
          <input
            type="radio"
            name="density"
            value={density}
            checked={prefs.density === density}
            onchange={() => setDensity(density)}
            class="sr-only"
          />
          <span class="size-2 rounded-full {prefs.density === density ? 'bg-brand-400' : 'bg-rc-600'}" aria-hidden="true"></span>
          {density.charAt(0).toUpperCase() + density.slice(1)}
        </label>
      {/each}
    </div>
  </fieldset>

  <fieldset class="space-y-3">
    <legend class="text-sm font-semibold text-white">Message display</legend>
    <div class="flex flex-wrap gap-3">
      {#each MESSAGE_DISPLAY_OPTIONS as display}
        <label
          class="flex cursor-pointer items-center gap-2 rounded-lg border px-4 py-2.5 text-sm transition
            {prefs.message_display === display
              ? 'border-brand-500/50 bg-brand-500/10 text-brand-200'
              : 'border-white/10 bg-white/[0.03] text-rc-300 hover:border-white/20'}"
        >
          <input
            type="radio"
            name="message_display"
            value={display}
            checked={prefs.message_display === display}
            onchange={() => setMessageDisplay(display)}
            class="sr-only"
          />
          <span class="size-2 rounded-full {prefs.message_display === display ? 'bg-brand-400' : 'bg-rc-600'}" aria-hidden="true"></span>
          {display.charAt(0).toUpperCase() + display.slice(1)}
        </label>
      {/each}
    </div>
  </fieldset>

  <div class="flex items-center gap-4 border-t border-white/10 pt-6">
    <button
      disabled
      class="inline-flex items-center gap-2 rounded-lg bg-brand-500/30 px-5 py-2.5 text-sm font-medium text-rc-400"
      title="Save is not available because the server does not support theme preference endpoints yet."
    >
      Save preferences
    </button>

    <button
      onclick={resetToDefaults}
      class="inline-flex items-center gap-2 rounded-lg border border-white/10 px-5 py-2.5 text-sm font-medium text-rc-300 transition hover:bg-white/5 hover:text-white"
    >
      Reset to defaults
    </button>
  </div>
</div>

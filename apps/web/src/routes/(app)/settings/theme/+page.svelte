<script lang="ts">
  import { ArrowLeft } from 'lucide-svelte';
  import ThemeSettingsForm from '$lib/components/settings/ThemeSettingsForm.svelte';
</script>

<svelte:head>
  <title>Theme settings - Rust.chat</title>
</svelte:head>

<div class="mx-auto max-w-2xl px-4 py-8 sm:px-6 lg:px-8">
  <a
    href="/lobby"
    class="mb-6 inline-flex items-center gap-1.5 text-sm text-rc-400 transition hover:text-white"
  >
    <ArrowLeft class="size-4" aria-hidden="true" />
    Back to lobby
  </a>

  <div class="mb-8 space-y-2">
    <h1 class="text-2xl font-semibold text-white">Theme settings</h1>
    <p class="text-sm text-rc-400">
      Theme settings are previewed locally. Server persistence is not yet available.
    </p>
  </div>

  <div class="rounded-card border border-white/10 bg-white/[0.03] p-6">
    <ThemeSettingsForm />
  </div>
</div>

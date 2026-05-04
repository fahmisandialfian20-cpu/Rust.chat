<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { ArrowRight, ShieldCheck, Server, Smartphone } from 'lucide-svelte';
  import { getAccessToken } from '$lib/stores/auth.svelte';

  onMount(() => {
    if (getAccessToken()) {
      goto('/lobby');
    }
  });

  const principles = [
    {
      title: 'Server-owned permissions',
      description: 'UI state is never trusted as authority. Channel access and actions are validated by the Rust backend.',
      icon: ShieldCheck
    },
    {
      title: 'Self-hosted core',
      description: 'The deployer is the Hoster and keeps highest authority over spaces, roles, channels, and storage.',
      icon: Server
    },
    {
      title: 'Multi-client ready',
      description: 'The Svelte UI is the reference web client and the base for the Tauri desktop shell.',
      icon: Smartphone
    }
  ];
</script>

<svelte:head>
  <title>Rust.chat</title>
</svelte:head>

<main class="min-h-screen px-6 py-8 text-rc-50 sm:px-10 lg:px-12">
  <section class="mx-auto flex min-h-[calc(100vh-4rem)] max-w-6xl flex-col justify-center gap-12">
    <div class="max-w-3xl space-y-7">
      <p class="inline-flex rounded-full border border-brand-300/20 bg-brand-500/10 px-4 py-2 text-sm font-medium text-brand-100 shadow-soft">
        Phase 9 · Frontend MVP foundation
      </p>

      <div class="space-y-5">
        <h1 class="text-5xl font-semibold tracking-tight text-white sm:text-6xl lg:text-7xl">
          Self-hosted chat with permissions at the core.
        </h1>
        <p class="max-w-2xl text-lg leading-8 text-rc-200">
          Rust.chat is a Discord/Telegram-like chat app where the Hoster controls spaces,
          roles, channels, and feature flags from a backend-first authority model.
        </p>
      </div>

      <div class="flex flex-col gap-3 sm:flex-row">
        <a
          href="/bootstrap"
          class="inline-flex items-center justify-center gap-2 rounded-full bg-brand-500 px-5 py-3 text-sm font-semibold text-white transition hover:bg-brand-400 focus:outline-none focus:ring-2 focus:ring-brand-300 focus:ring-offset-2 focus:ring-offset-rc-950"
        >
          Bootstrap Hoster
          <ArrowRight class="size-4" aria-hidden="true" />
        </a>
        <a
          href="/login"
          class="inline-flex items-center justify-center rounded-full border border-white/10 bg-white/5 px-5 py-3 text-sm font-semibold text-rc-100 transition hover:bg-white/10 focus:outline-none focus:ring-2 focus:ring-brand-300 focus:ring-offset-2 focus:ring-offset-rc-950"
        >
          Login
        </a>
      </div>
    </div>

    <div class="grid gap-4 md:grid-cols-3">
      {#each principles as item}
        <article class="rounded-card border border-white/10 bg-white/[0.045] p-6 shadow-soft backdrop-blur">
          <svelte:component this={item.icon} class="mb-5 size-6 text-brand-200" aria-hidden="true" />
          <h2 class="text-lg font-semibold text-white">{item.title}</h2>
          <p class="mt-3 text-sm leading-6 text-rc-300">{item.description}</p>
        </article>
      {/each}
    </div>
  </section>
</main>

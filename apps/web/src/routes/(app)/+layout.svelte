<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { LogOut, Shield, Menu, X } from 'lucide-svelte';
  import { getAccessToken, getUser, clearAuth } from '$lib/stores/auth.svelte';
  import { realtime } from '$lib/stores/realtime.svelte';
  import ReconnectBanner from '$lib/components/realtime/ReconnectBanner.svelte';

  let { children } = $props();
  let sidebarOpen = $state(false);

  onMount(() => {
    const token = getAccessToken();
    if (!token) {
      goto('/login');
      return;
    }
    realtime.connect(token);
  });

  function handleLogout() {
    realtime.disconnect();
    clearAuth();
    goto('/login');
  }

  const user = $derived(getUser() as { username?: string } | null);
</script>

<svelte:head>
  <title>Rust.chat</title>
</svelte:head>

<div class="flex h-screen bg-rc-950 text-rc-100">
  <button
    class="fixed left-4 top-4 z-50 flex size-10 items-center justify-center rounded-lg border border-white/10 bg-rc-900 md:hidden"
    onclick={() => (sidebarOpen = !sidebarOpen)}
    aria-label={sidebarOpen ? 'Close navigation' : 'Open navigation'}
  >
    {#if sidebarOpen}
      <X class="size-5" aria-hidden="true" />
    {:else}
      <Menu class="size-5" aria-hidden="true" />
    {/if}
  </button>

  <aside
    class="flex w-64 flex-col border-r border-white/10 bg-rc-950 transition-transform md:relative md:translate-x-0
      {sidebarOpen ? 'translate-x-0' : '-translate-x-full'} md:flex"
    role="navigation"
    aria-label="App navigation"
  >
    <div class="flex items-center gap-3 border-b border-white/10 px-5 py-5">
      <div class="flex size-9 items-center justify-center rounded-full bg-brand-500/20">
        <Shield class="size-5 text-brand-300" aria-hidden="true" />
      </div>
      <span class="text-base font-semibold text-white">Rust.chat</span>
    </div>

    <nav class="flex-1 space-y-1 overflow-y-auto px-3 py-4">
      <a
        href="/lobby"
        class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-rc-200 transition hover:bg-white/5 hover:text-white"
      >
        Spaces
      </a>
      <a
        href="/admin/roles"
        class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-rc-200 transition hover:bg-white/5 hover:text-white"
      >
        Admin
      </a>
      <a
        href="/settings/theme"
        class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-rc-200 transition hover:bg-white/5 hover:text-white"
      >
        Settings
      </a>
    </nav>

    <div class="border-t border-white/10 px-4 py-4">
      <div class="mb-3 flex items-center gap-3">
        <div class="flex size-8 items-center justify-center rounded-full bg-brand-500/30 text-xs font-semibold text-brand-200">
          {user?.username?.charAt(0)?.toUpperCase() ?? '?'}
        </div>
        <div class="min-w-0 flex-1">
          <p class="truncate text-sm font-medium text-white">{user?.username ?? 'User'}</p>
        </div>
      </div>
      <button
        onclick={handleLogout}
        class="inline-flex w-full items-center justify-center gap-2 rounded-lg border border-white/10 px-3 py-2 text-sm text-rc-300 transition hover:bg-white/5 hover:text-white"
      >
        <LogOut class="size-4" aria-hidden="true" />
        Sign out
      </button>
    </div>
  </aside>

  {#if sidebarOpen}
    <div
      class="fixed inset-0 z-40 bg-black/50 md:hidden"
      role="presentation"
      onclick={() => (sidebarOpen = false)}
    ></div>
  {/if}

  <main class="relative flex flex-1 flex-col overflow-y-auto">
    <ReconnectBanner />
    <div class="flex-1">
      {@render children()}
    </div>
  </main>
</div>

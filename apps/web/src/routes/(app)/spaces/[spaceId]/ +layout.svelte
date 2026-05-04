<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { LoaderCircle, AlertCircle, Hash } from 'lucide-svelte';
  import { listVisibleChannels } from '$lib/api/channels';
  import { getSpace } from '$lib/api/spaces';
  import ChannelList from '$lib/components/channels/ChannelList.svelte';
  import type { Channel } from '$lib/schemas/channels';
  import { getAccessToken } from '$lib/stores/auth.svelte';

  let { children } = $props();

  type LoadState = 'loading' | 'loaded' | 'error' | 'forbidden' | 'notfound';

  let viewState: LoadState = $state('loading');
  let channels: Channel[] = $state([]);
  let spaceName = $state('');
  let errorMessage = $state('');

  let spaceId = $derived(page.params.spaceId as string);

  onMount(() => {
    const token = getAccessToken();
    if (!token) {
      goto('/login');
      return;
    }
    load();
  });

  async function load() {
    viewState = 'loading';
    try {
      const [channelResult, spaceResult] = await Promise.all([
        listVisibleChannels(spaceId),
        getSpace(spaceId).catch(() => null),
      ]);
      channels = channelResult;
      spaceName = spaceResult?.name ?? 'Channels';
      viewState = 'loaded';
    } catch (err: unknown) {
      const e = err as { status?: number; message?: string };
      if (e.status === 401) {
        goto('/login');
        return;
      }
      if (e.status === 403) {
        viewState = 'forbidden';
        errorMessage = 'You do not have permission to access this space.';
        return;
      }
      if (e.status === 404) {
        viewState = 'notfound';
        return;
      }
      viewState = 'error';
      errorMessage = 'Something went wrong. Please try again.';
    }
  }
</script>

<svelte:head>
  <title>{spaceName} - Rust.chat</title>
</svelte:head>

<div class="flex h-full">
  <aside class="flex w-60 flex-col border-r border-white/10 bg-rc-950" aria-label="Space sidebar">
    <div class="flex items-center gap-2.5 border-b border-white/10 px-4 py-3.5">
      <Hash class="size-5 text-brand-400" aria-hidden="true" />
      <h1 class="truncate text-base font-semibold text-white">{spaceName}</h1>
    </div>

    {#if viewState === 'loading'}
      <div class="flex flex-1 items-center justify-center" role="status" aria-label="Loading channels">
        <LoaderCircle class="size-5 animate-spin text-rc-500" aria-hidden="true" />
        <span class="sr-only">Loading channels...</span>
      </div>
    {:else if viewState === 'loaded'}
      <div class="flex-1 overflow-y-auto">
        <ChannelList {channels} />
      </div>
    {:else if viewState === 'forbidden'}
      <div class="flex flex-1 items-center justify-center p-4">
        <div role="alert" class="flex items-start gap-3 rounded-lg border border-red-500/30 bg-red-500/10 p-4 text-sm text-red-200">
          <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
          <p>{errorMessage}</p>
        </div>
      </div>
    {:else if viewState === 'notfound'}
      <div class="flex flex-1 items-center justify-center p-4">
        <div role="alert" class="flex items-start gap-3 rounded-lg border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-300">
          <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
          <p>Space not found.</p>
        </div>
      </div>
    {:else if viewState === 'error'}
      <div class="flex flex-1 flex-col items-center justify-center gap-4 p-4">
        <div role="alert" class="flex items-start gap-3 rounded-lg border border-red-500/30 bg-red-500/10 p-4 text-sm text-red-200">
          <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
          <p>{errorMessage}</p>
        </div>
        <button
          onclick={load}
          class="rounded-lg bg-brand-600 px-4 py-2 text-sm font-medium text-white transition hover:bg-brand-500 focus-visible:outline-2 focus-visible:outline-brand-400"
        >
          Retry
        </button>
      </div>
    {/if}
  </aside>

  <main class="flex flex-1 flex-col overflow-y-auto bg-rc-950/50">
    {@render children()}
  </main>
</div>

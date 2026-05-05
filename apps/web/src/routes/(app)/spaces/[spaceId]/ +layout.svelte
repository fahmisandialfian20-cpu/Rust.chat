<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { LoaderCircle, AlertCircle, Hash } from 'lucide-svelte';
  import { listVisibleChannels, getMyPermissions } from '$lib/api/channels';
  import { getSpace } from '$lib/api/spaces';
  import { realtime } from '$lib/stores/realtime';
  import ChannelList from '$lib/components/channels/ChannelList.svelte';
  import type { Channel } from '$lib/schemas/channels';
  import { getAccessToken } from '$lib/stores/auth.svelte';

  let { children } = $props();

  type LoadState = 'loading' | 'loaded' | 'error' | 'forbidden' | 'notfound';

  let viewState: LoadState = $state('loading');
  let channels: Channel[] = $state([]);
  let spaceName = $state('');
  let permissions: string[] = $state([]);
  let errorMessage = $state('');
  let retryCount = $state(0);
  const MAX_RETRIES = 3;

  let spaceId = $derived(page.params.spaceId as string);

  onMount(() => {
    const token = getAccessToken();
    if (!token) {
      goto('/login');
      return;
    }

    realtime.connect(token);
    load();

    const unsubCreated = realtime.subscribe('channel.created', (payload) => {
      const ch = payload as Channel;
      if (ch.space_id === spaceId) {
        channels = [...channels, ch];
      }
    });

    const unsubUpdated = realtime.subscribe('channel.updated', (payload) => {
      const ch = payload as Channel;
      if (ch.space_id === spaceId) {
        channels = channels.map(c => c.id === ch.id ? ch : c);
      }
    });

    const unsubDeleted = realtime.subscribe('channel.deleted', (payload) => {
      const p = payload as { channel_id: string };
      channels = channels.filter(c => c.id !== p.channel_id);
    });

    const unsubVisibilityChanged = realtime.subscribe('channel.visibility_changed', () => {
      listVisibleChannels(spaceId).then(result => channels = result);
    });

    return () => {
      unsubCreated();
      unsubUpdated();
      unsubDeleted();
      unsubVisibilityChanged();
    };
  });

  async function load() {
    viewState = 'loading';
    retryCount = 0;
    await attemptLoad();
  }

  async function attemptLoad(): Promise<void> {
    for (let attempt = 0; attempt <= MAX_RETRIES; attempt++) {
      try {
        const [channelResult, spaceResult] = await Promise.all([
          listVisibleChannels(spaceId),
          getSpace(spaceId).catch(() => null),
        ]);
        channels = channelResult;
        spaceName = spaceResult?.name ?? 'Channels';

        const perms = await getMyPermissions(spaceId).catch(() => []);
        permissions = perms;

        viewState = 'loaded';
        return;
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
        if (attempt < MAX_RETRIES) {
          retryCount = attempt + 1;
          const delay = Math.min(1000 * Math.pow(2, attempt), 8000);
          await new Promise(r => setTimeout(r, delay));
        } else {
          viewState = 'error';
          errorMessage = 'Something went wrong. Please try again.';
        }
      }
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
        <ChannelList {channels} {permissions} />
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

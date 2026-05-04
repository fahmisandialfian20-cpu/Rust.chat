<script lang="ts">
  import { onMount } from 'svelte';
  import { LoaderCircle, AlertCircle, Globe, Lock, Shield } from 'lucide-svelte';
  import { listSpaces } from '$lib/api/spaces';
  import type { Space } from '$lib/schemas/spaces';

  type LoadState = 'loading' | 'loaded' | 'error' | 'empty';

  let viewState: LoadState = $state('loading');
  let spaces: Space[] = $state([]);
  let errorMessage = $state('');

  onMount(async () => {
    viewState = 'loading';
    try {
      const result = await listSpaces();
      if (result.length === 0) {
        viewState = 'empty';
      } else {
        spaces = result;
        viewState = 'loaded';
      }
    } catch (err: unknown) {
      const e = err as { status?: number; message?: string };
      if (e.status === 401 || e.status === 403) {
        errorMessage = 'You do not have permission to view spaces.';
      } else {
        errorMessage = 'Something went wrong. Please try again.';
      }
      viewState = 'error';
    }
  });
</script>

<svelte:head>
  <title>Lobby - Rust.chat</title>
</svelte:head>

<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
  <div class="mb-8 space-y-2">
    <h1 class="text-2xl font-semibold text-white">Spaces</h1>
    <p class="text-sm text-rc-400">
      These are the spaces you can access. Being in the lobby does not mean you can see all channels in a space.
    </p>
  </div>

  {#if viewState === 'loading'}
    <div class="space-y-4" role="status" aria-label="Loading spaces">
      {#each [1, 2, 3] as _i}
        <div class="animate-pulse rounded-card border border-white/10 bg-white/[0.03] p-6">
          <div class="mb-3 h-5 w-48 rounded bg-white/10"></div>
          <div class="h-4 w-full rounded bg-white/5"></div>
        </div>
      {/each}
      <span class="sr-only">Loading spaces...</span>
    </div>
  {:else if viewState === 'error'}
    <div role="alert" class="flex items-start gap-3 rounded-lg border border-red-500/30 bg-red-500/10 p-4 text-sm text-red-200">
      <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
      <p>{errorMessage}</p>
    </div>
  {:else if viewState === 'empty'}
    <div class="flex flex-col items-center justify-center rounded-card border border-dashed border-white/10 px-6 py-16 text-center">
      <Shield class="mb-4 size-10 text-rc-500" aria-hidden="true" />
      <h2 class="text-lg font-semibold text-rc-300">No spaces yet</h2>
      <p class="mt-2 max-w-sm text-sm text-rc-500">
        Spaces will appear here once they are created and you are granted access.
      </p>
    </div>
  {:else if viewState === 'loaded'}
    <div class="space-y-3" role="list" aria-label="Spaces list">
      {#each spaces as space (space.id)}
        <a
          href="/spaces/{space.id}"
          class="group block rounded-card border border-white/10 bg-white/[0.03] p-5 transition hover:border-white/20 hover:bg-white/[0.06]"
        >
          <div class="flex items-start justify-between gap-4">
            <div class="min-w-0 flex-1 space-y-1">
              <h2 class="text-base font-semibold text-white group-hover:text-brand-200">
                {space.name}
              </h2>
              {#if space.description}
                <p class="text-sm leading-6 text-rc-400 line-clamp-2">{space.description}</p>
              {/if}
            </div>
            <span
              class="inline-flex shrink-0 items-center gap-1.5 rounded-full border px-3 py-1 text-xs font-medium {space.visibility === 'Public'
                ? 'border-green-500/20 bg-green-500/10 text-green-300'
                : 'border-amber-500/20 bg-amber-500/10 text-amber-300'}"
            >
              {#if space.visibility === 'Public'}
                <Globe class="size-3" aria-hidden="true" />
              {:else}
                <Lock class="size-3" aria-hidden="true" />
              {/if}
              {space.visibility}
            </span>
          </div>
          <p class="mt-2 text-xs text-rc-600">
            Created {new Date(space.created_at).toLocaleDateString()}
          </p>
        </a>
      {/each}
    </div>

    <p class="mt-6 text-center text-xs text-rc-600">
      Lobby access lists spaces you can see. Channel access within a space may be further restricted.
    </p>
  {/if}
</div>

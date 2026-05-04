<script lang="ts">
  import { getConnectionState, getReconnectAttempt, realtime } from '$lib/stores/realtime.svelte';

  let showConnected = $state(false);
  let conn = $derived(getConnectionState());
  let attempt = $derived(getReconnectAttempt());
  let visible = $derived(conn === 'connecting' || conn === 'reconnecting' || conn === 'error' || showConnected);

  $effect(() => {
    if (conn === 'open') {
      showConnected = true;
      const t = setTimeout(() => { showConnected = false; }, 2000);
      return () => clearTimeout(t);
    }
  });

  function handleRetry() {
    realtime.retry();
  }
</script>

{#if visible}
  <div
    class="fixed left-0 right-0 top-0 z-60 transition-all duration-300 translate-y-0"
    role="status"
    aria-live="polite"
  >
    {#if conn === 'connecting'}
      <div class="flex items-center justify-center gap-2 bg-rc-800/90 px-4 py-2 text-sm text-rc-300 backdrop-blur-sm">
        <span class="inline-block size-2 animate-pulse rounded-full bg-rc-400"></span>
        Connecting...
      </div>
    {:else if conn === 'reconnecting'}
      <div class="flex items-center justify-center gap-2 bg-amber-900/80 px-4 py-2 text-sm text-amber-200 backdrop-blur-sm">
        <span class="inline-block size-2 animate-pulse rounded-full bg-amber-400"></span>
        Reconnecting...
      </div>
    {:else if conn === 'error'}
      <div class="flex items-center justify-center gap-3 bg-red-900/80 px-4 py-2 text-sm text-red-200 backdrop-blur-sm">
        <span class="inline-block size-2 rounded-full bg-red-400"></span>
        Connection lost
        <button
          onclick={handleRetry}
          class="ml-1 rounded-md bg-red-700/60 px-3 py-1 text-xs font-medium text-red-100 transition hover:bg-red-700 focus-visible:outline-2 focus-visible:outline-red-400"
        >
          Retry
        </button>
      </div>
    {:else if showConnected}
      <div class="flex items-center justify-center gap-2 bg-green-900/60 px-4 py-2 text-sm text-green-200 backdrop-blur-sm">
        <span class="inline-block size-2 rounded-full bg-green-400"></span>
        Connected
      </div>
    {/if}
  </div>
{/if}

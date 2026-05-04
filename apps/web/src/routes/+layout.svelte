<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';
  import { realtime } from '$lib/stores/realtime.svelte';
  import { getAccessToken } from '$lib/stores/auth.svelte';

  let { children } = $props();

  onMount(() => {
    const token = getAccessToken();
    if (token) {
      realtime.connect(token);
    }
  });
</script>

{@render children()}

<script lang="ts">
  import { Shield } from 'lucide-svelte';

  let { children } = $props();
</script>

<div class="flex min-h-screen items-center justify-center px-4 py-8">
  <div class="w-full max-w-sm space-y-8">
    <div class="flex flex-col items-center gap-3">
      <div class="flex size-12 items-center justify-center rounded-full bg-brand-500/20">
        <Shield class="size-6 text-brand-300" aria-hidden="true" />
      </div>
      <h1 class="text-2xl font-semibold tracking-tight text-white">Rust.chat</h1>
    </div>
    {@render children()}
  </div>
</div>

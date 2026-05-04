<script lang="ts">
  import { Send } from 'lucide-svelte';

  let {
    disabled = false,
    disabledReason = '',
    sending = false,
    onsend,
  }: {
    disabled?: boolean;
    disabledReason?: string;
    sending?: boolean;
    onsend?: (content: string) => void;
  } = $props();

  let text = $state('');

  let trimmed = $derived(text.trim());
  let canSend = $derived(!disabled && !sending && trimmed.length > 0);

  function handleSubmit() {
    if (!canSend) return;
    const content = trimmed;
    text = '';
    onsend?.(content);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  }
</script>

<div class="border-t border-white/10 bg-rc-950 px-4 py-3">
  {#if disabled && disabledReason}
    <p class="mb-2 text-xs text-rc-500" role="alert">{disabledReason}</p>
  {/if}
  <div class="flex items-end gap-2">
    <div class="relative flex-1">
      <label for="composer-input" class="sr-only">Message</label>
      <textarea
        id="composer-input"
        bind:value={text}
        onkeydown={handleKeydown}
        disabled={disabled || sending}
        rows="1"
        placeholder={disabled ? '' : 'Message...'}
        class="w-full resize-none rounded-lg border border-white/10 bg-rc-900 px-3 py-2.5 text-sm text-rc-100 placeholder-rc-500 transition focus:border-brand-500/50 focus:outline-none focus:ring-1 focus:ring-brand-500/30 disabled:cursor-not-allowed disabled:opacity-50"
      ></textarea>
    </div>
    <button
      onclick={handleSubmit}
      disabled={!canSend}
      aria-label="Send message"
      class="flex size-10 shrink-0 items-center justify-center rounded-lg bg-brand-600 text-white transition hover:bg-brand-500 focus-visible:outline-2 focus-visible:outline-brand-400 disabled:cursor-not-allowed disabled:opacity-40"
    >
      {#if sending}
        <div class="size-4 animate-spin rounded-full border-2 border-white/30 border-t-white" role="status">
          <span class="sr-only">Sending...</span>
        </div>
      {:else}
        <Send class="size-4" aria-hidden="true" />
      {/if}
    </button>
  </div>
</div>

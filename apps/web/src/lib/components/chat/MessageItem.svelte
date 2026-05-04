<script lang="ts">
  import type { Message } from '$lib/schemas/messages';

  let { message }: { message: Message } = $props();

  function formatTime(iso: string): string {
    const date = new Date(iso);
    const now = new Date();
    const isToday = date.toDateString() === now.toDateString();
    if (isToday) {
      return date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' });
    }
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  let timestamp = $derived(formatTime(message.created_at));
  let authorLabel = $derived(message.author_user_id.slice(0, 8));
</script>

<div class="group flex gap-3 px-4 py-2 transition hover:bg-white/[0.02]" role="listitem">
  <div class="flex size-8 shrink-0 items-center justify-center rounded-full bg-brand-500/20 text-xs font-semibold text-brand-300">
    {authorLabel.slice(0, 2).toUpperCase()}
  </div>
  <div class="min-w-0 flex-1">
    <div class="flex items-baseline gap-2">
      <span class="text-sm font-medium text-rc-200">{authorLabel}</span>
      <time class="text-xs text-rc-500" datetime={message.created_at}>{timestamp}</time>
    </div>
    <p class="mt-0.5 whitespace-pre-wrap break-words text-sm text-rc-100">{message.content}</p>
  </div>
</div>

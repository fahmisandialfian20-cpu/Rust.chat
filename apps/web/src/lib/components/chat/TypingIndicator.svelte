<script lang="ts">
  import { getTypingUsers } from '$lib/stores/typing.svelte';

  let {
    channelId,
    currentUserId,
  }: {
    channelId: string;
    currentUserId?: string;
  } = $props();

  let typingUserIds = $derived(
    getTypingUsers(channelId).filter((id) => id !== currentUserId)
  );

  let label = $derived.by(() => {
    const n = typingUserIds.length;
    if (n === 0) return '';
    if (n === 1) return `${typingUserIds[0]} is typing`;
    if (n === 2) return `${typingUserIds[0]} and ${typingUserIds[1]} are typing`;
    return 'Several people are typing';
  });
</script>

{#if label}
  <div class="flex items-center gap-1 px-4 py-1 text-xs text-rc-400" aria-live="polite" role="status">
    <span>{label}</span>
    <span class="inline-flex gap-px">
      <span class="typing-dot">.</span>
      <span class="typing-dot typing-dot--2">.</span>
      <span class="typing-dot typing-dot--3">.</span>
    </span>
  </div>
{/if}

<style>
  .typing-dot {
    animation: typing-bounce 1.4s infinite both;
    font-weight: bold;
  }

  .typing-dot--2 {
    animation-delay: 0.2s;
  }

  .typing-dot--3 {
    animation-delay: 0.4s;
  }

  @keyframes typing-bounce {
    0%, 20%, 100% {
      opacity: 0.3;
    }
    50% {
      opacity: 1;
    }
  }
</style>

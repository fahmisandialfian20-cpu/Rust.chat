<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { Hash, LoaderCircle, AlertCircle } from 'lucide-svelte';
  import { getChannel, getChannelFlags, getMyPermissions } from '$lib/api/channels';
  import { listMessages, sendMessage } from '$lib/api/messages';
  import { getAccessToken, getUser } from '$lib/stores/auth.svelte';
  import { realtime } from '$lib/stores/realtime';
  import { setTyping, clearChannel } from '$lib/stores/typing.svelte';
  import { subscribeToRealtime, clearAll as clearPresence } from '$lib/stores/presence.svelte';
  import MessageList from '$lib/components/chat/MessageList.svelte';
  import MessageComposer from '$lib/components/chat/MessageComposer.svelte';
  import TypingIndicator from '$lib/components/chat/TypingIndicator.svelte';
  import MediaButton from '$lib/components/media/MediaButton.svelte';
  import type { Message } from '$lib/schemas/messages';

  type ViewState = 'loading' | 'loaded' | 'error' | 'forbidden' | 'notfound';

  let viewState: ViewState = $state('loading');
  let channelName = $state('');
  let channelKind = $state<string | null>(null);
  let flags = $state<{ text_enabled: boolean; voice_group_enabled: boolean; video_group_enabled: boolean } | null>(null);
  let messages: Message[] = $state([]);
  let sending = $state(false);
  let errorMessage = $state('');
  let hasMore = $state(true);
  let loadingMore = $state(false);
  let permissions: string[] = $state([]);

  let spaceId = $derived(page.params.spaceId as string);
  let channelId = $derived(page.params.channelId as string);

  let currentUserId = $derived((getUser() as { id: string } | null)?.id ?? '');
  let canSendMessages = $derived(permissions.includes('send_messages'));

  let composerDisabled = $derived.by(() => {
    if (viewState !== 'loaded') return true;
    if (flags === null) return true;
    if (!flags.text_enabled) return true;
    if (channelKind !== 'Text') return true;
    if (sending) return true;
    if (!canSendMessages) return true;
    return false;
  });
  let composerDisabledReason = $derived.by(() => {
    if (viewState !== 'loaded') return 'Loading channel...';
    if (channelKind !== 'Text') return 'This channel does not support text messages.';
    if (flags && !flags.text_enabled) return 'Text messages are disabled in this channel.';
    if (!canSendMessages) return 'You do not have permission to send messages.';
    return '';
  });

  onMount(() => {
    const token = getAccessToken();
    if (!token) {
      goto('/login');
      return;
    }
    load();
    realtime.connect(token);
    const unsubPresence = subscribeToRealtime(realtime);
    return () => {
      realtime.disconnect();
      clearChannel(channelId);
      unsubPresence();
      clearPresence();
    };
  });

  $effect(() => {
    const cid = channelId;
    const unsub = realtime.subscribe('typing.update', (payload) => {
      const p = payload as { channel_id: string; user_id: string; is_typing: boolean };
      if (p.channel_id === cid) {
        setTyping(p.channel_id, p.user_id, p.is_typing);
      }
    });
    return () => {
      clearChannel(cid);
      unsub();
    };
  });

  async function load() {
    viewState = 'loading';
    try {
      const [channelResult, flagsResult, messagesResult] = await Promise.all([
        getChannel(spaceId, channelId),
        getChannelFlags(spaceId, channelId).catch(() => null),
        listMessages(channelId, { limit: 50 }),
      ]);
      channelName = channelResult.name;
      channelKind = channelResult.kind;
      flags = flagsResult;
      messages = messagesResult.reverse();
      hasMore = messagesResult.length >= 50;
      const perms = await getMyPermissions(spaceId).catch(() => []);
      permissions = perms;
      viewState = 'loaded';
    } catch (err: unknown) {
      const e = err as { status?: number; message?: string };
      if (e.status === 401) {
        goto('/login');
        return;
      }
      if (e.status === 403) {
        viewState = 'forbidden';
        errorMessage = 'You do not have permission to access this channel.';
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

  async function loadMore() {
    if (messages.length === 0) return;
    loadingMore = true;
    try {
      const oldest = messages[0];
      const older = await listMessages(channelId, { limit: 50, before: oldest.id });
      if (older.length === 0) {
        hasMore = false;
      } else {
        messages = [...older.reverse(), ...messages];
        hasMore = older.length >= 50;
      }
    } catch {
      // silently fail, user can retry
    } finally {
      loadingMore = false;
    }
  }

  function handleTypingChange(isTyping: boolean) {
    realtime.send({
      type: 'typing.update',
      payload: { channel_id: channelId, is_typing: isTyping },
    });
  }

  async function handleSend(content: string) {
    sending = true;
    try {
      const msg = await sendMessage(channelId, content);
      messages = [...messages, msg];
    } catch (err: unknown) {
      const e = err as { message?: string };
      errorMessage = e.message ?? 'Failed to send message.';
    } finally {
      sending = false;
    }
  }
</script>

<svelte:head>
  <title>{channelName || 'Channel'} - Rust.chat</title>
</svelte:head>

<div class="flex h-full flex-col">
  <header class="flex items-center gap-2.5 border-b border-white/10 px-5 py-3">
    <Hash class="size-5 text-brand-400" aria-hidden="true" />
    <h1 class="truncate text-base font-semibold text-white">
      {#if viewState === 'loading'}
        <span class="text-rc-500">Loading...</span>
      {:else}
        {channelName}
      {/if}
    </h1>
    <div class="ml-auto">
      {#if viewState === 'loaded' && flags}
        {#if channelKind === 'Voice'}
          <MediaButton {channelId} {spaceId} mode="voice" voiceGroupEnabled={flags.voice_group_enabled} videoGroupEnabled={flags.video_group_enabled} />
        {:else if channelKind === 'Video'}
          <MediaButton {channelId} {spaceId} mode="video" voiceGroupEnabled={flags.voice_group_enabled} videoGroupEnabled={flags.video_group_enabled} />
        {/if}
      {/if}
    </div>
  </header>

  {#if viewState === 'loading'}
    <div class="flex flex-1 items-center justify-center" role="status" aria-label="Loading messages">
      <LoaderCircle class="size-6 animate-spin text-rc-500" aria-hidden="true" />
      <span class="sr-only">Loading messages...</span>
    </div>
  {:else if viewState === 'forbidden'}
    <div class="flex flex-1 items-center justify-center p-8">
      <div role="alert" class="flex items-start gap-3 rounded-lg border border-red-500/30 bg-red-500/10 p-4 text-sm text-red-200">
        <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
        <p>{errorMessage}</p>
      </div>
    </div>
  {:else if viewState === 'notfound'}
    <div class="flex flex-1 items-center justify-center p-8">
      <div role="alert" class="flex items-start gap-3 rounded-lg border border-amber-500/30 bg-amber-500/10 p-4 text-sm text-amber-300">
        <AlertCircle class="mt-0.5 size-4 shrink-0" aria-hidden="true" />
        <p>Channel not found.</p>
      </div>
    </div>
  {:else if viewState === 'error'}
    <div class="flex flex-1 flex-col items-center justify-center gap-4 p-8">
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
  {:else if viewState === 'loaded'}
    <MessageList
      {messages}
      loading={false}
      {hasMore}
      {loadingMore}
      readOnly={!canSendMessages}
      {permissions}
      {currentUserId}
      onloadMore={loadMore}
    />
    <TypingIndicator {channelId} currentUserId={currentUserId} />
    <MessageComposer
      disabled={composerDisabled}
      disabledReason={composerDisabledReason}
      {sending}
      onsend={handleSend}
      onTypingChange={handleTypingChange}
    />
  {/if}
</div>

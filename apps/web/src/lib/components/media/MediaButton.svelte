<script lang="ts">
  import { onDestroy } from 'svelte';
  import { Phone, PhoneOff, Video, VideoOff, LoaderCircle } from 'lucide-svelte';
  import { getAccessToken } from '$lib/stores/auth.svelte';
  import { apiUrl } from '$lib/config';
  import { Room, RoomEvent } from 'livekit-client';

  let {
    channelId,
    spaceId,
    mode,
    voiceGroupEnabled,
    videoGroupEnabled,
  }: {
    channelId: string;
    spaceId: string;
    mode: 'voice' | 'video';
    voiceGroupEnabled: boolean;
    videoGroupEnabled: boolean;
  } = $props();

  let connected = $state(false);
  let connecting = $state(false);
  let error = $state('');

  let room: Room | null = null;

  let enabled = $derived(
    mode === 'voice' ? voiceGroupEnabled : videoGroupEnabled,
  );

  async function toggle() {
    if (connected) {
      room?.disconnect();
      room = null;
      connected = false;
      return;
    }

    connecting = true;
    error = '';

    try {
      const token = getAccessToken();
      if (!token) {
        error = 'Not authenticated';
        return;
      }

      const response = await fetch(
        apiUrl(`/api/v1/channels/${channelId}/media-token`),
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({ mode, intent: 'join' }),
        },
      );

      if (!response.ok) {
        const body = await response.json().catch(() => ({}));
        throw new Error(body.error ?? 'Failed to get media token');
      }

      const data: { provider: string; url: string; room: string; token: string } =
        await response.json();

      const newRoom = new Room({
        adaptiveStream: true,
        dynacast: true,
      });

      newRoom.on(RoomEvent.Disconnected, () => {
        connected = false;
        room = null;
      });

      newRoom.on(RoomEvent.ConnectionStateChanged, (state) => {
        if (state === 'connected') {
          connected = true;
        }
      });

      await newRoom.connect(data.url, data.token);

      if (mode === 'video') {
        await newRoom.localParticipant.setCameraEnabled(true);
        await newRoom.localParticipant.setMicrophoneEnabled(true);
      } else {
        await newRoom.localParticipant.setMicrophoneEnabled(true);
      }

      room = newRoom;
      connected = true;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Connection failed';
    } finally {
      connecting = false;
    }
  }

  onDestroy(() => {
    room?.disconnect();
  });
</script>

{#if enabled}
  <button
    onclick={toggle}
    disabled={connecting}
    class="inline-flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-sm font-medium transition
      {connected
        ? 'bg-red-500/20 text-red-300 hover:bg-red-500/30'
        : 'bg-white/10 text-rc-300 hover:bg-white/20'}
      disabled:cursor-not-allowed disabled:opacity-50"
    title={mode === 'voice' ? 'Voice channel' : 'Video channel'}
  >
    {#if connecting}
      <LoaderCircle class="size-4 animate-spin" />
    {:else if mode === 'voice'}
      {#if connected}
        <PhoneOff class="size-4" />
        <span>Leave</span>
      {:else}
        <Phone class="size-4" />
        <span>Join</span>
      {/if}
    {:else}
      {#if connected}
        <VideoOff class="size-4" />
        <span>Leave</span>
      {:else}
        <Video class="size-4" />
        <span>Join</span>
      {/if}
    {/if}
  </button>
  {#if error}
    <p class="text-xs text-red-400">{error}</p>
  {/if}
{/if}

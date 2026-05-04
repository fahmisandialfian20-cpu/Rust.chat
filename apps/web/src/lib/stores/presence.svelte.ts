import { z } from 'zod';
import type { WebsocketEnvelope } from '$lib/schemas/websocket';

const presenceStatusSchema = z.enum(['online', 'offline', 'idle']);

type PresenceStatus = 'online' | 'offline' | 'idle' | 'unknown';

const presenceState = $state<Map<string, PresenceStatus>>(new Map());
let _version = $state(0);

export function setPresence(userId: string, status: PresenceStatus): void {
  presenceState.set(userId, status);
  _version++;
}

export function getPresence(userId: string): PresenceStatus {
  _version;
  return presenceState.get(userId) ?? 'unknown';
}

export function clearAll(): void {
  presenceState.clear();
  _version++;
}

export function subscribeToRealtime(
  realtime: { subscribe: (type: string, handler: (payload: unknown, envelope: WebsocketEnvelope) => void) => () => void }
): () => void {
  const unsub = realtime.subscribe('presence.updated', (payload) => {
    const parsed = z.object({
      user_id: z.string().min(1),
      status: presenceStatusSchema,
    }).safeParse(payload);

    if (parsed.success) {
      setPresence(parsed.data.user_id, parsed.data.status);
    }
  });

  return unsub;
}

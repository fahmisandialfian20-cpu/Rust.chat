import { writable } from 'svelte/store';
import { z } from 'zod';

export const websocketEnvelopeSchema = z.object({
  version: z.literal(1),
  type: z.string().min(1),
  request_id: z.string().min(1).optional(),
  payload: z.unknown().default({}),
  sent_at: z.string().datetime().optional()
});

export type WebsocketEnvelope = z.infer<typeof websocketEnvelopeSchema>;

export type SocketConnectionState = 'idle' | 'connecting' | 'open' | 'reconnecting' | 'closed' | 'error';

export interface SocketState {
  connection: SocketConnectionState;
  lastEvent: WebsocketEnvelope | null;
  error: string | null;
}

const initialState: SocketState = {
  connection: 'idle',
  lastEvent: null,
  error: null
};

function createSocketStore() {
  const { subscribe, set, update } = writable<SocketState>(initialState);

  return {
    subscribe,
    reset: () => set(initialState),
    setConnection: (connection: SocketConnectionState) => {
      update((state) => ({ ...state, connection, error: null }));
    },
    acceptRawEvent: (value: unknown) => {
      const parsed = websocketEnvelopeSchema.safeParse(value);

      if (!parsed.success) {
        update((state) => ({
          ...state,
          connection: 'error',
          error: 'Received an invalid WebSocket event envelope.'
        }));
        return false;
      }

      update((state) => ({ ...state, lastEvent: parsed.data, error: null }));
      return true;
    },
    setError: (error: string) => {
      update((state) => ({ ...state, connection: 'error', error }));
    }
  };
}

export const socket = createSocketStore();

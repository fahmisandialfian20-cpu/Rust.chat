import { writable } from 'svelte/store';
import { websocketEnvelopeSchema, parseWsEvent } from '$lib/schemas/websocket';
import type { WebsocketEnvelope } from '$lib/schemas/websocket';

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
      const result = parseWsEvent(value);

      if (!result.success) {
        update((state) => ({
          ...state,
          connection: 'error',
          error: result.error
        }));
        return false;
      }

      update((state) => ({ ...state, lastEvent: result.data, error: null }));
      return true;
    },
    setError: (error: string) => {
      update((state) => ({ ...state, connection: 'error', error }));
    }
  };
}

export const socket = createSocketStore();

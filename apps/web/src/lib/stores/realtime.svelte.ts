import { browser } from '$app/environment';
import { websocketUrl } from '$lib/config';
import { parseWsEvent } from '$lib/schemas/websocket';
import type { WebsocketEnvelope } from '$lib/schemas/websocket';
import type { SocketConnectionState } from '$lib/stores/socket';

type MessageHandler = (payload: unknown, envelope: WebsocketEnvelope) => void;

let connectionState = $state<SocketConnectionState>('idle');
let reconnectAttempt = $state(0);

export function getConnectionState(): SocketConnectionState {
  return connectionState;
}

export function getReconnectAttempt(): number {
  return reconnectAttempt;
}

class RealtimeConnection {
  private ws: WebSocket | null = null;
  private handlers = new Map<string, Set<MessageHandler>>();
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private token: string | null = null;

  connect(token: string): void {
    if (!browser) return;
    this.token = token;
    if (this.ws?.readyState === WebSocket.OPEN || this.ws?.readyState === WebSocket.CONNECTING) return;
    connectionState = 'connecting';
    this.connectInternal();
  }

  private connectInternal(): void {
    if (!this.token) return;
    const url = websocketUrl();
    this.ws = new WebSocket(url);
    this.ws.onopen = () => {
      connectionState = 'open';
      reconnectAttempt = 0;
    };
    this.ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        const result = parseWsEvent(data);
        if (result.success) {
          this.dispatch(result.data);
        }
      } catch {
        // ignore malformed messages
      }
    };
    this.ws.onclose = () => {
      if (this.token) {
        connectionState = 'reconnecting';
        this.scheduleReconnect();
      } else {
        connectionState = 'closed';
      }
    };
    this.ws.onerror = () => {
      connectionState = 'error';
      this.ws?.close();
    };
  }

  private dispatch(envelope: WebsocketEnvelope): void {
    const typeHandlers = this.handlers.get(envelope.type);
    if (typeHandlers) {
      for (const handler of typeHandlers) {
        handler(envelope.payload, envelope);
      }
    }
  }

  send(envelope: { type: string; payload: unknown; request_id?: string }): void {
    if (connectionState !== 'open' || !this.ws) return;
    const msg = JSON.stringify({
      version: 1,
      ...envelope,
      sent_at: new Date().toISOString(),
    });
    this.ws.send(msg);
  }

  subscribe(type: string, handler: MessageHandler): () => void {
    if (!this.handlers.has(type)) {
      this.handlers.set(type, new Set());
    }
    this.handlers.get(type)!.add(handler);
    return () => {
      this.handlers.get(type)?.delete(handler);
    };
  }

  disconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    this.token = null;
    connectionState = 'closed';
    if (this.ws) {
      this.ws.onclose = null;
      this.ws.close();
      this.ws = null;
    }
  }

  retry(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    if (this.token) {
      this.ws?.close();
      this.ws = null;
      connectionState = 'connecting';
      this.connectInternal();
    }
  }

  private scheduleReconnect(): void {
    if (!this.token) return;
    if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
    reconnectAttempt++;
    const delay = Math.min(1000 * Math.pow(2, reconnectAttempt), 30000);
    this.reconnectTimer = setTimeout(() => this.connectInternal(), delay);
  }
}

export const realtime = new RealtimeConnection();

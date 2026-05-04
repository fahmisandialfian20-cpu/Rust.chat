# Feature Spec: WebSocket Reconnect State Banner

## Goal

Show clear realtime connection state to users and implement a controlled reconnect lifecycle for the shared WebSocket store.

## Files

Allowed files:

- Extend `apps/web/src/lib/stores/socket.ts`.
- Add `apps/web/src/lib/components/realtime/ReconnectBanner.svelte`.
- Wire banner in `apps/web/src/routes/+layout.svelte` or `(app)/+layout.svelte` depending on auth lifecycle.
- Extend config helpers if WebSocket URL/token handling is needed.

Do not broaden WebSocket event schema validation in this task beyond what is required for connection lifecycle.

## WebSocket endpoint

Endpoint:

- `GET /api/v1/ws`

Auth options from context:

- session cookie
- Authorization bearer token
- short-lived WebSocket token

Current frontend should prefer bearer token from auth store unless backend session cookie flow is chosen. Do not expose refresh token in the WebSocket URL.

## Connection states

The store should support:

- `idle`
- `connecting`
- `open`
- `reconnecting`
- `closed`
- `error`

## UX flow

1. Authenticated app initializes WebSocket connection once.
2. When connected, no banner is shown by default.
3. On transient disconnect, banner shows `Reconnecting...`.
4. On reconnect success, banner briefly shows restored state or disappears.
5. On terminal error, banner shows retry action.
6. On logout, socket closes and resets state.

## Reconnect policy

- Use exponential backoff with jitter.
- Start around 500ms-1s.
- Cap delay around 30s.
- Avoid multiple concurrent sockets.
- Stop reconnecting when user logs out or token is absent.
- Do not reconnect infinitely with an obviously invalid token without surfacing error.

## Security rules

- Do not put long-lived refresh token in WebSocket query params.
- Do not log token values.
- Validate hello/connection events before trusting payloads.
- Do not replay queued user actions blindly after reconnect unless a later task explicitly designs idempotency.

## Acceptance criteria

- Shared socket store owns one WebSocket instance.
- App displays reconnect banner for `reconnecting` and `error` states.
- Backoff policy prevents tight loops.
- Logout/unauthenticated state closes socket cleanly.
- Existing chat/presence/typing consumers do not break.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after reconnect lifecycle and banner are complete. Do not implement full event validation expansion in this task.

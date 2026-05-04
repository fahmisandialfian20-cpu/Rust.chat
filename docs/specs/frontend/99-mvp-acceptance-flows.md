# Frontend MVP Acceptance Flows

This document maps Phase 9 tasks to the product-level acceptance flows. It is not a separate implementation task unless the project lead explicitly creates one.

## Flow A: Hoster bootstrap to first chat

Required tasks:

1. `01-foundation.md`
2. `02-bootstrap-hoster.md`
3. `03-auth-forms.md`
4. `04-lobby-spaces.md`
5. `05-channel-list.md`
6. `06-chat-area.md`
7. `09-role-editor.md`
8. `10-channel-settings.md`
9. `12-reconnect-banner.md`
10. `13-ws-event-validation.md`

Expected manual flow:

1. Fresh instance has no owner.
2. Hoster opens `/bootstrap`.
3. Hoster creates first account.
4. Hoster lands in authenticated app shell/lobby.
5. Hoster creates or sees a space, depending on backend readiness.
6. Hoster creates public/private channels, depending on backend readiness.
7. Hoster configures roles and feature flags, depending on backend readiness.
8. Hoster enters allowed text channel.
9. Hoster sends and receives a realtime text message.

Frontend must not mark unavailable backend operations as successful. If backend endpoints are missing, UI may show a disabled state and the task report must list the blocker.

## Flow B: Member invite registration to allowed chat

Required tasks:

1. `03-auth-forms.md`
2. `04-lobby-spaces.md`
3. `05-channel-list.md`
4. `06-chat-area.md`
5. `07-typing-indicator.md`
6. `08-presence-indicator.md`
7. `12-reconnect-banner.md`
8. `13-ws-event-validation.md`

Expected manual flow:

1. Member opens invite-backed registration route.
2. Member registers with invite code.
3. Member logs in or is automatically authenticated from registration response.
4. Member sees only spaces returned by backend.
5. Member sees only channels returned by backend-visible channel endpoint.
6. Private channels without access are absent, not merely hidden by local filter.
7. Member can read and send messages only when backend permits and channel flags allow it.
8. Typing and presence indicators operate only within authorized channel context.

## Flow C: Composer disabled states

Required tasks:

1. `05-channel-list.md`
2. `06-chat-area.md`
3. `10-channel-settings.md`

Composer must be disabled when:

- User is unauthenticated.
- Channel is not loaded or not authorized.
- Channel is archived.
- Channel kind is not text-compatible.
- `text_enabled` is false.
- Backend permission context lacks `send_messages`.
- Message is empty/whitespace-only.
- Submit request is already in flight.

## Flow D: Realtime resilience

Required tasks:

1. `06-chat-area.md`
2. `07-typing-indicator.md`
3. `08-presence-indicator.md`
4. `12-reconnect-banner.md`
5. `13-ws-event-validation.md`

Expected behavior:

- Invalid WebSocket events are rejected before UI state updates.
- Reconnect banner appears on disconnect/retry.
- Presence becomes stale/unknown after disconnect until refreshed.
- Typing indicators expire automatically.
- Message duplication is prevented by message id/request id handling.

## Final frontend validation checklist

At the end of any final frontend integration pass, run:

```bash
npm --prefix ./apps/web run check
npm --prefix ./apps/web run build
npm --prefix ./apps/web test
```

Then manually verify the flows above against a running backend with PostgreSQL and Redis.

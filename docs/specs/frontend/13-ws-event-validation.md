# Feature Spec: WebSocket Event Validation

## Goal

Validate every WebSocket event used by the frontend with Zod before any store/UI consumes it.

## Files

Allowed files:

- `apps/web/src/lib/schemas/websocket.ts`
- Extend `apps/web/src/lib/stores/socket.ts`
- Extend typed stores for chat, typing, and presence as needed.
- Add focused tests or schema assertions if a test runner exists; otherwise keep validation exercised through TypeScript/check/build.

Do not introduce new UI features in this task.

## Canonical envelope from context

Expected envelope:

- `version: 1`
- `type: string`
- `request_id?: string`
- `payload: object`
- `sent_at?: ISO datetime`

Required server event schemas:

- `hello.ok`
- `message.created`
- `message.updated`
- `message.deleted`
- `typing.updated`
- `presence.updated`
- `channel.created`
- `channel.updated`
- `permission.updated`
- `member.joined`
- `member.left`
- `notification.created`
- `media.room.updated`
- `error`

Currently implemented/minimum frontend consumers may only need a subset, but this task must define schemas for all expected MVP event names so future UI work does not parse raw objects.

## Known backend mismatch

Current backend code may emit legacy tagged events shaped like:

- top-level `type`
- data under `data`

If backend is not yet aligned, implement a narrow normalizer:

1. Accept canonical envelope first.
2. If canonical parse fails, attempt legacy parse for known backend events only.
3. Normalize to canonical internal shape.
4. Reject unknown/invalid events with a non-fatal socket store error.

Do not allow arbitrary payload passthrough.

## Validation behavior

- Unknown event type: ignore and record diagnostic warning state, do not crash UI.
- Invalid payload for known type: ignore event and set non-fatal error.
- Version mismatch: ignore event unless explicit compatibility adapter exists.
- `error` event: parse stable `code`, `message`, optional `details`.

## Security rules

- Do not trust events for permission decisions unless event type is specifically permission update and backend contract is validated.
- Do not render HTML from event payloads.
- Do not append message events outside the active authorized channel context.
- Do not reveal notification content for private channels unless backend/user preference explicitly allows it.

## Acceptance criteria

- `websocket.ts` exports Zod schemas and TypeScript types for the full event set.
- Socket store rejects invalid events before updating feature stores.
- Legacy adapter exists only if backend still needs it and is constrained to known event shapes.
- Existing chat, typing, presence, and reconnect behavior continue to pass checks/build.
- No new visible UI feature is introduced in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after schema validation is complete and all frontend checks pass. Do not implement additional product UI in this task.

# Feature Spec: Typing Indicator UI

## Goal

Show typing activity for the active channel without leaking private channel activity or creating excessive WebSocket traffic.

## Files

Allowed files:

- Extend `apps/web/src/lib/stores/socket.ts`.
- Add `apps/web/src/lib/stores/typing.ts` if a separate derived store is cleaner.
- Extend `apps/web/src/lib/components/chat/MessageComposer.svelte`.
- Add `apps/web/src/lib/components/chat/TypingIndicator.svelte`.
- Extend WebSocket schemas under `apps/web/src/lib/schemas/`.

Do not implement presence in this task.

## WebSocket contract

Expected server event:

- `typing.updated`

Expected payload:

- `channel_id: uuid`
- `user_id: uuid`
- `is_typing: boolean`

Expected client event, if backend supports client-send over WebSocket:

- `typing.update` or project-approved equivalent.

Known backend gap:

- Current backend event naming/shape may not match the versioned envelope from the context docs. If this is unresolved, implement schema support behind a small adapter and document the mismatch in the task report. Do not weaken validation globally.

## UX flow

1. User focuses composer in an authorized channel.
2. When content changes from empty to non-empty, send `is_typing: true` with debounce/throttle.
3. After inactivity timeout or composer clears/submits, send `is_typing: false`.
4. Other users' typing state appears above or below composer.
5. Current user's own typing event is not shown back as an indicator.
6. When user leaves channel, clear local typing indicators for that channel.

## Rate limiting and lifecycle

- Minimum throttle: do not send typing true more than once every 2 seconds per channel.
- Auto-expire remote typing state after 5 seconds without refresh.
- Send typing false on submit, composer blur, and route leave when feasible.

## Security rules

- Only display typing events for the currently active authorized channel.
- Do not show user identifiers that are not available in channel member context. If names are unavailable, show generic text such as `Someone is typing...`.
- Do not create a polling fallback that reveals channel activity.

## Acceptance criteria

- Typing indicator component exists and is wired to active chat route.
- Zod schema validates `typing.updated` payload.
- Local outbound typing events are throttled/debounced.
- Remote typing state expires automatically.
- No presence indicator is implemented in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after typing UI is complete and validated. Do not implement presence in this task.

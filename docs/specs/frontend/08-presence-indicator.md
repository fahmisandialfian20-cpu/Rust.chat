# Feature Spec: Presence Indicator UI

## Goal

Show online/offline presence for users relevant to the active chat context without exposing private membership or activity beyond backend-authorized data.

## Files

Allowed files:

- Extend `apps/web/src/lib/stores/socket.ts`.
- Add `apps/web/src/lib/stores/presence.ts` if a separate store is cleaner.
- Add `apps/web/src/lib/components/presence/PresenceDot.svelte`.
- Add `apps/web/src/lib/components/presence/PresenceLabel.svelte` if needed.
- Extend user/member schemas under `apps/web/src/lib/schemas/`.

Do not implement typing changes in this task.

## WebSocket contract

Expected server event:

- `presence.updated`

Expected payload:

- `user_id: uuid`
- `status: string`

Expected statuses:

- `online`
- `offline`
- `idle` if backend supports it
- `unknown` as frontend fallback only

Known backend gap:

- Current backend event shape may use legacy tagged events. If not aligned with the context envelope, add a localized adapter and document the mismatch. Do not weaken global event validation.

## UX flow

1. When the user enters the app shell, presence store starts in `unknown` state for users.
2. When `presence.updated` arrives, update presence map by `user_id`.
3. Message author UI can display a presence dot only when user identity is already authorized and known.
4. Channel/member UI can display aggregate presence only from authorized member data.
5. On WebSocket disconnect/reconnect, mark stale presence as `unknown` until fresh events arrive.

## Security and privacy rules

- Do not show presence for users not present in backend-authorized member/message data.
- Do not reveal private channel membership from presence events.
- Do not use presence to decide permissions.
- Do not show message previews or private channel names from notifications/presence.

## UI requirements

- Presence dot must have accessible text, not color-only state.
- Offline/unknown states must be visually distinct enough.
- If no presence data exists, UI remains stable and shows `unknown` or hides the dot depending on context.

## Acceptance criteria

- Presence store tracks status by user id.
- `presence.updated` payload is validated with Zod.
- Chat/message UI can consume presence without breaking when data is absent.
- Disconnect/reconnect clears or downgrades stale presence appropriately.
- No typing feature changes are implemented in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after presence display and store behavior are complete. Do not implement admin features in this task.

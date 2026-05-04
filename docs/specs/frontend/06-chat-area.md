# Feature Spec: Chat Area History and Send Message

## Goal

Implement the text chat screen for a selected channel: load message history, display messages, and send new text messages when the backend-authorized channel context allows it.

## Routes and files

Preferred route:

- `apps/web/src/routes/(app)/spaces/[spaceId]/channels/[channelId]/+page.svelte`

Allowed support files:

- `apps/web/src/lib/api/messages.ts`
- `apps/web/src/lib/schemas/messages.ts`
- `apps/web/src/lib/components/chat/MessageList.svelte`
- `apps/web/src/lib/components/chat/MessageItem.svelte`
- `apps/web/src/lib/components/chat/MessageComposer.svelte`
- Extend `apps/web/src/lib/stores/socket.ts` only for receiving `message.created` after REST send is done.

Do not implement typing indicator or presence in this task.

## Backend contracts

Message history:

- `GET /api/v1/channels/{channel_id}/messages?limit=50&before={message_id?}`

Send message:

- `POST /api/v1/channels/{channel_id}/messages`

Send request:

- `content: string`
- `kind?: string`
- `reply_to_message_id?: uuid`

Message DTO expected:

- `id`
- `channel_id`
- `author_user_id`
- `content`
- `kind`
- `reply_to_message_id?`
- `edited_at?`
- `deleted_at?`
- `created_at`

Required channel context:

- The composer must be enabled only when backend-provided context says the user can send messages and `text_enabled` is true.
- If no permission/feature-flag context exists yet, default composer state must be disabled with an explanatory message.

## UX flow

1. User opens a visible text channel.
2. Page loads channel context and message history.
3. Message list renders oldest-to-newest in view while preserving pagination state.
4. Composer is disabled until permission and feature flag allow sending.
5. User writes message and submits.
6. Submit uses REST endpoint first.
7. Optimistic UI is optional, but if used, messages must reconcile with server response by id/request id.
8. Incoming `message.created` events append only when the message belongs to the active channel.

## Composer disabled rules

Disable composer when any of these is true:

- Channel is not loaded.
- Channel kind is not text-compatible.
- `text_enabled` is false.
- Backend permission context does not include `send_messages` allowed.
- User is unauthenticated.
- Request is currently submitting.

## Security rules

- Do not allow send based only on frontend role names.
- Do not show messages from a channel if backend returns 403/404.
- Do not send empty/whitespace-only content.
- Do not trust WebSocket events for unauthorized channels; ignore events outside active authorized context.

## Acceptance criteria

- Chat route renders for selected channel.
- History loads through typed API helper and Zod schema.
- Composer sends text via REST only when allowed by server context.
- Composer is disabled for missing permission/disabled text flag.
- Incoming `message.created` can update the active list without duplicate ids.
- No typing or presence UI is implemented in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after history, basic message display, and send behavior are validated. Do not implement typing or presence in this task.

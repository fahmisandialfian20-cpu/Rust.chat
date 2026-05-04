# Feature Spec: Channel List with Permission-Scoped Visibility

## Goal

Show the channel list for a selected space while preserving the rule that users must not see private channels they are not allowed to access.

## Routes and files

Preferred route/layout:

- `apps/web/src/routes/(app)/spaces/[spaceId]/+layout.svelte`

Allowed support files:

- `apps/web/src/lib/api/channels.ts`
- `apps/web/src/lib/schemas/channels.ts`
- `apps/web/src/lib/components/channels/ChannelList.svelte`
- `apps/web/src/lib/components/channels/ChannelListItem.svelte`

Do not implement message history or composer in this task.

## Backend contract

Required member-safe endpoint:

- Preferred: `GET /api/v1/spaces/{space_id}/channels/visible?limit=50&offset=0`
- Alternative: any backend endpoint that explicitly returns only channels visible to the authenticated user.

Current backend note:

- There is a `list_visible_channels` handler function intent, but route wiring may be missing.
- Raw `GET /api/v1/spaces/{space_id}/channels` must not be used for member channel navigation unless backend guarantees it is permission-scoped.

Channel DTO fields expected:

- `id`
- `space_id`
- `parent_id?`
- `name`
- `slug`
- `kind`
- `visibility`
- `position`
- `topic?`
- `created_by`
- `archived_at?`
- `created_at`
- `updated_at`

## UX flow

1. User opens `/spaces/{spaceId}`.
2. Layout fetches space context and visible channels.
3. Channels are sorted by `position`, then by name as fallback.
4. Text, voice, and video channels have distinct icons/labels.
5. Clicking a channel navigates to `/spaces/{spaceId}/channels/{channelId}`.
6. If no visible channels exist, show an empty state that does not reveal hidden channel names/counts.

## Visibility requirements

- Private channel absence must come from the backend response.
- Do not render a hidden/locked private channel placeholder for unauthorized users.
- Do not infer channel access from `visibility === "public"` alone. Backend still decides.

## Error handling

- 401: route to login.
- 403: show access denied for the space.
- 404: show not found without revealing whether the space/channel exists privately.
- Network error: show retry action.

## Acceptance criteria

- `/spaces/[spaceId]` app layout renders visible channel list.
- The API helper and Zod schema validate channel DTOs.
- Raw unscoped channel endpoint is not used for member UI unless explicitly documented as backend-scoped.
- Empty state leaks no private metadata.
- No chat message UI is implemented in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after channel list/navigation shell is complete. Do not implement chat history or composer in this task.

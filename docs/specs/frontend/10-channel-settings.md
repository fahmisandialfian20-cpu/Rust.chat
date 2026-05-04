# Feature Spec: Admin Channel Settings and Feature Flags

## Goal

Provide an admin UI for channel metadata and channel feature flags, including toggles that later drive composer/media UI behavior.

## Routes and files

Preferred route:

- `apps/web/src/routes/admin/channels/+page.svelte`

Allowed support files:

- Extend `apps/web/src/lib/api/channels.ts`.
- Extend `apps/web/src/lib/schemas/channels.ts`.
- `apps/web/src/lib/components/admin/ChannelSettingsForm.svelte`
- `apps/web/src/lib/components/admin/FeatureFlagToggles.svelte`

Do not implement role editor changes in this task.

## Backend contracts

Channel metadata:

- `GET /api/v1/spaces/{space_id}/channels/{channel_id}`
- `PUT /api/v1/spaces/{space_id}/channels/{channel_id}`

Feature flags:

- `GET /api/v1/spaces/{space_id}/channels/{channel_id}/feature-flags`
- `PUT /api/v1/spaces/{space_id}/channels/{channel_id}/feature-flags`

Current backend feature flag fields:

- `text_enabled`
- `file_upload_enabled`
- `voice_group_enabled`
- `video_group_enabled`
- `threads_enabled`
- `reactions_enabled`

Context/product fields to watch for future alignment:

- `send_file_enabled` vs current `file_upload_enabled`
- `mentions_enabled`
- `pin_message_enabled`

Agent coder must align with current backend fields and document any context/backend naming mismatch in the task report.

## UX flow

1. Admin opens `/admin/channels`.
2. UI selects a space and channel from backend-scoped options.
3. UI loads channel metadata and feature flags.
4. Admin edits name/topic/visibility if supported.
5. Admin toggles feature flags.
6. Save metadata and feature flag updates with clear loading state.
7. Success updates local UI from server response.

## Security rules

- Do not show settings controls unless backend says user can manage channels.
- Do not infer manage permission from role names.
- Do not expose private channels to unauthorized admins/members.
- Feature flags are UI hints only; backend remains enforcement authority.

## Validation rules

Use Zod v4:

- Channel name: non-empty trimmed string.
- Topic: optional string with reasonable UI max length.
- Visibility: `public` or `private`.
- Feature flags: booleans only.

## Acceptance criteria

- `/admin/channels` renders channel settings shell.
- Feature flag toggles cover all current backend feature flags.
- Save calls typed API helpers and handles success/error states.
- Chat composer disabled behavior from task 06 can consume updated `text_enabled` state if already implemented.
- No role editor changes are implemented in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after channel settings and feature flag UI are validated. Do not implement theme settings in this task.

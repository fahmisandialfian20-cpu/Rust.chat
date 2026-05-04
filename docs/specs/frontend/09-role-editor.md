# Feature Spec: Admin Role Editor with Permission Checklist

## Goal

Provide an admin UI for creating/editing roles and assigning checklist-style permissions, while ensuring backend permission checks remain authoritative.

## Routes and files

Preferred route:

- `apps/web/src/routes/admin/roles/+page.svelte`

Allowed support files:

- `apps/web/src/lib/api/roles.ts`
- `apps/web/src/lib/schemas/roles.ts`
- `apps/web/src/lib/schemas/permissions.ts`
- `apps/web/src/lib/components/admin/RoleEditor.svelte`
- `apps/web/src/lib/components/admin/PermissionChecklist.svelte`

Do not implement channel settings in this task.

## Backend contracts

Preferred endpoints:

- `GET /api/v1/spaces/{space_id}/roles`
- `POST /api/v1/spaces/{space_id}/roles`
- `GET /api/v1/spaces/{space_id}/roles/{role_id}`
- `PUT /api/v1/spaces/{space_id}/roles/{role_id}`
- `DELETE /api/v1/spaces/{space_id}/roles/{role_id}`
- `PUT /api/v1/spaces/{space_id}/roles/{role_id}/permissions`

Known backend gap:

- Role CRUD handlers may not exist yet. If missing, agent coder may build the typed UI shell with disabled submit and clear blocker messaging, but must not fake successful role persistence.

## Permission checklist source

Use the server-aligned permission keys from `context/05-permissions-rbac.md` and `apps/server/src/permissions/keys.rs`:

- `manage_instance`
- `manage_spaces`
- `manage_roles`
- `manage_members`
- `manage_channels`
- `manage_invites`
- `view_audit_log`
- `view_space`
- `view_channel`
- `read_messages`
- `send_messages`
- `edit_own_message`
- `delete_own_message`
- `edit_any_message`
- `delete_any_message`
- `pin_messages`
- `mention_everyone`
- `send_files`
- `create_threads`
- `manage_threads`
- `add_reactions`
- `join_voice`
- `start_voice`
- `join_video`
- `start_video`
- `share_screen`
- `kick_members`
- `ban_members`
- `mute_members`
- `manage_moderation`
- `customize_own_profile`
- `customize_space`
- `use_webhooks`

## UX flow

1. Admin opens `/admin/roles`.
2. UI loads spaces/admin context first if required.
3. UI lists roles for selected space.
4. Selecting a role opens editor with name/color/position if supported and permission checklist.
5. Permission groups are displayed by category: instance/admin, space/channel, messages, files/threads/reactions, media, moderation, profile/customization.
6. Save sends the changed role payload to backend.
7. Backend errors are surfaced clearly.

## Security rules

- Do not show role editor to users unless backend says they can manage roles.
- Do not infer admin status from role name.
- Do not allow editing/demoting Hoster in UI unless backend explicitly allows it.
- UI disabling is only convenience; backend must still reject unauthorized changes.

## Acceptance criteria

- `/admin/roles` renders a role management shell.
- Permission checklist includes all server permission keys exactly once.
- Zod schemas validate role and permission payloads.
- Save flow uses backend endpoints if available, otherwise remains disabled with documented blocker.
- No channel settings are implemented in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after role editor UI/contract is complete. Do not implement channel settings in this task.

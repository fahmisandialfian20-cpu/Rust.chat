# Feature Spec: Lobby and Spaces List

## Goal

Create the authenticated app entry area where a user sees only spaces that the backend says they can access.

## Routes and files

Preferred routes:

- `apps/web/src/routes/(app)/+layout.svelte`
- `apps/web/src/routes/(app)/lobby/+page.svelte`
- `apps/web/src/routes/+page.svelte` may redirect to `/login` or `/lobby` depending on auth state.

Allowed support files:

- `apps/web/src/lib/api/spaces.ts`
- `apps/web/src/lib/schemas/spaces.ts`
- Extend `apps/web/src/lib/stores/auth.ts`
- Add app shell components under `apps/web/src/lib/components/app/`

Do not implement channel list or chat in this task.

## Backend contract

Preferred endpoint for member UI:

- `GET /api/v1/spaces?limit=50&offset=0`

Required behavior:

- The response must be permission-scoped by the backend before production use.
- Frontend displays the returned list as-is and does not try to infer hidden spaces locally.

Space DTO fields currently expected from backend:

- `id`
- `name`
- `slug`
- `description?`
- `icon_object_id?`
- `created_by`
- `visibility`
- `settings`
- `created_at`
- `updated_at`

Known backend gap:

- Current spaces handler may list spaces without authenticated user scoping. If this is still true, agent coder must not ship member-visible private space logic. Either wire to a scoped endpoint if available, or render an authenticated shell with an explicit blocker note in the task report.

## UX flow

1. User lands on `/lobby` after auth.
2. App shell validates local auth token presence and calls `GET /api/v1/auth/me` if needed.
3. If unauthenticated, redirect to `/login`.
4. If authenticated, fetch spaces from the server.
5. Show loading, empty, error, and success states.
6. Each space item links to `/spaces/{spaceId}`.
7. The lobby must explain that lobby access is not channel access.

## UI requirements

- Left or top navigation can be introduced in `(app)/+layout.svelte`.
- Show current user identity from auth store.
- Include logout affordance if auth store already supports it; otherwise show user state only and leave logout for a later auth polish task.
- Empty state should guide Hoster/Admin to create a space only if backend permissions indicate that action is allowed. If permission data is absent, do not show create action yet.

## Security rules

- Do not display spaces from cached data until authenticated state is confirmed.
- Do not implement frontend-only filtering for private spaces.
- Do not assume every authenticated user can create spaces.

## Acceptance criteria

- Authenticated layout exists under `(app)`.
- `/lobby` loads spaces from backend through a typed API helper.
- Spaces are rendered exactly from server response.
- Loading, empty, and error states exist.
- Unauthenticated users are routed to `/login`.
- No channel UI is implemented in this task.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after lobby and spaces list are complete. Do not implement channel routes in this task.

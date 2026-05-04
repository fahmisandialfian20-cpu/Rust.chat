# Feature Spec: Hoster Bootstrap Page

## Goal

Allow the first deployer to create the primary Hoster account from the UI after a fresh deployment.

## Route and files

Preferred route:

- `apps/web/src/routes/(auth)/bootstrap/+page.svelte`

Allowed support files for this task only:

- `apps/web/src/routes/(auth)/+layout.svelte` if an auth shell is needed.
- `apps/web/src/lib/api/auth.ts` for the bootstrap request.
- `apps/web/src/lib/schemas/auth.ts` for Zod validation.
- `apps/web/src/lib/stores/auth.ts` only if token persistence is needed by this page.

Do not create login/register pages in this task.

## Backend contract

Endpoint:

- `POST /api/v1/auth/bootstrap-owner`

Request:

- `username: string`
- `password: string`

Response:

- `user`
- `access_token`
- `refresh_token`

Known server behavior:

- Fails with conflict when instance already has owner.
- Username must be at least 3 characters.
- Username must not contain spaces.
- Server remains source of truth for all validation.

## UX flow

1. User opens `/bootstrap`.
2. Page explains this is only for the first Hoster after deployment.
3. User enters username and password.
4. Zod validation runs before submit.
5. Submit button shows loading state.
6. On success, persist auth tokens through the shared auth store/API helper and redirect to the next app entry route.
7. If backend returns conflict, show a clear message and link to `/login`.
8. If network/API error occurs, show stable error text without leaking internals.

## Validation rules

Use Zod v4:

- `username`: trim, min 3, no whitespace.
- `password`: min 6 to match current backend minimum used in register flow; UI may display stronger password guidance but must not reject valid backend-compatible passwords unless product decision changes.

## Permission and security rules

- Do not infer Hoster status from frontend state beyond the authenticated response.
- Do not store password after submit.
- Store tokens in one shared auth abstraction so later web/desktop differences can be isolated.
- Do not expose refresh token in logs or UI.

## Acceptance criteria

- `/bootstrap` renders in the auth shell.
- Valid form submits to `/api/v1/auth/bootstrap-owner`.
- Invalid local input is blocked with inline messages.
- Conflict response tells user to login instead.
- Success stores tokens and redirects.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after this page works and validation passes. Do not implement login/register in the same task.

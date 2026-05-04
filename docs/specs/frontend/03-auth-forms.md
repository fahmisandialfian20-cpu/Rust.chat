# Feature Spec: Login and Register Forms

## Goal

Provide member authentication screens for existing users and invite-based registration while preserving the server as the auth authority.

## Routes and files

Preferred routes:

- `apps/web/src/routes/(auth)/login/+page.svelte`
- `apps/web/src/routes/(auth)/register/+page.svelte`

Allowed support files:

- Extend `apps/web/src/lib/api/auth.ts`.
- Extend `apps/web/src/lib/schemas/auth.ts`.
- Extend `apps/web/src/lib/stores/auth.ts`.
- Add small reusable auth form components under `apps/web/src/lib/components/auth/` if needed.

Do not implement lobby/spaces in this task.

## Backend contracts

Login endpoint:

- `POST /api/v1/auth/login`

Login request:

- `username_or_email: string`
- `password: string`
- `client?: { client_type, platform?, device_name? }`

Register endpoint:

- `POST /api/v1/auth/register`

Register request:

- `username: string`
- `password: string`
- `invite_code?: string`

Response for both:

- `user`
- `access_token`
- `refresh_token`

Known backend gap:

- `invite_code` exists in handler payload but current service may not enforce invite acceptance yet. Frontend must still send the code if present, but must not pretend invite validation happened unless backend confirms it.

## UX flow: login

1. User opens `/login`.
2. User enters username/email and password.
3. Client metadata is included: `client_type: web`, platform from browser helper, optional device name.
4. On success, auth store persists tokens and user.
5. Redirect to lobby.
6. On unauthorized, show generic invalid credentials message.

## UX flow: register

1. User opens `/register` or `/register?invite=CODE`.
2. If invite query exists, prefill hidden/read-only invite code state.
3. User enters username and password.
4. On success, persist auth state and redirect to lobby.
5. If backend rejects invite or username, show inline error.

## Validation rules

Use Zod v4:

- Login identifier: min 1.
- Username: trim, min 3, no whitespace.
- Password: min 6.
- Invite code: optional non-empty string when present.

## Security rules

- Do not log tokens, passwords, or invite codes.
- Do not hardcode bearer tokens.
- Token persistence must be isolated in `auth` store/helper for later secure Tauri storage.
- Browser local storage/session storage choice must be documented in code comments if used.

## Acceptance criteria

- `/login` and `/register` render with shared auth UI language.
- Both forms validate with Zod before submit.
- Login sends client metadata.
- Register forwards invite code when present.
- Auth tokens and user are available from a shared auth store after success.
- Auth errors show stable user-facing messages.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after login/register are complete and validated. Do not implement lobby in this task.

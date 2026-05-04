# Feature Spec: Theme Settings

## Goal

Allow a user to configure personal theme preferences while keeping theme data tied to backend-owned user preference records when available.

## Routes and files

Preferred route:

- `apps/web/src/routes/(app)/settings/theme/+page.svelte`

Allowed support files:

- `apps/web/src/lib/api/theme.ts`
- `apps/web/src/lib/schemas/theme.ts`
- `apps/web/src/lib/stores/theme.ts`
- `apps/web/src/lib/components/settings/ThemeSettingsForm.svelte`

Do not implement profile editing, devices, or notifications in this task.

## Backend contract

Preferred endpoints:

- `GET /api/v1/profile/theme`
- `PUT /api/v1/profile/theme`

Expected persisted concept from context/database:

- `user_theme_preferences`

Known backend gap:

- Theme preference endpoints may not exist yet. If missing, agent coder may implement local preview with explicit disabled save state, but must not claim persistence.

## Theme model

Minimum frontend theme fields:

- `mode`: `dark`, `light`, or `system`
- `accent`: controlled token name, not arbitrary CSS
- `density`: `comfortable` or `compact`
- `message_display`: `cozy` or `compact`

Future-safe optional fields:

- `reduced_motion`
- `high_contrast`

## UX flow

1. User opens theme settings route.
2. UI loads saved preferences if backend endpoint exists.
3. User changes theme controls.
4. UI previews theme immediately using CSS variables/classes.
5. Save persists to backend when endpoint exists.
6. Reset returns to server/default values.

## Security and safety rules

- Do not allow arbitrary user CSS.
- Do not inject raw CSS strings from server or local storage.
- Only allow predefined token values.
- If using local storage fallback for preview, namespace keys clearly and document that it is not authoritative persistence.

## Tailwind v4 requirements

- Do not add `tailwind.config.js`.
- Theme tokens remain in `src/app.css` and runtime changes should be done through classes/data attributes/CSS variables.
- Keep tokens compatible with Tauri WebView.

## Acceptance criteria

- Theme settings route renders in authenticated app shell.
- User can preview allowed theme options.
- Save uses typed backend API if endpoint exists; otherwise save is disabled with blocker messaging.
- No arbitrary CSS input exists.
- Settings survive page navigation if persisted or local preview fallback is intentionally used.
- `npm --prefix ./apps/web run check` passes.
- `npm --prefix ./apps/web run build` passes.
- `npm --prefix ./apps/web test` passes.

## Stop condition

Stop after theme settings are complete. Do not implement reconnect banner in this task.

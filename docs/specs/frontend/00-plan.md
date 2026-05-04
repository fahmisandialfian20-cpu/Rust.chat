# Frontend MVP Plan

> Scope: Phase 9 web app/reference client and future Tauri reuse.
>
> Rule: implement and validate one task at a time. Do not batch multiple feature implementations in one change.

## Frontend definition

`apps/web` is the full Rust.chat web app:

- user-facing chat client;
- Hoster/Admin management UI;
- reference client for backend REST/WebSocket contracts;
- reusable UI base for Tauri desktop.

It is not a random chat demo, not a marketing-only site, and not the authority for permissions.

## Required planning docs

Before any feature task, agent coder must read:

1. `00-context7-validation.md`
2. `00-web-app-architecture.md`
3. `00-ui-ux-design-system.md`
4. `00-library-and-state-policy.md`
5. this plan
6. the selected feature spec

## Agent coder execution flow

1. Start from this document, then read the spec for the next `Planned` task.
2. Confirm dependencies are `Done`. If a dependency is not done, stop and report.
3. Implement only the selected task.
4. Do not implement the next task opportunistically.
5. Run task validation:
   - `npm --prefix ./apps/web run check`
   - `npm --prefix ./apps/web run build`
   - `npm --prefix ./apps/web test`
6. If validation succeeds, update the task status in this file and the related checkbox in `TODO.md`.
7. If validation fails, fix within 1-2 focused attempts. If still failing, stop and report the blocker.

## Ordered task queue

| Order | Feature/function | Spec | Status | Depends on |
|---:|---|---|---|---|
| 1 | Frontend foundation | `docs/specs/frontend/01-foundation.md` | Done | Context + TODO |
| 2 | Hoster bootstrap page | `docs/specs/frontend/02-bootstrap-hoster.md` | Planned | 1 |
| 3 | Login/register forms | `docs/specs/frontend/03-auth-forms.md` | Planned | 2 |
| 4 | Lobby spaces list | `docs/specs/frontend/04-lobby-spaces.md` | Planned | 3 |
| 5 | Channel list with permission-scoped visibility | `docs/specs/frontend/05-channel-list.md` | Planned | 4 |
| 6 | Chat area history + send message | `docs/specs/frontend/06-chat-area.md` | Planned | 5 |
| 7 | Typing indicator UI | `docs/specs/frontend/07-typing-indicator.md` | Planned | 6 |
| 8 | Presence indicator UI | `docs/specs/frontend/08-presence-indicator.md` | Planned | 6 |
| 9 | Admin role editor | `docs/specs/frontend/09-role-editor.md` | Planned | 4 |
| 10 | Admin channel settings + feature flags | `docs/specs/frontend/10-channel-settings.md` | Planned | 5 |
| 11 | Theme settings | `docs/specs/frontend/11-theme-settings.md` | Planned | 3 |
| 12 | WebSocket reconnect state banner | `docs/specs/frontend/12-reconnect-banner.md` | Planned | 6 |
| 13 | WebSocket event validation | `docs/specs/frontend/13-ws-event-validation.md` | Planned | 12 |

## Web-app quality gates

Every feature task must preserve:

- full loading/empty/error/forbidden states where applicable;
- accessible labels and focus states;
- responsive behavior for desktop/tablet/small web screens;
- server-owned permission model;
- Zod validation for form/API/WS boundaries introduced by the task;
- Svelte 5 runes and `$app/state` guidance;
- Tailwind v4 CSS-first design tokens.

## Validation policy

Each task must end with:

1. `npm --prefix ./apps/web run check`
2. `npm --prefix ./apps/web run build`
3. `npm --prefix ./apps/web test`

If a task needs backend integration, add the smallest possible frontend contract/schema first, then wire it to the real API in that task. Do not fake backend permission decisions in the UI.

## Product guardrails

- The frontend never decides final permissions; it only reflects server-returned state.
- Private channel visibility must be driven by server-scoped channel responses.
- Web and Tauri use the same UI shell and REST/WebSocket contracts.
- Use Svelte 5 runes in components.
- Use Tailwind CSS v4 with CSS-first theme tokens in `src/app.css`.
- Use Zod v4 for form and WebSocket payload validation.
- With the current static/Tauri-oriented foundation, prefer client-side API calls over `+page.server.ts` unless the deployment target changes to SSR.

## Known backend contract gaps to watch

These are not frontend bugs. Agent coder must not work around them by trusting frontend state.

- Permission-scoped `my spaces` endpoint may be missing.
- Permission-scoped visible channel endpoint is present as handler intent but may not be routed.
- Role CRUD endpoints may be missing.
- Theme preferences endpoints may be missing.
- WebSocket envelope in context uses `{ version, type, request_id, payload, sent_at }`, while current backend event code may still emit a legacy tagged shape.

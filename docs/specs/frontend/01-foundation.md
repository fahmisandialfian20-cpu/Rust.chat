# Feature Spec: Frontend Foundation

## Goal

Create a valid SvelteKit 2 + Svelte 5 foundation for the Rust.chat web app/reference client. This foundation must support a real authenticated chat application, admin UI, and future Tauri reuse.

## Scope

This task sets up the project foundation only:

- SvelteKit app entry files under `apps/web/src`.
- TypeScript config.
- Vite config with Tailwind CSS v4 plugin.
- Tailwind v4 CSS-first theme tokens.
- shadcn-svelte-compatible aliases and `cn` utility.
- Zod-backed WebSocket envelope parsing skeleton.
- Full SvelteKit ambient type scaffold in `src/app.d.ts`.
- A minimal first page that points toward auth routes without pretending the app flows are implemented.
- Validation scripts for check/build/test.

## Non-scope

This task does not implement:

- Hoster bootstrap submit flow.
- Login/register submit flow.
- Space/channel/message API calls.
- Admin role/channel editors.
- Full WebSocket connect lifecycle in the layout.

## Web-app foundation requirements

The foundation must assume the frontend will become the full web app:

- Route groups are expected: `(auth)`, `(app)`, and `admin`.
- Authenticated app shell must be possible without replacing the foundation.
- API helpers and stores must be possible under `$lib`.
- UI component system must support forms, app shell, chat, admin panels, settings, and realtime banners.
- Static/Tauri-compatible mode is acceptable, but the architecture must not prevent later SSR migration if product requirements change.

## `src/app.d.ts` ambient contract

Keep an explicit SvelteKit `App` namespace scaffold:

- `App.Error` for stable error code/message shape.
- `App.Locals` for future SSR/hooks auth context if added.
- `App.PageData` for route data typing.
- `App.PageState` for shallow routing/page state.
- `App.Platform` for future adapter/Tauri/platform-specific typing.

Do not remove these just because they are initially empty. They document the intended extension points for later agent coder tasks.

## UX contract

The first page should communicate that Rust.chat is a self-hosted chat platform with server-owned permissions and multi-client support. It should expose clear links for the next auth routes without pretending those flows are implemented yet.

After task 02 begins, this initial page may become a redirect/router decision page according to auth state.

## Technical contract

- Components must use Svelte 5 style, including `$props()` where props are needed.
- Use `$state()` for local mutable component state.
- Use `$derived()` for computed values.
- Do not use deprecated `$app/stores`.
- Do not add `tailwind.config.js`; theme values live in `src/app.css`.
- SvelteKit is configured as an SPA-compatible static build for Tauri reuse unless a later architecture task changes this.
- The WebSocket store must validate event envelopes with Zod before accepting server messages.
- The store must not trust payload permissions; permission UI will be driven by future server DTOs.

## Acceptance criteria

- `apps/web/src` exists with a SvelteKit app shell.
- `src/app.d.ts` contains the full ambient `App` namespace scaffold.
- Tailwind v4 is configured through Vite plugin and CSS `@theme`.
- shadcn-svelte-compatible `components.json` and `cn` utility exist.
- Zod is available for schemas.
- `npm --prefix ./apps/web run check` succeeds.
- `npm --prefix ./apps/web run build` succeeds.
- `npm --prefix ./apps/web test` succeeds.
- `TODO.md` Phase 9 setup task is marked complete only after validation succeeds.

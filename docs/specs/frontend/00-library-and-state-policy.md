# Library and State Policy

This document defines approved frontend libraries and how agent coder should use them for the Rust.chat web app.

## Required stack

### SvelteKit 2 + Svelte 5

Use for routing, layouts, route groups, app shell, and typed route components.

Rules:

- Use Svelte 5 runes in components.
- Use `$props()` for props.
- Use `$state()` for local mutable state.
- Use `$derived()` for computed values.
- Use `$app/state`, not `$app/stores`.
- Keep `src/app.d.ts` as the ambient App namespace scaffold.

### Tailwind CSS v4

Use for styling and design tokens.

Rules:

- Configure via `@tailwindcss/vite`.
- Use `@theme {}` in `src/app.css`.
- Do not add `tailwind.config.js`.
- Avoid arbitrary theme values when a token should exist.

### shadcn-svelte + Bits UI

Use for accessible, reusable UI component patterns.

Rules:

- Use current shadcn-svelte from `shadcn-svelte.com`.
- Keep `components.json` aliases aligned with `$lib` paths.
- Use `cn` helper from `$lib/utils`.
- Once components are copied into the repo, treat them as source code and keep them accessible.

### Zod v4

Use for schemas.

Rules:

- Validate form input before submit.
- Validate API responses before storing/rendering when practical.
- Validate all WebSocket events before feature stores consume them.
- Use `safeParse` for user/API/WS data paths.
- Export inferred types from schemas.

## Current supporting libraries

- `lucide-svelte`: icons only. Do not use icons as the only accessible label.
- `clsx`: conditional class composition.
- `tailwind-merge`: merge Tailwind classes in `cn` helper.
- `tw-animate-css`: animation utility compatible with CSS import flow.
- `@tauri-apps/api`: only for desktop-specific integration behind an adapter. Do not call it in generic web code without browser/Tauri guards.

## Form policy

Default for Phase 9:

- Use client-side Svelte state + Zod validation + typed API helpers.
- Keep forms simple and explicit.
- Use shadcn-svelte/Bits UI form primitives if components are added.

Do not add `sveltekit-superforms` by default because the current frontend is static/Tauri-compatible and does not rely on SvelteKit server actions. If the project switches to SSR/action forms, create a separate architecture decision task first.

## API client policy

Create small domain clients under `apps/web/src/lib/api/`:

- `auth.ts`
- `spaces.ts`
- `channels.ts`
- `messages.ts`
- `roles.ts`
- `theme.ts`

Each helper should:

- use the shared API base URL helper;
- include bearer token/session handling through auth abstraction;
- parse JSON safely;
- normalize stable error shape;
- validate response with Zod when schema exists;
- never log secrets.

## Store policy

Shared stores allowed:

- `auth` for user/token/session state.
- `socket` for one WebSocket connection and reconnect lifecycle.
- `typing` for active channel typing map.
- `presence` for presence map.
- `theme` for local preview and server-backed preferences.

Avoid large global stores for everything. Route data and local component state should remain local where possible.

## Dependency policy

Before adding a new dependency, agent coder must answer:

1. Is it needed for the current single task?
2. Does SvelteKit/Svelte/Tailwind already solve it?
3. Does it work with Svelte 5 and Tailwind v4?
4. Does it compromise static/Tauri-compatible builds?
5. Can it be deferred?

If not clearly needed, do not add it.

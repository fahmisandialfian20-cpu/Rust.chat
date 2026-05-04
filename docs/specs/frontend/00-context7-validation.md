# Context7 Frontend Validation

This document records frontend validation against current documentation surfaced through Context7. Agent coder must read this before implementing Phase 9 frontend tasks.

## Validated source targets

- SvelteKit: `/sveltejs/kit` and `/websites/svelte_dev_kit`
- Svelte: `/sveltejs/svelte`
- Tailwind CSS: `/tailwindlabs/tailwindcss.com`
- shadcn-svelte: `/websites/shadcn-svelte`
- Zod v4: `/websites/zod_dev_v4`

## Validated conclusions

### SvelteKit 2 web-app architecture

- SvelteKit is suitable for a full web application with route groups, layouts, load functions, error boundaries, and typed route data.
- Route groups such as `(auth)` and `(app)` organize layouts without changing public URLs.
- `src/app.d.ts` is the official ambient typing location for the `App` namespace.
- `App.Error`, `App.Locals`, `App.PageData`, `App.PageState`, and `App.Platform` should be kept as an explicit scaffold even if some are empty at first.
- Static adapter with SPA fallback is valid for a web/Tauri reusable client, but it changes data-loading strategy: prefer client-side API calls unless the deployment target changes to SSR/adapter-node.

### Svelte 5

- Components should use Svelte 5 runes.
- Use `$props()` for props and route component data.
- Use `$state()` for local mutable state.
- Use `$derived()` for computed values instead of old reactive assignments.
- Avoid mutating props directly unless using `$bindable()` intentionally.
- Use callback props or local state for child-to-parent communication.

### SvelteKit state imports

- `$app/stores` is deprecated in current SvelteKit 2/Svelte 5 guidance.
- Use `$app/state` for `page`, `navigating`, and `updated` when needed.

### Tailwind CSS v4

- Use `@tailwindcss/vite` in `vite.config.ts`.
- Use `@import "tailwindcss"` in global CSS.
- Use CSS-first `@theme {}` design tokens in `src/app.css`.
- Do not create `tailwind.config.js` for this project.
- Tailwind v4 theme tokens become CSS variables, which is useful for theme settings and runtime UI tokens.

### shadcn-svelte

- Use current shadcn-svelte from `shadcn-svelte.com`, not the Tailwind v3 site.
- Keep `components.json` with aliases for components, utils, hooks, and ui.
- Keep a `cn` utility using `clsx` and `tailwind-merge`.
- shadcn-svelte components are copied into the repo; agent coder should treat them as owned source once added.
- Some shadcn-svelte migration docs still show `tailwind.config.js`; those snippets are not applicable to this Tailwind v4 project. Use CSS tokens instead.

### Zod v4

- Use Zod v4 for form input validation and API/WebSocket payload parsing.
- Prefer `safeParse` for user input, API responses, and WebSocket events so UI can show stable errors without throwing into the render path.
- Export inferred TypeScript types from schemas to keep API helpers and components aligned.

## Decisions for Rust.chat web app

- The frontend is the full web reference client and admin UI for Rust.chat, not a marketing-only landing page.
- The web app must cover auth, lobby, spaces, channels, chat, admin roles, admin channel flags, settings, and realtime state.
- The frontend may cache and render state, but the backend is the source of truth for permissions, channel visibility, feature flags, and message authorization.
- The current foundation can remain static/Tauri-compatible. If SSR/server actions become a product requirement, create a separate architecture-change task before adding `+page.server.ts` dependencies.

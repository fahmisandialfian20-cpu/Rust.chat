# Web App Architecture Guide

Rust.chat frontend Phase 9 is a real web application and reference client for the Rust backend. It is not just a demo UI, landing page, or desktop-only shell.

## Product role

The web app must support:

- first Hoster bootstrap;
- login/register/invite entry;
- lobby;
- space navigation;
- permission-scoped channel navigation;
- realtime text chat;
- typing and presence indicators;
- admin role editor;
- admin channel settings and feature flags;
- user theme settings;
- WebSocket reconnect and event validation;
- acceptance flows that can prove backend contracts.

## Authority boundaries

The web app owns:

- rendering;
- local navigation;
- form state;
- optimistic UI only where safe;
- local cache of server-approved data;
- reconnect UX;
- accessibility and responsive behavior.

The web app does not own:

- final auth decisions;
- permission decisions;
- private channel visibility decisions;
- file/storage authorization;
- LiveKit token generation;
- role enforcement;
- feature flag enforcement.

## Route groups

Recommended route layout:

- `(auth)` for bootstrap/login/register.
- `(app)` for authenticated lobby, spaces, channels, chat, and settings.
- `admin` for admin panels, protected by backend permission context.

Route groups are organizational only and should not affect public URLs.

## Runtime mode

Current foundation is static/Tauri-compatible:

- API calls should happen through typed client-side helpers.
- Use bearer token/session abstraction in the auth store.
- Avoid `+page.server.ts` unless the project explicitly switches to SSR/adapter-node.
- If the TODO mentions `+page.server.ts`, agent coder must reconcile that with this architecture and document the decision.

## Data loading policy

Use this order:

1. Auth store restores token/user state.
2. App shell calls `/api/v1/auth/me` to confirm identity when needed.
3. Route-level components call typed API helpers.
4. API helpers validate responses with Zod.
5. UI renders loading, empty, error, unauthorized, and success states.

Do not display private/permissioned data from stale cache until auth state is confirmed.

## State layers

Use simple layers before adding heavy state libraries:

- Local component state with Svelte runes.
- Shared auth/socket/theme stores for cross-route concerns.
- Route data and component props via SvelteKit/Svelte 5 conventions.
- Optional data query library only after repeated caching/refetch complexity becomes painful and is approved.

## WebSocket policy

- WebSocket is initialized by authenticated app lifecycle, not by individual components repeatedly.
- Socket store owns the single connection and reconnect state.
- Feature stores consume validated normalized events only.
- Invalid or unknown events are rejected without crashing the UI.

## Admin UI policy

Admin screens must be treated as first-class web app surfaces:

- Role editor uses checklist permissions from backend-aligned keys.
- Channel settings uses server feature flag DTOs.
- Admin controls are hidden/disabled based on backend permission context, but backend remains enforcement authority.
- Missing backend endpoints are blockers or disabled UI states, not frontend-success mocks.

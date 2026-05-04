# AGENTS.md - Rust.chat

## Project overview

Rust.chat is a **self-hosted chat application** (Discord/Telegram-like) where the deployer is the highest authority (the **Hoster**).

The Rust backend is the **product core**. Web, desktop, and mobile are clients — all consuming the same server contracts.

## Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust, Axum, Tokio, SQLx |
| Database | PostgreSQL |
| Cache/Real-time | Redis |
| Frontend (web) | SvelteKit, TypeScript, Tailwind CSS v4 |
| Desktop | Tauri (wraps Svelte UI) |
| Mobile | Flutter (Dart) |
| Voice/Video | LiveKit (optional) |
| Storage | Pluggable abstraction (`FileStorage` trait) |

## Context folder

Read `context/` files **in this exact order** before writing any code.
Files prefixed with `_deprecated_` must be **ignored completely**.

1. `context/01-product-requirements.md` — product vision, actors (Hoster, Admin, Member), use cases
2. `context/02-multiclient-architecture.md` — multi-client system design (web + desktop + mobile)
3. `context/03-technology-research.md` — tech decisions and rationale
4. `context/04-domain-model-database.md` — database schema (all tables)
5. `context/05-permissions-rbac.md` — RBAC model, PermissionKey enum, 5-layer check
6. `context/06-api-websocket-contract.md` — REST + WebSocket API contracts
7. `context/07-module-responsibilities.md` — server module structure and responsibilities
8. `context/08-storage-and-media.md` — storage abstraction + media token flow
9. `context/09-client-strategy.md` — client build order (web → desktop → mobile)
10. `context/10-frontend-web-desktop-guide.md` — SvelteKit + Tauri guide
11. `context/11-mobile-flutter-guide.md` — Flutter mobile guide
12. `context/12-operations-runbook.md` — local dev setup, env vars, health checks
13. `context/13-security-observability.md` — security rules, rate limits, audit logs
14. `context/14-agent-execution-plan.md` — implementation phases (Phase 0–11)
15. `context/15-source-map.md` — external references and source links

## Non-negotiable rules

- **Do NOT** build a random chat demo
- **Do NOT** skip permission model
- **Do NOT** trust frontend permission state (validate on backend)
- **Do NOT** allow every logged-in user to see every channel
- **Do NOT** store large files on VPS by default (use pluggable storage)
- **Do NOT** implement WebRTC from scratch (use LiveKit)
- **Do NOT** expose LiveKit API secrets to browser
- **Do NOT** hardcode PikPak as only storage backend
- **Do NOT** design backend as if it belongs to one UI (it must serve web, desktop, mobile)
- **Do NOT** put business logic in handlers (handlers parse input only, logic goes in services)
- **Do NOT** bypass PermissionService

## MVP definition

Complete when:

1. Hoster can bootstrap first account after deployment
2. Hoster can create a space
3. Hoster can create public and private channels
4. Hoster can create roles with checklist-style permissions
5. Members can register through invite links
6. Members can only see channels they are allowed to see
7. Members can exchange realtime text messages
8. Channel feature flags correctly disable actions (file upload, voice, etc.)
9. PostgreSQL stores durable data
10. Redis handles presence, typing indicators, pub/sub, rate limits
11. App can run locally with Docker Compose

## Directory structure

```
apps/
  server/          - Rust Axum backend
  web/             - SvelteKit frontend (reference client + admin UI)
  desktop/         - Tauri desktop shell (placeholder until Phase 4)
  mobile/          - Flutter mobile app (placeholder until Phase 5)

packages/
  api-contracts/   - OpenAPI schema, generated TS/Dart clients, WS event schemas

infra/             - Docker Compose, reverse proxy config, LiveKit config
context/           - This context pack (read-only reference)
```

## Running locally

```bash
# 1. Start infrastructure (Postgres + Redis)
docker compose -f infra/docker-compose.dev.yml up -d

# 2. Run backend
cd apps/server && cargo run

# 3. Run frontend
cd apps/web && pnpm dev
```

## Validated library versions (do NOT deviate)

See `TODO.md` for the full validated `Cargo.toml` and frontend package list.

Critical gotchas that will break the build if ignored:

| Library | Gotcha |
|---------|--------|
| `axum 0.8` | WebSocket route MUST use `routing::any()`, not `get()`. Startup uses `axum::serve()` + `TcpListener`. |
| `jsonwebtoken 10` | MUST add feature `rust_crypto` or `aws_lc_rs` — compile error without it. |
| `argon2 0.5.3` | Use crate named `argon2` (RustCrypto). Do NOT use `rust-argon2` (different crate). |
| `uuid 1.23.1` | Use `Uuid::now_v7()` for all DB primary keys. Use `Uuid::new_v4()` only for tokens. |
| `object_store` | Use `0.12.5`, NOT `0.13.x` (requires Rust 1.85). |
| `sqlx 0.8` | Features: `runtime-tokio` + `tls-rustls` separately. Old `runtime-tokio-native-tls` is deprecated. |
| `Tailwind CSS v4` | No `tailwind.config.js`. All config in `@theme {}` block in CSS. Install via `@tailwindcss/vite`. |
| `SvelteKit + Svelte 5` | Use Runes: `$props()`, `$state()`, `$derived()`. Use `$app/state` not `$app/stores` (deprecated). |
| `shadcn-svelte` | Use Tailwind v4-compatible version from `shadcn-svelte.com`, NOT `tw3.shadcn-svelte.com` (v3). |

## Key files

- `TODO.md` — phase-by-phase task list with acceptance criteria (read this before starting any phase)
- `context/templates/docker-compose.dev.yml` — local dev infrastructure
- `context/templates/.env.example` — all environment variables with defaults
- `context/templates/recommended-repo-layout.md` — full directory layout reference
- `context/manifest.json` — context pack file index

## MCP Servers

This project has the following MCP servers configured and connected.

### Connected MCP Servers

| Name | Type | Status |
|------|------|--------|
| github | local | connected |
| context7 | remote | connected |

### MCP Notes

- **No MCP filesystem needed** - use OpenCode's internal tools (`opencode_file_read`, `opencode_file_list`, etc.)
- MCP servers are accessed by name, e.g., `use github` or `use context7`

## Skills

Skills required for developing this project.

### Process Skills

- **sequential-thinking** - For structured debugging and problem solving
- **brainstorming** - For creative work, creating features, components, or modifications
- **systematic-debugging** - For debugging bugs, test failures, or unexpected behavior
- **verification-before-completion** - For verifying work before claiming completion
- **test-driven-development** - For implementing features or bugfixes

### Database Skills

- `supabase/postgres-best-practices` - PostgreSQL best practices
- `neondatabase/neon-postgres` - Serverless Postgres patterns

### Frontend Skills (SvelteKit)

- `vercel-labs/composition-patterns` - Component composition patterns
- `anthropics/frontend-design` - Frontend design & UI/UX

### Security & Code Quality

- `trailofbits/static-analysis` - CodeQL, Semgrep, SARIF tools
- `trailofbits/property-based-testing` - Property-based testing patterns

### Monitoring

- `getsentry/sentry-workflow` - Fix production issues workflow
- `getsentry/sentry-node-sdk` - Sentry SDK for Rust/Node backend
- `getsentry/sentry-react-sdk` - Sentry SDK for Svelte frontend

### Documentation

- `google-labs-code/design-md` - Creating DESIGN.md files

### Deployment

- `netlify/netlify-deploy` - Deployment automation
- `cloudflare/wrangler` - Cloudflare Workers, Pages, KV, R2

## Notes

- OpenCode internal tools are sufficient for file operations
- MCP github and context7 are already available and connected
- Use skills for structured workflows

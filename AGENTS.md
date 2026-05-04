# AGENTS.md — Rust.chat

Self-hosted chat (Discord-like). Hoster = highest authority per deployment.

## Stack (cheat-sheet)

| Layer | Tech |
|-------|------|
| Backend | Rust, Axum 0.8, Tokio, SQLx 0.8 |
| DB | PostgreSQL 16 |
| Cache / Real-time | Redis 7 (pub/sub, presence, rate limit) |
| Web UI | SvelteKit 2 + Svelte 5 (Runes), Tailwind CSS v4, shadcn-svelte |
| Desktop | Tauri (wraps web UI) |
| Mobile | Flutter |
| Media | LiveKit (optional, Phase 10+) |

## Quick start

### Native workflow (recommended for daily development)

```bash
# 1. Infra (PostgreSQL + Redis only)
docker compose -f infra/docker-compose.dev.yml up -d postgres redis

# 2. Backend
cd apps/server && cargo run

# 3. Frontend
cd apps/web && npm run dev
```

### Full Docker workflow (integration testing)

```bash
# 1. Copy environment file
cp .env.example .env

# 2. Start all services
docker compose -f infra/docker-compose.dev.yml up -d --build
```

## Non-negotiable rules (3-second scan)

1. **Backend validates everything** — never trust frontend permission state.
2. **Handlers are thin** — parse input only; logic lives in services.
3. **PermissionService is the gate** — no endpoint bypasses it.
4. **UUID v7 for DB PKs**, v4 only for tokens.
5. **No raw secrets in responses** — JWT secret, LiveKit key, pepper stay server-side.
6. **No raw invite tokens stored** — store HMAC/hash only.
7. **Use `FileStorage` trait** — don't hardcode a single provider.

## Where details live

| Need | Go to |
|------|-------|
| Active task tracker (local/private, not tracked) | `TODO.md` |
| Tasks, acceptance criteria, infrastructure plan | `context-v2/` |
| Product requirements, actors, MVP scope | `context/01-product-requirements.md` |
| Database schema (all tables) | `context/04-domain-model-database.md` |
| RBAC model, PermissionKey enum, 5-layer check | `context/05-permissions-rbac.md` |
| REST + WebSocket API contracts | `context/06-api-websocket-contract.md` |
| SvelteKit / Tauri / Flutter guides | `context/10-*`, `context/11-*` |
| Security rules, rate limits, audit logs | `context/13-security-observability.md` |
| Env vars, Docker, health checks | `context-v2/01-infrastructure-plan.md` |
| Full context index | `context/manifest.json` |

## Critical gotchas (will break build)

- Axum 0.8 WS route → `routing::any()`, not `get()`.
- `jsonwebtoken` 10 → needs feature `rust_crypto` or `aws_lc_rs`.
- `argon2` → crate `argon2` (RustCrypto), NOT `rust-argon2`.
- `sqlx` 0.8 → features `runtime-tokio` + `tls-rustls` separately.
- Tailwind v4 → no `tailwind.config.js`; config in `@theme {}` inside CSS.
- Svelte 5 → use `$app/state`, not `$app/stores`.

## MCP Servers

| Name | Status |
|------|--------|
| github | connected |
| context7 | connected |

## Skills

### Process
- `brainstorming`, `writing-plans`, `executing-plans`
- `systematic-debugging`, `verification-before-completion`
- `test-driven-development`

### Domain
- `apollographql/rust-best-practices`
- `supabase/postgres-best-practices`, `neondatabase/neon-postgres`
- `redis/redis-development`
- `vercel-labs/composition-patterns`, `anthropics/frontend-design`
- `google-labs-code/shadcn-ui`, `getsentry/sentry-svelte-sdk`
- `flutter/flutter-architecting-apps`
- `trailofbits/static-analysis`, `trailofbits/property-based-testing`
- `getsentry/sentry-workflow`, `getsentry/sentry-node-sdk`
- `google-labs-code/design-md`

## Notes

- OpenCode tools are sufficient for file ops.
- Use skills for structured workflows.
- Deprecated context files (`_deprecated_*`) are ignored.

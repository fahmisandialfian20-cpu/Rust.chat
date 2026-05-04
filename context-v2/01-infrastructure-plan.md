# Infrastructure Plan

## 1. Current Understanding

Rust.chat is a self-hosted chat platform (Discord-like). The Hoster is the highest authority per deployment. The backend is the sole source of truth; no client may bypass it.

The current development environment must support the MVP core:
- PostgreSQL 16 for durable data.
- Redis 7 for cache, sessions, pub/sub, presence, typing, and rate limiting.
- Rust backend (`apps/server`) built with Axum 0.8, SQLx 0.8, Redis 0.27, Tokio.
- Web frontend (`apps/web`) built with SvelteKit 2, Svelte 5 (Runes), Tailwind CSS v4.
- Optional LiveKit placeholder in configuration, but disabled by default (`LIVEKIT_ENABLED=false`).

The developer workflow described in `AGENTS.md` runs infrastructure services (PostgreSQL, Redis) via Docker Compose, while the backend and frontend are run natively (`cargo run`, `npm run dev`). The existing `infra/docker-compose.dev.yml` also defines `server` and `web` services, which creates ambiguity about whether the intended dev workflow is fully native or fully containerized.

The existing backend already exposes `/healthz` (liveness) and `/readyz` (DB + Redis connectivity). These should be reused for infrastructure health checks.

## 2. Current Infrastructure Files Found

| File | Purpose | Notes |
|------|---------|-------|
| `infra/docker-compose.dev.yml` | Dev compose | Defines `postgres`, `redis`, `server`, `livekit`, `web`. Build contexts and `.env` paths assume execution from `infra/`. |
| `context/templates/docker-compose.dev.yml` | Template compose | Only `postgres` and `redis`; `server` is commented out. Suggests an earlier intent to run backend natively. |
| `infra/nginx.conf` | Nginx config for web container | Static SPA fallback for SvelteKit (`try_files`). |
| `infra/livekit.yaml` | LiveKit dev config | Inactive by default. |
| `.env.example` (root) | Root env template | Contains core variables but is less documented than `context/templates/.env.example`. |
| `context/templates/.env.example` | Detailed env template | Better comments; includes commented-out cloud storage vars (S3, GCS, Azure) and PikPak adapter. |
| `apps/server/Dockerfile` | Backend container image | Multi-stage: `rust:1.85-slim-bookworm` builder, `debian:bookworm-slim` runtime. Copies `migrations/`. |
| `apps/web/Dockerfile` | Frontend container image | Multi-stage: `node:22-alpine` builder, `nginx:alpine` runtime. Expects build context at repo root. |

## 3. MVP Infrastructure Boundary

The MVP development environment must provide:

1. **PostgreSQL** container with persistent named volume.
2. **Redis** container (no persistence required for dev; data is ephemeral).
3. **Local upload volume** mounted into the backend when running via Docker.
4. **Environment variable documentation** (`.env.example`) that is complete and tracked.
5. **Docker Compose** file(s) that are reproducible from a fresh clone.
6. **Health checks** for PostgreSQL, Redis, backend, and frontend reachability.
7. **Clear ports** documented for local development.
8. **Optional LiveKit** must remain present but clearly marked as inactive and not required for MVP.

The backend and frontend may be run **natively** for day-to-day development, or **via Docker Compose** for integration verification. The infrastructure must support both without conflict.

## 4. Non-Goals

These must not be worked on unless a future issue explicitly requests them:

- Production Kubernetes or Terraform.
- Cloud provider automation (S3/R2/GCS/Azure integration beyond placeholder env vars).
- Mobile build infrastructure or Desktop release pipeline.
- LiveKit production deployment or TURN tuning.
- CI/CD deployment automation.
- Monitoring stack (Prometheus/Grafana).
- Sentry integration.
- Cloudflare Tunnel / Nginx Proxy Manager automation.
- Full production security hardening or automated backup/restore.

## 5. Required Services

| Service | Image / Build | Role | MVP |
|---------|---------------|------|-----|
| `postgres` | `postgres:16` | Durable relational data | **Yes** |
| `redis` | `redis:7` | Cache, sessions, pub/sub, presence, rate limits | **Yes** |
| `server` | `apps/server/Dockerfile` | Rust backend API + WebSocket gateway | **Yes** (native by default; container optional) |
| `web` | `apps/web/Dockerfile` | SvelteKit web UI served by nginx | **Yes** (native by default; container optional) |
| `livekit` | `livekit/livekit-server:latest` | Voice/video (Phase 10+) | No — disabled by default |

## 6. Service Network Design

- **Network mode:** Docker Compose default bridge network (`rust-chat_default` or explicit named network).
- **Internal communication:**
  - `server` → `postgres` on port `5432`
  - `server` → `redis` on port `6379`
  - `web` (browser) → `server` on port `8080`
  - `web` (browser) → `web` (nginx) on port `3000`
- **Port bindings (host):**
  - `5432` → PostgreSQL (dev only; must not be exposed in production patterns)
  - `6379` → Redis (dev only)
  - `8080` → Backend API / WebSocket
  - `3000` → Web frontend (nginx)
  - `7880-7881` → LiveKit (only if explicitly enabled)
- **Security rule:** PostgreSQL and Redis must only be reachable from the backend within the Docker network. Host port bindings are acceptable for local development.

## 7. Environment Variable Strategy

### Tracked documentation
- The **root** `.env.example` must be the canonical source. `context/templates/.env.example` is more detailed and should be merged into the root file. `context/templates/.env.example` can then be removed or kept as a historical reference.

### Required variables for local development

```text
# Application
APP_ENV=development
APP_PUBLIC_URL=http://localhost:3000
API_PUBLIC_URL=http://localhost:8080

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Database & Cache
DATABASE_URL=postgres://chatapp:chatapp@localhost:5432/chatapp
REDIS_URL=redis://localhost:6379

# Auth secrets (development placeholders — must be changed for production)
SESSION_SECRET=change-me-use-at-least-64-random-characters-here-do-not-use-this-default
PASSWORD_PEPPER=change-me-use-a-different-random-string-here-do-not-use-this-default
JWT_SECRET=change-me-use-at-least-64-random-characters-for-jwt-signing-do-not-use-this
JWT_ACCESS_TTL_SECONDS=900
JWT_REFRESH_TTL_SECONDS=2592000

# Storage
STORAGE_PROVIDER=local
LOCAL_STORAGE_DIR=/data/uploads

# LiveKit (optional, disabled)
LIVEKIT_ENABLED=false
LIVEKIT_URL=ws://localhost:7880
LIVEKIT_API_KEY=devkey
LIVEKIT_API_SECRET=secret

# Rate limits
RATE_LIMIT_LOGIN=5
RATE_LIMIT_REGISTER=5
RATE_LIMIT_MESSAGE_SEND=30
RATE_LIMIT_FILE_UPLOAD=10
RATE_LIMIT_WS_CONNECT=20

# Logging
RUST_LOG=info
LOG_FORMAT=pretty
```

### Development vs Docker-internal URLs
- When running **natively** (`cargo run`, `npm run dev`), use `localhost` in `DATABASE_URL` and `REDIS_URL`.
- When running **inside Docker Compose**, override via compose `environment` or an `env_file` to use service hostnames (`postgres`, `redis`).
- The root `.env.example` should default to `localhost` because the primary dev workflow is native. Docker Compose can inject overrides.

## 8. Volumes and Persistence

| Volume | Service | Purpose | Notes |
|--------|---------|---------|-------|
| `pg_data` | `postgres` | PostgreSQL data directory | Named volume, survives `docker compose down`. |
| `upload_data` | `server` | Local file uploads (`LOCAL_STORAGE_DIR`) | Required when `STORAGE_PROVIDER=local` and server runs in container. |

No other persistence is required for MVP development. Redis runs without persistence (`appendonly no` default), which is acceptable for dev.

## 9. Health Checks

### Existing application health endpoints
- `GET /healthz` — Liveness. Returns `200 OK` immediately.
- `GET /readyz` — Readiness. Returns `200 OK` only if PostgreSQL and Redis are reachable; otherwise `503 Service Unavailable`.

### Infrastructure health checks to add
- **PostgreSQL** — `pg_isready -U chatapp` (Compose `healthcheck`).
- **Redis** — `redis-cli ping` expecting `PONG` (Compose `healthcheck`).
- **Backend** — `curl -f http://localhost:8080/healthz` or `http://localhost:8080/readyz`.
- **Web (nginx)** — `curl -f http://localhost:3000`.

### Compose `depends_on` strategy
- `server` must `depends_on` `postgres` and `redis` with `condition: service_healthy` (requires Docker Compose v2.20+ or v3 format with explicit health conditions).
- `web` must `depends_on` `server` at least with `condition: service_started`.

## 10. Local Development Workflow

### Primary workflow (recommended for day-to-day)

```bash
# 1. Start infrastructure only
docker compose -f infra/docker-compose.dev.yml up -d postgres redis

# 2. Run backend natively
cd apps/server && cargo run

# 3. Run frontend natively
cd apps/web && npm run dev
```

### Optional fully-containerized workflow (for integration checks)

```bash
# 1. Create local environment file
cp .env.example .env
# edit .env and replace placeholder secrets

# 2. Start everything including server and web
docker compose -f infra/docker-compose.dev.yml up -d --build
```

### Rationale
- Native builds are faster for Rust and Node.js during active development (incremental compilation, HMR).
- Docker Compose is the source of truth for PostgreSQL and Redis versions/configuration.
- The compose file must remain usable for both modes: infrastructure-only and full-stack.

### Package manager
- **npm** is the chosen package manager for the web frontend.
- `package-lock.json` must be committed to ensure reproducible installs.
- Do not mix npm with pnpm or yarn in the same project.

## 11. Security Notes for Development vs Production

### Development
- Simple credentials (`chatapp`/`chatapp`) are acceptable.
- Host port binding for PostgreSQL (`5432`) and Redis (`6379`) is acceptable for local tools (pgAdmin, Redis Insight, `sqlx-cli`).
- `.env.example` contains placeholder secrets. The file header must clearly warn: **"NEVER use these defaults in production."**

### Production (documented, not implemented)
- PostgreSQL and Redis must not be exposed to the public internet.
- Secrets (`SESSION_SECRET`, `PASSWORD_PEPPER`, `JWT_SECRET`, `LIVEKIT_API_SECRET`) must be generated with a cryptographically secure random source (minimum 64 characters).
- `.env.example` is **not** production configuration.
- Cloud storage credentials (S3, GCS, Azure) must never be committed.
- Clients must never communicate directly with PostgreSQL or Redis.

## 12. Known Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| **Build context mismatch** | `infra/docker-compose.dev.yml` is designed to run from `infra/`, but `web` build context is `..` and `server` uses `../.env`. This is inconsistent and confusing. | Document the intended working directory or restructure paths to be runnable from repo root. |
| **localhost vs Docker hostnames** | `.env.example` uses `localhost` for DB/Redis. If a developer runs `docker compose up server`, the container cannot reach `localhost:5432` unless using `host.docker.internal`. | Compose must override `DATABASE_URL` and `REDIS_URL` to use service names (`postgres`, `redis`). |
| **No compose health checks** | Services may start in wrong order, causing backend crash loops on first boot. | Add `healthcheck` blocks to `postgres` and `redis`, and use `depends_on` with conditions. |
| **Root `.env.example` less detailed than template** | Developers may miss cloud storage placeholder documentation. | Merge `context/templates/.env.example` into root `.env.example` and add clear section comments. |
| **LiveKit present but inactive** | `livekit` service runs in compose even when `LIVEKIT_ENABLED=false`. This consumes resources and may confuse new developers. | Keep LiveKit in compose but clearly comment it out or document that it is optional and disabled by default. |
| **Web Dockerfile expects repo-root context** | `apps/web/Dockerfile` references `apps/web/package.json`. It cannot be built with `apps/web/` as context. | Ensure compose `build.context` is always repo root for the `web` service. |

## 13. Required Changes

### A. Merge and update `.env.example`
1. Replace root `.env.example` with the more detailed content from `context/templates/.env.example`.
2. Ensure defaults use `localhost` for native dev.
3. Keep cloud storage placeholders commented out.
4. Remove or deprecate `context/templates/.env.example` to avoid duplication.

### B. Refactor `infra/docker-compose.dev.yml`
1. Add explicit named network (optional but recommended).
2. Add `healthcheck` to `postgres`:
   ```yaml
   healthcheck:
     test: ["CMD-SHELL", "pg_isready -U chatapp"]
     interval: 5s
     timeout: 5s
     retries: 5
   ```
3. Add `healthcheck` to `redis`:
   ```yaml
   healthcheck:
     test: ["CMD", "redis-cli", "ping"]
     interval: 5s
     timeout: 5s
     retries: 5
   ```
4. Update `server` service:
   - Use `depends_on` with `condition: service_healthy` for `postgres` and `redis`.
   - Override `DATABASE_URL` and `REDIS_URL` via `environment` to use Docker service names.
   - Ensure `volumes` mounts `upload_data` to `/data/uploads`.
5. Update `web` service:
   - Set `build.context: .` and `build.dockerfile: apps/web/Dockerfile` so compose can be run from repo root.
   - Or document clearly that `web` build requires repo-root context.
6. Handle LiveKit:
   - Either comment out the `livekit` service with a clear note, or keep it running but document that it is optional and disabled by the application.

### C. Consider repo-root compose execution
- The most natural workflow is `docker compose -f infra/docker-compose.dev.yml up -d` run from the **repo root**.
- If so, adjust all relative paths in `infra/docker-compose.dev.yml` (e.g., `.env` instead of `../.env`, build contexts relative to repo root).

### D. Add `infra/docker-compose.full.yml` (optional future)
- Not required now. Mention in plan that a future `full.yml` can include LiveKit, PikPak, and other optional services so `dev.yml` stays minimal.

### E. Add verification script or documentation
- Document exact commands to verify each service is healthy after `docker compose up`.

## 14. Acceptance Criteria

The infrastructure plan is acceptable when:

- [ ] The plan focuses strictly on MVP development infrastructure.
- [ ] It does not introduce production/cloud automation.
- [ ] It identifies all current infrastructure files in the repo.
- [ ] It defines required services (`postgres`, `redis`, `server`, `web`) and their responsibilities.
- [ ] It defines the environment variable strategy (root `.env.example`, localhost defaults, Docker overrides).
- [ ] It defines volume and persistence strategy (`pg_data`, `upload_data`).
- [ ] It defines health checks (`/healthz`, `/readyz`, compose-level checks).
- [ ] It defines verification commands for developers.
- [ ] It clearly separates development defaults from production requirements.
- [ ] It explicitly postpones LiveKit production work, mobile, desktop, and advanced storage.
- [ ] It gives a clear next implementation sequence (see Section 13).

## 15. Verification Commands

### Configuration validation
```bash
docker compose -f infra/docker-compose.dev.yml config
```

### Start infrastructure services
```bash
docker compose -f infra/docker-compose.dev.yml up -d postgres redis
```

### Check container health
```bash
docker compose -f infra/docker-compose.dev.yml ps
docker compose -f infra/docker-compose.dev.yml logs postgres
docker compose -f infra/docker-compose.dev.yml logs redis
```

### Backend checks (run natively)
```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

### Frontend checks (run natively)
```bash
cd apps/web
npm install
npm run check
npm run build
```

### Health endpoint checks (after backend is running)
```bash
curl http://localhost:8080/healthz
curl http://localhost:8080/readyz
```

### Optional full containerized stack
```bash
docker compose -f infra/docker-compose.dev.yml up -d --build
```

> **Note:** The agent must report results when running these commands. If a command cannot run (e.g., missing `pnpm`), the agent must explain why.

## 16. Future Infrastructure Work

Planned for post-MVP phases:

- **Production deployment patterns:** Kubernetes manifests, Terraform modules, reverse proxy (Traefik/Nginx) automation.
- **Cloud storage integration:** S3-compatible (R2, MinIO), GCS, Azure Blob. The `object_store` crate is already a dependency; only configuration is needed.
- **LiveKit production:** TURN/STUN, TLS termination, automated provisioning.
- **CI/CD:** GitHub Actions for `cargo test`, `cargo clippy`, `npm run check`, `npm run build`, Docker image builds.
- **Observability:** Structured logging (`LOG_FORMAT=json`), Prometheus metrics, Grafana dashboards, Sentry error tracking.
- **Security hardening:** Network policies, secret management (Vault/Sealed Secrets), automated CVE scanning.
- **Backup/restore:** PostgreSQL `pg_dump`/`pg_restore` automation, upload volume snapshots.
- **Desktop/Mobile release pipelines:** Tauri build artifacts, Flutter APK/TestFlight distribution.

---

*This plan was created as part of the infrastructure planning phase. The infrastructure files in this branch have already been updated to reflect this plan.*

# Rust.chat Infrastructure Agent Brief

## Purpose

This is the active context for the agent that will work on the infrastructure side of Rust.chat.

The previous context pack is broad and includes future product areas such as mobile, desktop, LiveKit, advanced storage, audit dashboards, and production hardening. This document narrows the work to infrastructure planning only.

The next agent must not implement application features yet.

The next agent must use this context, the existing repository, Context7, and available planning/design skills to create a clear infrastructure plan/spec before changing infrastructure files.

Recommended output from the next agent:

```text
context-v2/01-infrastructure-plan.md
```

Do not write application code as part of this step.

---

## Project Summary

Rust.chat is a self-hosted chat platform inspired by Discord and Telegram.

The application is intended to be deployed by a Hoster, who becomes the highest authority in the instance.

The backend is the source of truth. All clients must communicate with the backend API and WebSocket gateway. No client may bypass backend permissions.

The project will eventually support:

- Web client
- Desktop client
- Mobile client
- PostgreSQL
- Redis
- Realtime WebSocket messaging
- File storage
- Optional LiveKit voice/video
- Future production deployment patterns

The current infrastructure focus is not to build everything at once. The current focus is to make the local and development infrastructure clear, reproducible, and safe for the MVP core.

---

## Infrastructure Scope for the Current Stage

The current infrastructure work should focus on the MVP development environment only.

Target infrastructure components:

1. PostgreSQL for durable data
2. Redis for cache, sessions, pub/sub, presence, typing, and rate limits
3. Rust backend service
4. Web frontend service
5. Local upload volume for development only
6. Optional LiveKit placeholder/config only if already present, but not as an active feature target
7. `.env.example` and environment variable documentation
8. Docker Compose development workflow
9. Basic health checks and verification commands
10. Clear separation between development defaults and production requirements

---

## Explicit Non-Goals

Do not work on these yet unless a future issue explicitly asks for them:

- Production Kubernetes
- Production Terraform
- Cloud provider automation
- Mobile build infrastructure
- Desktop release pipeline
- LiveKit production deployment
- TURN server production tuning
- S3/PikPak/R2 storage integration
- CI/CD deployment automation
- Monitoring stack such as Prometheus/Grafana
- Sentry integration
- Cloudflare Tunnel automation
- Nginx Proxy Manager automation
- Full production security hardening
- Backup/restore automation beyond documenting future requirements

These may be mentioned as future work, but they must not distract from the current MVP development infrastructure.

---

## Required Agent Behavior

The next agent must start by creating a plan/spec.

The agent should read:

1. This file
2. `AGENTS.md`
3. Existing infrastructure files such as `infra/`, `.env.example`, Dockerfiles, and compose files
4. Existing application layout only as needed to understand service names and build contexts
5. Context7 documentation for validated Docker Compose, PostgreSQL, Redis, Rust, Node/SvelteKit, and related tooling when needed

The agent must not read the entire old context pack unless necessary.

The agent must not start by editing Docker Compose directly.

The first output must be a planning/spec document.

---

## Required Output: Infrastructure Plan

Create:

```text
context-v2/01-infrastructure-plan.md
```

The plan must include:

```md
# Infrastructure Plan

## 1. Current Understanding
## 2. Current Infrastructure Files Found
## 3. MVP Infrastructure Boundary
## 4. Non-Goals
## 5. Required Services
## 6. Service Network Design
## 7. Environment Variable Strategy
## 8. Volumes and Persistence
## 9. Health Checks
## 10. Local Development Workflow
## 11. Security Notes for Development vs Production
## 12. Known Risks
## 13. Required Changes
## 14. Acceptance Criteria
## 15. Verification Commands
## 16. Future Infrastructure Work
```

The plan must be measurable and limited to the MVP development infrastructure.

The agent must not implement the plan until it has been reviewed.

---

## Development Infrastructure Principles

### 1. Keep development simple

The MVP development environment should be easy to run from a fresh clone.

A developer should be able to start required infrastructure with a small number of commands.

### 2. Avoid production pretending

Development defaults may use simple credentials, local ports, and local volumes.

Production requirements must be documented separately and must not reuse development secrets.

### 3. Do not hide required configuration

Every required environment variable must appear in `.env.example` or equivalent tracked documentation.

### 4. Do not rely on private local files for fresh clones

Private local files such as `.env` or local-only TODO files may exist, but the tracked repository must still contain enough infrastructure context for a new agent or developer to understand the expected setup.

### 5. Infrastructure must support the backend as the source of truth

Infrastructure must expose the backend API and WebSocket gateway in a way that supports:

- Authentication
- Permission checks
- Message REST routes
- WebSocket realtime routes
- PostgreSQL access from backend only
- Redis access from backend only

PostgreSQL and Redis should not be treated as public application APIs.

---

## Expected MVP Services

The local MVP infrastructure should eventually support these services:

```text
postgres
redis
server
web
```

Optional or future services:

```text
livekit
reverse-proxy
object-storage
worker
```

LiveKit may remain in the repository if already present, but it must not become the active focus until the core chat MVP is stable.

---

## Docker Compose Direction

The Docker Compose setup should be understandable and reproducible.

The plan should evaluate whether the current compose files should be split into:

```text
infra/docker-compose.dev.yml
infra/docker-compose.full.yml
```

or kept as one development file.

Recommended direction:

- `docker-compose.dev.yml` should prioritize the core MVP: PostgreSQL, Redis, backend, web.
- Future optional services should be clearly marked and should not break the core MVP when disabled.
- Development ports should be documented.
- Volumes should be named and understandable.

---

## Environment Variable Direction

The infrastructure plan must define which variables are required for local development.

At minimum, the plan should cover:

```text
SERVER_HOST
SERVER_PORT
DATABASE_URL
REDIS_URL
SESSION_SECRET
PASSWORD_PEPPER
JWT_SECRET
JWT_ACCESS_TTL_SECONDS
JWT_REFRESH_TTL_SECONDS
STORAGE_PROVIDER
LOCAL_STORAGE_DIR
RUST_LOG
```

If frontend is included:

```text
PUBLIC_API_BASE_URL
PUBLIC_WS_URL
```

If LiveKit remains present but inactive:

```text
LIVEKIT_ENABLED
LIVEKIT_URL
LIVEKIT_API_KEY
LIVEKIT_API_SECRET
```

Development placeholders are allowed in `.env.example`, but the file must clearly state that production must replace them.

---

## Security Boundary for Infrastructure Work

The infrastructure agent must not weaken application security.

Rules:

- Do not expose PostgreSQL publicly for production patterns.
- Do not expose Redis publicly for production patterns.
- Do not commit real secrets.
- Do not treat `.env.example` as production config.
- Do not hardcode production secrets in compose files.
- Do not make frontend communicate directly with PostgreSQL or Redis.
- Do not bypass backend APIs.
- Do not design infrastructure that requires clients to know internal service secrets.

---

## Health Check Direction

The infrastructure plan should define health checks for development.

Expected checks:

- PostgreSQL is accepting connections.
- Redis is accepting connections.
- Backend health endpoint responds.
- Web frontend is reachable.
- Backend can connect to PostgreSQL and Redis.

The plan should identify existing health endpoints before inventing new ones.

---

## Verification Commands

The infrastructure plan must require commands similar to:

```bash
docker compose -f infra/docker-compose.dev.yml config
```

If services are intended to run via compose:

```bash
docker compose -f infra/docker-compose.dev.yml up -d
```

Backend checks:

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Frontend checks if web infrastructure or frontend build files are changed:

```bash
cd apps/web
npm ci
npm run check
npm run build
```

The agent must report command results.

If a command cannot be run, the agent must explain why.

---

## Acceptance Criteria for the Plan

The infrastructure plan is acceptable only if:

- It focuses on MVP development infrastructure.
- It does not start production/cloud automation.
- It identifies current infrastructure files in the repo.
- It defines required services and their responsibilities.
- It defines environment variable strategy.
- It defines volume and persistence strategy.
- It defines health checks.
- It defines verification commands.
- It separates development defaults from production requirements.
- It explicitly postpones LiveKit production work, mobile, desktop, and advanced storage.
- It gives a clear next implementation sequence.

---

## Required Next Step

Using this document, create:

```text
context-v2/01-infrastructure-plan.md
```

Do not implement infrastructure changes yet.

Do not change application code.

Do not add future production infrastructure.

The next output should be a clear, measurable, staged infrastructure plan for the MVP development environment.

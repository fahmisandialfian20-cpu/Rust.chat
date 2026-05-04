# 05 — Storage and Infrastructure

## Infrastructure Goal

The infrastructure must support MVP development first.

Current focus:

- PostgreSQL
- Redis
- Rust backend
- Svelte web client
- local upload storage for development
- Docker Compose for development

Do not build production cloud automation during MVP core stabilization.

---

## PostgreSQL

PostgreSQL is the durable source of truth.

Use PostgreSQL for:

- users
- sessions / devices when durable
- spaces
- memberships
- roles
- permissions
- channels
- invites
- messages
- file metadata

PostgreSQL is preferred for this project because it works well with relational permissions, SQLx, constraints, migrations, and future JSON fields.

Do not switch to MariaDB/MySQL unless a dedicated database decision task is created.

### Alternatives

#### Supabase (Cloud PostgreSQL)

Supabase is a valid alternative to local PostgreSQL. It provides managed PostgreSQL with SSL.

**How to use:**
1. Get connection string from Supabase Dashboard → Settings → Database → Connection string
2. Format: `postgresql://postgres:<password>@db.<project-ref>.supabase.co:5432/postgres`
3. Update `DATABASE_URL` and `TEST_DATABASE_URL` in `.env`

**⚠️ Warning for testing:**
- Tests run `DROP SCHEMA public CASCADE` on every test run
- Use a dedicated Supabase project for development/testing
- Do NOT use production database for tests

#### Local PostgreSQL (Docker or native)

Standard Docker Compose workflow (see Local Development Infrastructure below).

---

## Redis

Redis is for temporary and real-time support.

Use Redis for:

- cache
- sessions when appropriate
- presence
- typing indicators
- pub/sub
- rate limits
- short-lived WebSocket state

Redis is not the durable source of truth for messages, spaces, roles, or permissions.

### Running Redis Without Docker

If Docker is not available, Redis can run through:

- **WSL2** (recommended for Windows): Install Redis in WSL2, expose port 6379
- **Native Windows**: Redis for Windows (unofficial, limited)
- **Upstash Redis** (cloud): `REDIS_URL=rediss://default:<token>@<host>`

---

## File Storage Direction

The VPS may have limited disk.

Large files should eventually be stored outside the VPS or through a pluggable storage provider.

Possible future providers:

- local filesystem for development
- S3-compatible storage
- Cloudflare R2
- PikPak adapter using `Quan666/PikPakAPI`
- other providers

MVP rule:

```text
Use a storage abstraction. Do not hardcode one provider.
```

PikPak can be explored later as an adapter/provider, but it must not be deeply coupled into chat, auth, users, avatars, or messages.

---

## Local Development Infrastructure

Recommended native workflow:

```bash
docker compose -f infra/docker-compose.dev.yml up -d postgres redis
cd apps/server && cargo run
cd apps/web && npm run dev
```

Full Docker integration workflow:

```bash
cp .env.example .env
docker compose -f infra/docker-compose.dev.yml up -d --build
```

---

## Environment Variables

`.env.example` must document all required variables.

`.env` is local/private and must not be committed.

Development defaults are acceptable in `.env.example`, but production must replace secrets.

Important variables:

```text
SERVER_HOST
SERVER_PORT
DATABASE_URL
TEST_DATABASE_URL          # Required for cargo test (same as DATABASE_URL or separate test DB)
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

Frontend variables:

```text
PUBLIC_API_BASE_URL
PUBLIC_WS_URL
```

Future LiveKit variables:

```text
LIVEKIT_ENABLED
LIVEKIT_URL
LIVEKIT_API_KEY
LIVEKIT_API_SECRET
```

---

## Development Security Rules

1. Do not expose PostgreSQL as a public production API.
2. Do not expose Redis as a public production API.
3. Do not commit real secrets.
4. Do not put provider secrets in frontend code.
5. Do not let clients talk directly to storage providers with server secrets.
6. Backend must issue safe access URLs/tokens when needed.

---

## Health Checks

Development infrastructure should support:

- PostgreSQL health check
- Redis health check
- backend `/healthz`
- backend `/readyz`
- frontend reachability

---

## Current Non-Goals

Do not work on these unless a task explicitly asks for them:

- Kubernetes
- Terraform
- production reverse proxy automation
- Cloudflare Tunnel automation
- Nginx Proxy Manager automation
- Prometheus/Grafana
- Sentry
- backup automation
- PikPak implementation
- S3/R2 implementation
- LiveKit production deployment

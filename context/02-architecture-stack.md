# 02 — Architecture and Stack

## Architecture Direction

Rust.chat uses a backend-first architecture.

```text
Web / Desktop / Mobile Client
        ↓
Backend API + WebSocket Gateway
        ↓
PostgreSQL + Redis + Storage Providers
```

The backend owns authentication, permissions, business rules, and durable state.

Clients render UI and call backend contracts.

Clients must not talk directly to PostgreSQL, Redis, or storage provider secrets.

---

## Current Stack

### Backend

Current direction:

- Rust
- Axum
- Tokio
- SQLx
- Serde
- JWT/session auth
- WebSocket support

Rust is appropriate because the backend needs strong correctness, good concurrency, and a reliable type system.

### Frontend

Current direction:

- SvelteKit
- TypeScript
- Tailwind CSS

Svelte is acceptable for the web client if the project keeps UI simple and avoids framework overengineering.

### Database

Current direction:

- PostgreSQL

PostgreSQL is a good fit because the app needs relational data:

- users
- sessions
- spaces
- members
- roles
- permissions
- channels
- invites
- messages
- files

MariaDB/MySQL can also store relational data, but PostgreSQL is a strong default for this project because of SQLx support, constraints, JSON support, migrations, and complex permission queries.

Do not switch database unless a task explicitly asks for a database decision review.

### Cache / Realtime Support

Current direction:

- Redis

Redis is used for:

- sessions/cache
- presence
- typing indicators
- pub/sub
- rate limits
- short-lived state

Redis is not the durable source of truth for messages or permissions.

### File Storage

Current direction:

- `FileStorage` abstraction
- local storage for development
- pluggable providers later

Possible future providers:

- S3-compatible storage
- Cloudflare R2
- PikPak adapter
- other remote storage

Do not hardcode PikPak as the only provider.

### Voice / Video

Future direction:

- LiveKit

LiveKit is not part of the current MVP core stabilization.

---

## Client Direction

### Web

Web is the first client and reference UI.

It should support MVP flows first:

- login/register
- lobby
- spaces
- channel list
- messages
- role/channel admin later

### Desktop

Desktop should come later, likely by wrapping the web UI with Tauri.

Do not start desktop until API contracts and web MVP are stable.

### Mobile

Mobile should come later after API contracts are stable.

Flutter is acceptable as a future direction.

Do not start mobile during backend MVP stabilization.

---

## Infrastructure Direction

Local development uses:

- PostgreSQL container
- Redis container
- backend native or containerized
- frontend native or containerized

Preferred daily workflow:

```bash
docker compose -f infra/docker-compose.dev.yml up -d postgres redis
cd apps/server && cargo run
cd apps/web && npm run dev
```

Full Docker is for integration checks.

---

## Important Architecture Rules

1. Backend is the source of truth.
2. PostgreSQL stores durable data.
3. Redis stores temporary/cache/realtime state.
4. Permissions are checked on the backend.
5. WebSocket events use the same permission rules as REST.
6. Storage providers are behind backend abstractions.
7. LiveKit secrets must never go to the browser.
8. Clients must not know database, Redis, or provider secrets.

---

## Current Technical Uncertainties

These can be reviewed later, but must not block MVP core:

- final storage provider for large files
- final native desktop packaging
- final mobile framework details
- final production deployment topology
- final UI theme system

For now, prioritize backend correctness and MVP flow.

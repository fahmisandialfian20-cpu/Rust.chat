# AGENTS.md - Rust.chat

## Project overview

Rust.chat is a **self-hosted chat application** (Discord/Telegram-like) where the deployer is the highest authority (the **Hoster**).

## Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust, Axum, Tokio, SQLx |
| Database | PostgreSQL |
| Cache/Real-time | Redis |
| Frontend | SvelteKit, TypeScript, Tailwind CSS |
| Voice/Video | LiveKit (optional) |
| Storage | Pluggable abstraction |

## Context folder

Read `context/` files in order:

1. `context/01-product-requirements.md` - product vision, actors (Hoster, Admin, Member), use cases
2. `context/02-architecture.md` - system design
3. `context/03-technology-research.md` - tech decisions
4. `context/04-domain-model-database.md` - database schema
5. `context/05-permissions-rbac.md` - RBAC model
6. `context/06-api-websocket-contract.md` - API contracts
7. `context/12-agent-execution-plan.md` - implementation phases

## Non-negotiable rules

- **Do NOT** build a random chat demo
- **Do NOT** skip permission model
- **Do NOT** trust frontend permission state (validate on backend)
- **Do NOT** allow every logged-in user to see every channel
- **Do NOT** store large files on VPS by default (use pluggable storage)
- **Do NOT** implement WebRTC from scratch (use LiveKit)
- **Do NOT** expose LiveKit API secrets to browser
- **Do NOT** hardcode PikPak as only storage backend

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
apps/server    - Rust backend
apps/web     - SvelteKit frontend
infra       - Docker Compose, deployment config
context     - This context pack
```

## Running locally

```bash
# Start infrastructure (Postgres + Redis)
docker compose -f infra/docker-compose.dev.yml up -d

# Run backend
cd apps/server && cargo run

# Run frontend
cd apps/web && npm run dev
```

## Key files

- `context/templates/docker-compose.dev.yml` - local dev environment
- `context/templates/.env.example` - environment variables template
- `context/templates/Cargo.toml.recommended.md` - Rust dependencies
- `context/manifest.json` - source map links
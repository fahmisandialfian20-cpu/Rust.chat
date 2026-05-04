# AGENTS.md — Rust.chat Agent Router

Rust.chat is a self-hosted chat application inspired by Discord and Telegram.

The deployer is the **Hoster** and is the highest authority in the instance.

This file is only a router. Do not turn this file into a long project encyclopedia.

---

## Mandatory Reading Order

For every task, read only:

1. `context/00-project-overview.md`
2. `context/01-product-scope.md`
3. The one task-specific context file requested by the user or issue

If no task-specific file is provided, ask for one or create a plan/spec first.

Do **not** load the entire `context/` folder.

Do **not** read archived, deprecated, or old reference files unless the task explicitly asks for them.

---

## Canonical Context Files

The active core context is exactly these 7 files:

```text
context/00-project-overview.md
context/01-product-scope.md
context/02-architecture-stack.md
context/03-domain-permissions.md
context/04-client-ui-experience.md
context/05-storage-infrastructure.md
context/06-agent-workflow.md
```

Anything outside these files is reference-only unless the active task says otherwise.

---

## Current Development Direction

Current active direction:

```text
MVP Core Stabilization
```

Work should be split into small task contexts, for example:

```text
auth
rbac
channel-visibility
message-permissions
websocket-mvp
infrastructure
frontend-shell
```

Do not implement the entire application in one pass.

Do not jump to future phases.

---

## Non-Negotiable Rules

1. Backend is the source of truth.
2. Frontend permission state is UI convenience only, not security.
3. Protected backend actions must validate authenticated user context.
4. Protected backend actions must use `PermissionService` or equivalent service-level permission checks.
5. Handlers must stay thin; business logic belongs in services.
6. Real handlers must never use `Uuid::nil()` as the acting user.
7. Private channels must not be visible to unauthorized users.
8. Message read/send/edit/delete must be permission checked.
9. WebSocket events must follow the same permission rules as REST.
10. Do not add LiveKit, mobile, desktop, advanced storage, or UI polish unless the task explicitly asks for it.

---

## Quick Start

### Native workflow, recommended for daily development

```bash
# 1. Start infrastructure only
docker compose -f infra/docker-compose.dev.yml up -d postgres redis

# 2. Run backend
cd apps/server && cargo run

# 3. Run frontend
cd apps/web && npm run dev
```

### Full Docker workflow, integration check

```bash
cp .env.example .env
docker compose -f infra/docker-compose.dev.yml up -d --build
```

---

## Verification Commands

Backend changes:

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Frontend changes:

```bash
cd apps/web
npm install
npm run check
npm run build
```

Infrastructure changes:

```bash
docker compose -f infra/docker-compose.dev.yml config
```

If a command cannot run, report why.

---

## Local Files

`TODO.md` may exist in the local workspace, but it is private and not tracked.

Use `TODO.md` only when the user explicitly says the local agent should use it.

Tracked source of truth is the canonical context listed above.

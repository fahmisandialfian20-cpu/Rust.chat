# AGENTS.md — Context Center

This is the **central instruction hub** for the `context/` folder. All agents working on Rust.chat must read this file first to understand how context is organized, connected, and maintained.

This file is distinct from the workspace-root `AGENTS.md`. The root `AGENTS.md` handles agent workflow and project-wide rules. **This file handles context navigation, standards enforcement, and progress tracking.**

---

## Purpose

The `context/` folder contains the **canonical source of truth** for:
- Project architecture and direction
- Domain models and permission rules
- Code standards and conventions
- Progress tracking for MVP Core Stabilization

**No agent should work without reading the required context files.**

---

## Canonical Context Files

The active context consists of **exactly these files**:

```text
context/
├── AGENTS.md              ← You are here. Read first.
├── 00-project-overview.md ← What Rust.chat is, authority model, core rules
├── 01-product-scope.md    ← MVP scope, features, in/out of scope
├── 02-architecture-stack.md ← Backend/frontend stack, infrastructure
├── 03-domain-permissions.md ← Domain objects, permission model, security rules
├── 04-client-ui-experience.md ← UI direction, empty app behavior, design rules
├── 05-storage-infrastructure.md ← PostgreSQL, Redis, storage, env vars
├── 06-agent-workflow.md   ← How agents should plan and implement
├── code-standards.md      ← Rust + SvelteKit coding standards
└── progress-tracker.md    ← MVP feature status, what's next
```

---

## Mandatory Reading Order

For every task, read in this exact order:

```text
1. AGENTS.md (this file)
2. 00-project-overview.md
3. 01-product-scope.md
4. The ONE task-specific context file (if provided)
5. code-standards.md (before writing code)
6. progress-tracker.md (to check current status)
```

**Do not read the entire `context/` folder.** Read only what the task requires.

**Do not read archived or deprecated files** unless the task explicitly asks for them.

---

## Context File Descriptions

### `00-project-overview.md`
- What Rust.chat is (self-hosted chat, inspired by Discord/Telegram)
- Authority model: Hoster > Admin/Moderator > Member
- Core product rules (backend is source of truth, etc.)
- Current development goal: MVP Core Stabilization

### `01-product-scope.md`
- MVP core scope (auth, spaces, channels, roles, messages, websocket, etc.)
- Out of scope for MVP (voice/video, mobile, themes, notifications, etc.)
- Product quality direction (clean, natural, friendly, not overloaded)

### `02-architecture-stack.md`
- Backend: Rust + Axum + Tokio + SQLx + JWT + WebSocket
- Frontend: SvelteKit + TypeScript + Tailwind CSS
- Database: PostgreSQL (durable), Redis (cache/realtime)
- Infrastructure: Docker Compose for dev
- Architecture rules (backend owns auth, permissions, state)

### `03-domain-permissions.md`
- Domain objects: User, Space, Channel, Role, Message, Invite, etc.
- Permission model: role-based with checklist-style permissions
- Permission keys: ManageInstance, ViewChannel, SendMessages, etc.
- Message permission rules (read/send/edit/delete)
- Non-negotiable security rules
- Invite rules and testing direction

### `04-client-ui-experience.md`
- Web client first, desktop/mobile later
- UI areas: login, lobby, spaces, channels, chat
- Empty app behavior (fresh deploy flow)
- Design rules (avoid long copy, clear actions, honest empty states)

### `05-storage-infrastructure.md`
- PostgreSQL for durable data (users, sessions, spaces, messages, etc.)
- Redis for temporary state (cache, presence, typing, rate limits)
- File storage abstraction (local for dev, pluggable for production)
- Environment variables documentation
- Development security rules

### `06-agent-workflow.md`
- Required reading pattern
- Task context pattern (small scoped tasks)
- Planning before implementation
- Implementation rules (small changes, no rewriting unrelated modules)
- Stop conditions (scope creep, ambiguous security, unclear permissions)
- Good vs bad agent behavior

### `code-standards.md`
- **Read this before writing any code.**
- Rust backend standards (handler→service→repository→domain)
- SvelteKit frontend standards (Svelte 5 runes, TypeScript strict)
- Database schema rules (uuid PK, timestamptz, indexing)
- API & WebSocket conventions
- Security checklist (8 items before merging)
- Verification commands (cargo test, cargo clippy, npm run check)

### `progress-tracker.md`
- **Read this to check current status before starting work.**
- Backend feature completion status (22 features)
- Frontend feature completion status (10 features)
- Infrastructure feature completion status (5 features)
- Testing & quality status
- Priority list for remaining MVP work
- Out of scope tracker

---

## Task-Specific Context

For focused implementation work, create or read a task-specific context file:

```text
context/tasks/auth.md
context/tasks/rbac.md
context/tasks/channel-visibility.md
context/tasks/message-permissions.md
context/tasks/websocket-mvp.md
context/tasks/infrastructure.md
context/tasks/frontend-shell.md
```

A task context must include:
- **Goal** — what to achieve
- **Scope** — what is included
- **Non-goals** — what is explicitly excluded
- **Files to inspect** — existing code to review
- **Files allowed to change** — boundaries
- **Expected behavior** — acceptance criteria
- **Tests** — what must be proven
- **Verification commands** — how to validate
- **Stop conditions** — when to pause for review

**Never implement the whole app at once. Work on one task context at a time.**

---

## Context Maintenance Rules

1. **Keep canonical files up to date.** If you change architecture, permissions, or standards, update the relevant context file.
2. **Keep progress-tracker.md current.** Mark features as 🟢 Done, 🟡 Partial, or 🔴 Not Started after every task.
3. **Do not let context files exceed 250 lines.** If a file grows too large, split it or compact it.
4. **Archive outdated task contexts.** Move completed or obsolete task files to `context/tasks/archive/`.
5. **Add a date to significant updates.** Use "Last updated: YYYY-MM-DD" in `progress-tracker.md`.

---

## Enforcement

Before starting any work, verify:

```text
☐ Read this file (AGENTS.md)
☐ Read 00-project-overview.md
☐ Read 01-product-scope.md
☐ Read code-standards.md (if writing code)
☐ Read progress-tracker.md (to know current status)
☐ Read task-specific context (if one exists)
☐ Checked which files exist in context/ (no stale references)
```

After completing work, verify:

```text
☐ Updated progress-tracker.md if feature status changed
☐ Updated relevant canonical context if behavior/rules changed
☐ Verified code against code-standards.md
☐ Ran verification commands from code-standards.md
☐ No Uuid::nil() used as real acting user
☐ Backend permission checks are in place
☐ WebSocket uses same rules as REST
```

---

## Quick Reference

| Need to know... | Read this file |
|-----------------|----------------|
| What is this project? | `00-project-overview.md` |
| What is in/out of scope? | `01-product-scope.md` |
| What stack is used? | `02-architecture-stack.md` |
| How do permissions work? | `03-domain-permissions.md` |
| What should the UI feel like? | `04-client-ui-experience.md` |
| How is data stored? | `05-storage-infrastructure.md` |
| How should agents work? | `06-agent-workflow.md` |
| How should I write code? | `code-standards.md` |
| What is done and what's next? | `progress-tracker.md` |
| What task am I working on? | `context/tasks/{topic}.md` |

---

## Contact & Ownership

This context folder is the **single source of truth** for Rust.chat project intelligence. If something is unclear, ask before guessing. If you discover outdated information, fix it and note the update date.

**Last updated: 2026-05-05**

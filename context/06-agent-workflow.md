# 06 — Agent Workflow

## Purpose

This file defines how agents should work on Rust.chat.

The goal is to prevent broad, hallucinated, or all-in-one changes.

Agents with small context windows must be able to work safely by reading only the core context and the current task context.

---

## Required Reading Pattern

Every agent must read:

1. `AGENTS.md`
2. `context/00-project-overview.md`
3. `context/01-product-scope.md`
4. One task-specific context file

Do not read the entire context folder.

Do not treat archived/reference files as active instructions.

---

## Task Context Pattern

Every implementation task should have a small task context.

Example:

```text
context/tasks/auth.md
context/tasks/rbac.md
context/tasks/message-permissions.md
context/tasks/websocket-mvp.md
context/tasks/infrastructure.md
```

A task context should include:

```text
Goal
Scope
Non-goals
Files to inspect
Files allowed to change
Expected behavior
Tests
Verification commands
Stop conditions
```

---

## Planning Before Implementation

For medium or risky tasks, the agent must first create a plan/spec.

Flow:

```text
read core context
read task context
inspect relevant files
write plan/spec
stop for review
```

Only after review should implementation start.

---

## Implementation Rules

1. Keep changes small.
2. Do not rewrite unrelated modules.
3. Do not add future features while fixing MVP core.
4. Do not change infrastructure during backend tasks unless required.
5. Do not change frontend during backend tasks unless required.
6. Do not change backend during UI-only tasks unless required.
7. Report every file changed.
8. Report commands run and results.

---

## Stop Conditions

Stop and ask for review if:

- the task scope becomes broad
- old context conflicts with current code
- security behavior is ambiguous
- permissions are unclear
- many unrelated files would need changes
- tests cannot run
- external library behavior is uncertain
- the agent is tempted to implement future features

---

## Good Agent Output

A good planning output says:

```text
I read the required context.
I inspected these files.
The current behavior is this.
The target behavior is this.
The risks are this.
I propose these small implementation steps.
I will stop here for review.
```

A good implementation output says:

```text
Changed files:
- ...

Behavior implemented:
- ...

Tests/commands run:
- command -> result

Known limitations:
- ...
```

---

## Bad Agent Behavior

Do not:

- implement the whole app at once
- jump to LiveKit/mobile/desktop without task request
- silently change project architecture
- rely on frontend permission checks
- bypass backend services
- use placeholder IDs in real handlers
- claim completion without verification
- hide failed commands
- read every context file and confuse future goals with current work

---

## Thinking Agent vs Execution Agent

A thinking/planning agent can create specs and break work into small tasks.

An execution/non-thinking agent should receive one small task context and one reviewed plan.

Execution agents should not be asked to infer the whole project direction.

They should execute a small scoped task with clear acceptance criteria.

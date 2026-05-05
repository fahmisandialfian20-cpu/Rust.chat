# Task Context: Handoff Workflow Stabilization

## Goal

Stabilize the agent handoff workflow so every future agent starts from the latest project progression instead of scanning or rediscovering the repository from scratch.

This is a **documentation/workflow task only**. It does not implement backend, frontend, database, or infrastructure behavior.

The key outcome is that `context/progress-tracker.md` becomes an explicit required reading item for future task execution.

---

## Current Progress Reference

Before doing anything, read these files in this order:

1. `AGENTS.md`
2. `context/00-project-overview.md`
3. `context/01-product-scope.md`
4. `context/progress-tracker.md`
5. `context/06-agent-workflow.md`
6. This task context: `context/tasks/handoff-workflow-stabilization.md`

Current known progression from `context/progress-tracker.md`:

- Current phase: `MVP Core Stabilization`
- Last completed work:
  - Foundation Repair
  - Permission Boundary Tests
  - 3 failing-test bug fixes
- Known verification evidence:
  - `cargo test -- --test-threads=1` passed with 20 tests
  - `cargo clippy -- -D warnings` passed
  - `cargo fmt --check` passed
- Next planned MVP priorities:
  1. WebSocket Permission Tests
  2. Invite Security Tests
  3. End-to-End Message Flow
  4. Frontend Channel Visibility

Do not repeat the completed foundation or permission-boundary audit work unless the current code contradicts the tracker.

---

## Existing Infrastructure

The repository already has these workflow/context files:

```text
AGENTS.md
context/00-project-overview.md
context/01-product-scope.md
context/06-agent-workflow.md
context/progress-tracker.md
context/tasks/foundation-repair.md
context/tasks/foundation-repair-plan.md
context/tasks/permission-boundary-tests.md
context/tasks/permission-boundary-tests-plan.md
context/tasks/bugfix-3-failing-tests.md
```

Important existing behavior:

- `AGENTS.md` routes agents to the core context files and one task-specific context file.
- `context/06-agent-workflow.md` defines safe agent behavior and task context structure.
- `context/progress-tracker.md` records latest progress, verification evidence, completed phases, and next priorities.
- Existing task contexts already use sections such as Goal, Scope, Non-goals, Files to inspect, Files allowed to change, Expected behavior, Tests, Verification commands, and Stop conditions.

Current gap:

- `context/06-agent-workflow.md` does **not yet explicitly require** reading `context/progress-tracker.md` before task execution.
- Existing task context pattern does not explicitly require a `Current Progress Reference` section.
- Existing task context pattern does not explicitly require an `Existing Infrastructure` section.
- Existing task context pattern does not explicitly require a `Progress Tracker Update Requirement` section.

---

## Scope

Update workflow documentation so future agents can work from progression-aware, infrastructure-aware task contexts.

Required updates:

1. Update `context/06-agent-workflow.md` so the required reading pattern includes `context/progress-tracker.md`.
2. Clarify that implementation agents must read the progress tracker before inspecting source files.
3. Update the task context pattern in `context/06-agent-workflow.md` to include these sections:
   - `Current Progress Reference`
   - `Existing Infrastructure`
   - `Progress Tracker Update Requirement`
4. Clarify that a task context should prevent broad rediscovery by listing existing relevant services, handlers, repositories, tests, routes, or docs when known.
5. Clarify that after completing a task, the agent should update `context/progress-tracker.md` with:
   - task completed
   - files changed
   - commands run
   - pass/fail results
   - next recommended task
6. Review `context/progress-tracker.md` only to ensure it remains consistent with the new workflow language.

---

## Non-Goals

Do not:

- Modify backend Rust code.
- Modify frontend Svelte code.
- Modify database migrations.
- Modify Docker or infrastructure files.
- Create multiple new task contexts.
- Start WebSocket permission tests.
- Start invite security tests.
- Start message E2E tests.
- Start frontend channel visibility work.
- Re-audit the whole repository.
- Read the entire `context/` folder.
- Convert `AGENTS.md` into a long project encyclopedia.

---

## Files to Inspect

Only inspect these files unless a contradiction requires stopping for review:

```text
AGENTS.md
context/00-project-overview.md
context/01-product-scope.md
context/progress-tracker.md
context/06-agent-workflow.md
context/tasks/handoff-workflow-stabilization.md
```

---

## Files Allowed to Change

Allowed:

```text
context/06-agent-workflow.md
context/progress-tracker.md
```

Do not change any other file for this task.

If changing another file seems necessary, stop and ask for review.

---

## Expected Behavior

After this task:

1. Future agents know they must read `context/progress-tracker.md` before executing task-specific work.
2. Future task contexts are expected to include progress and infrastructure awareness.
3. Agents should no longer start by broadly scanning the repository when progression already exists.
4. `context/06-agent-workflow.md` remains concise and workflow-focused.
5. `context/progress-tracker.md` records that handoff workflow stabilization was completed.

---

## Tests

No backend or frontend tests are required because this is a documentation-only workflow task.

Do not run expensive backend/frontend verification commands unless source files are accidentally changed.

---

## Verification Commands

Run lightweight verification only:

```text
git diff -- context/06-agent-workflow.md context/progress-tracker.md
```

Optional, if available:

```text
git status --short
```

Expected result:

- Only `context/06-agent-workflow.md` and `context/progress-tracker.md` are modified.
- No backend/frontend files are modified.

---

## Stop Conditions

Stop and ask for review if:

- Updating workflow docs requires changing `AGENTS.md`.
- `context/progress-tracker.md` contradicts the latest git history in a way that cannot be resolved from the existing files.
- You are tempted to inspect broad source directories.
- You are tempted to start implementing WebSocket, invite, message, frontend, or infrastructure work.
- More than the two allowed files need changes.

---

## Progress Tracker Update Requirement

When done, append or update a concise entry in `context/progress-tracker.md` that says:

- Handoff workflow was stabilized.
- `context/progress-tracker.md` is now required reading for future task execution.
- Task contexts should include current progress, existing infrastructure, and progress update requirements.
- Next recommended task remains: `Realtime Infrastructure Map` before WebSocket permission test implementation.

Do not mark WebSocket, invite, message E2E, or frontend channel visibility work as completed.

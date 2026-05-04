# Rust.chat MVP Core Backend Agent Brief

## Purpose

This is the active context for the next local coding agent task.

The previous infrastructure task focused on development infrastructure and Docker Compose cleanup. This new task moves one step closer to the application core, but the agent must still **not implement everything at once**.

The purpose of this brief is to make the agent create a focused plan/spec for MVP core backend stabilization before touching code.

The next agent must not start coding immediately.

The next agent must first create a scoped plan/spec document.

Recommended output:

```text
context-v2/03-mvp-core-backend-plan.md
```

Do not implement backend changes until that plan is reviewed.

---

## Workflow Model

The human user is coordinating with two assistants:

1. **ChatGPT connected to GitHub**
   - Creates and reviews tracked context files.
   - Reviews branches and GitHub changes.
   - Creates issues or review notes when needed.

2. **Local editor agent**
   - Runs inside the local workspace/editor.
   - Reads the tracked context files.
   - Writes plans/specs and later implements narrowly scoped tasks.
   - Must not infer large future work from old broad context.

This file is written for the local editor agent.

---

## Project Summary

Rust.chat is a self-hosted chat platform inspired by Discord and Telegram.

The deployer of the application is the highest authority and is called the **Hoster**.

The backend is the source of truth. All clients must communicate with the backend API and WebSocket gateway. No client may bypass backend permissions.

The system may eventually support:

- Web client
- Desktop client
- Mobile client
- File storage providers
- LiveKit voice/video
- Advanced moderation
- Audit logs
- Production deployment patterns

However, the current task is not to build all of that.

The current task is to stabilize the MVP core backend plan.

---

## Current Stage

Current active stage:

```text
MVP Core Backend Stabilization Planning
```

This means the agent should inspect the current backend and create a plan to stabilize the core backend flows:

1. Authentication
2. Sessions / token validation
3. Hoster bootstrap
4. Spaces
5. Space membership
6. Channels
7. Channel visibility
8. Roles and permissions
9. Invites
10. Messages
11. Basic WebSocket permission behavior
12. Backend tests

The immediate deliverable is a plan/spec, not implementation.

---

## Non-Goals for This Task

Do not work on these in this task:

- LiveKit implementation
- Voice/video behavior
- Mobile app
- Desktop app
- Storage provider expansion
- PikPak integration
- S3/R2/GCS/Azure integration
- UI polish
- Theme system
- Production deployment
- Monitoring stack
- Full audit dashboard
- Notification system
- Reactions
- Threads
- Bot system
- Webhooks
- Large frontend redesign

These may remain future ideas, but they must not enter the current plan except under a short "future work" section.

---

## Why This Task Exists

The project currently has a large amount of context. Earlier agents may jump across too many phases and try to implement advanced features before validating the core backend.

This task exists to force a smaller workflow:

```text
brief -> plan/spec -> review -> narrow implementation issue -> implementation -> review
```

The local agent must not skip the plan/spec step.

---

## Required Agent Behavior

Before writing any code, the local agent must:

1. Read this file.
2. Read `AGENTS.md`.
3. Read `context-v2/01-infrastructure-plan.md` only for infrastructure assumptions.
4. Inspect the current backend files as needed.
5. Use Context7 for library/version validation when relevant.
6. Create `context-v2/03-mvp-core-backend-plan.md`.
7. Stop and ask for review.

The agent must not implement backend changes as part of this first planning step.

The agent must not rewrite the whole backend.

The agent must not create a giant feature batch.

---

## Required Output

Create:

```text
context-v2/03-mvp-core-backend-plan.md
```

The plan must include:

```md
# MVP Core Backend Plan

## 1. Current Understanding
## 2. Files Inspected
## 3. Current Backend State
## 4. MVP Core Boundary
## 5. Non-Goals
## 6. Critical Security Rules
## 7. Auth and Session Plan
## 8. Space and Membership Plan
## 9. Channel Visibility Plan
## 10. RBAC / Permission Plan
## 11. Invite Flow Plan
## 12. Message Permission Plan
## 13. WebSocket MVP Plan
## 14. Required Tests
## 15. Verification Commands
## 16. Known Risks
## 17. Proposed Implementation Issues
## 18. Stop Conditions
```

The plan must be measurable and limited to the MVP core backend.

---

## Backend Source of Truth Rule

The backend must be the only authority for:

- Authentication
- Session validation
- Space membership
- Channel visibility
- Role permissions
- Invite acceptance
- Message read/send/edit/delete permission
- WebSocket event authorization
- File access permission later
- Media token permission later

Frontend checks are allowed only for UI convenience.

Frontend checks are not security.

Every protected backend action must validate permissions server-side.

---

## Critical Security Rules

These rules are non-negotiable:

1. Protected routes must require authenticated user context.
2. Message routes must never use placeholder user IDs.
3. Real handlers must never pass `Uuid::nil()` as the acting user.
4. Private channels must not be visible to unauthorized members.
5. A member must not read messages from a hidden channel.
6. A member must not send messages without `SendMessages` permission.
7. A user must not edit another user's message without `EditAnyMessage`.
8. A user must not delete another user's message without `DeleteAnyMessage`.
9. Hoster bypass must be explicit and tested.
10. WebSocket message sending must use the same permission model as REST.

---

## MVP Permission Concepts

The plan must cover these permission concepts:

```text
ManageInstance
ManageSpaces
ManageMembers
ManageRoles
ManageChannels
ManageInvites
ViewSpace
ViewChannel
ReadMessages
SendMessages
EditOwnMessage
DeleteOwnMessage
EditAnyMessage
DeleteAnyMessage
```

The plan should identify which of these already exist in code and which ones are missing, inconsistent, or unused.

---

## Message Route Requirements

The plan must specifically evaluate and plan fixes for message routes.

Expected message rules:

### List messages

```text
authenticated
AND member of the space
AND can view the channel
AND has ReadMessages permission
```

### Send message

```text
authenticated
AND member of the space
AND can view the channel
AND has SendMessages permission
```

### Edit message

```text
authenticated
AND message belongs to an accessible channel
AND (
  user owns the message AND has EditOwnMessage
  OR user has EditAnyMessage
)
```

### Delete message

```text
authenticated
AND message belongs to an accessible channel
AND (
  user owns the message AND has DeleteOwnMessage
  OR user has DeleteAnyMessage
)
```

The plan must inspect whether the current implementation satisfies these rules.

If not, the plan must propose narrow implementation issues.

---

## Channel Visibility Requirements

The plan must evaluate channel visibility behavior.

Rules:

- Joining the instance does not grant access to all channels.
- Joining a space does not grant access to every private channel.
- Public channels may be visible to space members by default.
- Private channels require explicit permission or role/override access.
- Backend list endpoints must filter unauthorized channels.
- Backend message endpoints must reject unauthorized channel access.

---

## Invite Flow Requirements

The plan must describe the MVP invite behavior.

It should answer:

1. Who can create invites?
2. What does an invite target: instance, space, or channel?
3. What happens when a new user accepts an invite?
4. What happens when an existing user accepts an invite?
5. How expiration and max uses should behave for MVP?
6. Whether invite tokens are stored raw or hashed/HMACed?

Implementation can happen later, but the expected behavior must be clear.

---

## WebSocket MVP Requirements

The plan must keep WebSocket MVP small.

Allowed MVP WebSocket events:

```text
hello
hello.ok
message.send
message.created
message.updated
message.deleted
typing.updated
presence.updated
error
```

Rules:

- WebSocket auth must be validated.
- WebSocket `message.send` must check the same permission as REST message send.
- The server must reject events targeting channels the user cannot access.
- The server must not trust client-supplied role, space, or permission state.

---

## Required Tests in the Plan

The plan must propose tests for:

1. Unauthenticated user cannot access protected backend routes.
2. Member cannot see private channel without permission.
3. Member cannot read messages in hidden/private channel.
4. Member cannot send message without `SendMessages`.
5. Member can send message with `SendMessages`.
6. User cannot edit another user's message without `EditAnyMessage`.
7. User cannot delete another user's message without `DeleteAnyMessage`.
8. Hoster can bypass normal restrictions.
9. Invite accept creates correct membership.
10. WebSocket `message.send` respects `SendMessages`.

---

## Verification Commands

The plan must require these commands before any later implementation is considered complete:

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

If frontend files are touched in a later task:

```bash
cd apps/web
npm install
npm run check
npm run build
```

If Docker/infrastructure files are touched in a later task:

```bash
docker compose -f infra/docker-compose.dev.yml config
```

The local agent must report command results.

If a command cannot run, the local agent must explain why.

---

## Stop Conditions

The local agent must stop and ask for review if:

- The required plan/spec is created.
- A change would require touching many unrelated modules.
- A change would require LiveKit, mobile, desktop, or storage provider work.
- Existing context conflicts with current code.
- The agent finds security-sensitive ambiguity.
- Tests cannot be run.
- The agent is tempted to rewrite the whole backend.

---

## Required Next Step

Using this document, create:

```text
context-v2/03-mvp-core-backend-plan.md
```

Do not implement backend changes yet.

Do not change application code.

Do not create a giant all-in-one implementation.

The next output should be a clear, measurable, staged plan for MVP core backend stabilization.

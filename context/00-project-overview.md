# 00 — Project Overview

## What Rust.chat Is

Rust.chat is a self-hosted chat application inspired by Discord and Telegram.

The person who deploys the application is the **Hoster**.

The Hoster is the highest authority in the instance.

The application starts empty. The Hoster creates spaces, channels, roles, permissions, and invite flows.

---

## Product Goal

Build a private/self-hosted communication platform where a small community, team, or group can run its own chat server.

The product must support:

- login and registration
- invite-based joining
- lobby/entry area
- many spaces
- custom channels
- public channels
- private channels
- roles and permission checklists
- admins and moderators
- real-time text chat
- file sharing later
- voice/video later
- web client first
- native desktop and mobile later

---

## Authority Model

```text
Hoster > Admin / Moderator > Member
```

### Hoster

The Hoster is the instance owner.

The Hoster can:

- bootstrap the first account
- create spaces
- create public and private channels
- create roles
- grant permissions through checklist-style role settings
- promote members to admin or moderator roles
- create invite links
- decide what each channel allows
- disable channel features such as voice, video, or file sending

### Admin / Moderator

Admins and moderators are normal members with extra permissions.

They only have the permissions granted by the Hoster or another authorized role.

### Member

Members can register or join through invite links.

Members can enter the lobby/application, but that does **not** mean they can access every space or every channel.

---

## Core Product Rules

1. The backend is the source of truth.
2. Frontend checks are only for user experience.
3. All permissions must be verified by the backend.
4. Entering the lobby does not grant access to every channel.
5. Public channels are visible to allowed space members.
6. Private channels require explicit permission or invite/access rule.
7. Channel features can be customized per channel.
8. The app must not assume a default public server exists.
9. The app must work when freshly deployed with no spaces or channels.

---

## Current Development Goal

Current goal:

```text
MVP Core Stabilization
```

This means the project should focus on the working foundation first:

- auth
- sessions / tokens
- Hoster bootstrap
- spaces
- members
- channels
- RBAC / permissions
- invites
- messages
- basic WebSocket realtime
- local dev infrastructure
- tests

---

## Current Non-Goals

Do not work on these until a task explicitly asks for them:

- LiveKit voice/video implementation
- mobile app
- desktop app
- PikPak integration
- advanced storage provider integration
- themes/skins beyond planning
- notification system
- reactions
- threads
- bots
- webhooks
- production cloud automation
- monitoring dashboards

---

## Success Definition for MVP Core

The MVP core is successful when:

1. Hoster can bootstrap the first account.
2. Hoster can create a space.
3. Hoster can create public and private channels.
4. Hoster can create roles with permission checklists.
5. Members can join through invite links.
6. Members only see channels they are allowed to see.
7. Members can read/send/edit/delete messages only when permitted.
8. Backend rejects unauthorized REST and WebSocket actions.
9. Local development can run with PostgreSQL and Redis.
10. Backend tests prove the permission boundaries.

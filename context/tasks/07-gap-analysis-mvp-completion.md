# Gap Analysis: MVP Core Stabilization

**Date:** 2026-05-05
**Status:** Phase 3 Backend Complete ✅ | Frontend Channel Visibility 🟡 In Progress
**Purpose:** Identify all remaining work to declare MVP Core complete

---

## MVP Success Definition vs Actual Status

From `context/00-project-overview.md`:

| # | Success Criteria | Status | Gap |
|---|-----------------|--------|-----|
| 1 | Hoster can bootstrap first account | ✅ | None |
| 2 | Hoster can create a space | ✅ | None |
| 3 | Hoster can create public/private channels | ✅ | None |
| 4 | Hoster can create roles with permission checklists | ✅ | None |
| 5 | Members can join through invite links | ✅ | None |
| 6 | **Members only see channels they are allowed to see** | 🟡 | Frontend WS sync partial; needs completion |
| 7 | **Members R/W/E/D messages only when permitted** | ✅ Backend / 🟡 Frontend | Frontend UI doesn't disable actions based on permissions |
| 8 | Backend rejects unauthorized REST and WS actions | ✅ | None |
| 9 | Local development runs with PostgreSQL + Redis | ✅ | None |
| 10 | Backend tests prove permission boundaries | ✅ | 40/40 tests pass |

---

## Remaining Work to Complete MVP Core

### 🔴 Critical Path (Must Complete)

#### 1. Frontend Channel Visibility Completion
**Status:** 🟡 Started (agent coder implemented base)
**Missing:**
- Handle `channel.updated` events (rename, position change)
- Handle `channel.deleted` events (remove from list)
- Handle `channel.visibility_changed` (re-fetch visible channels)
- Permission-based action buttons in `ChannelList.svelte` (+ button, context menu)
- "Create first channel" CTA when empty + has `ManageChannels`

**Files:** `context/tasks/frontend-channel-visibility.md` ✅

#### 2. Frontend Permission Integration
**Status:** 🔴 Not started
**Needed:**
- Disable `MessageComposer.svelte` when no `SendMessages`
- Show "Read-only" badge in `MessageList.svelte`
- Hide file upload button when no `SendFiles`
- Show "No permission" tooltip on disabled controls

**Effort:** Small (1 session)

#### 3. E2E User Journey Test
**Status:** 🔴 Not started
**Needed:**
- Full flow: Bootstrap → Create space → Create channel → Invite user → Join → Send message
- Can be manual testing script or automated Playwright test
- Proves the entire MVP works end-to-end

**Effort:** Small-Medium (1-2 sessions)

---

### 🟡 Important (Should Complete)

#### 4. WebSocket Event Broadcasting (Backend)
**Status:** 🟢 Partial
**Current:** Frontend can receive `channel.created`
**Missing:**
- Backend doesn't broadcast `channel.updated` / `channel.deleted` / `channel.visibility_changed` events yet
- These events need to be emitted from channel service/handlers after mutation

**Effort:** Small (add event publishing in 3 service methods)

#### 5. Frontend Error Handling Polish
**Status:** 🟡 Partial
**Missing:**
- Exponential backoff retry already in layout, but no toast notifications
- Network failure UI is just loading spinner forever
- Should show "Connection lost, retrying..." with cancel option

**Effort:** Small

---

### 🟢 Nice to Have (Can Defer)

#### 6. `get_invite` / `list_invites` Authorization
**Status:** 🔴 Not started (from code review)
**Note:** Invite codes are hidden via `InviteResponse`, but metadata is still readable by any authenticated user
**Risk:** Low — no sensitive data leaked (just space_id, created_by, counts)

#### 7. WebSocket Rate Limiting
**Status:** 🔴 Not started
**Note:** No rate limit on WS commands; spam possible
**Risk:** Low-Medium — requires malicious client

#### 8. Invite Rate Limit Config
**Status:** 🟡 Partial
**Current:** `accept_invite` uses login rate limit config
**Fix:** Add `rate_limit.invite_accept` field to config
**Risk:** Very low

---

## Recommended Completion Order

```
1. Frontend Channel Visibility (complete WS events handling)     ← 1 session
2. Frontend Permission Integration (disable actions)               ← 1 session  
3. Backend: Broadcast channel.updated/deleted events               ← 0.5 session
4. E2E Journey Test (manual or Playwright)                         ← 1 session
5. Polish: Error states, toasts, loading UX                        ← 0.5 session
```

**Total: ~4 sessions to MVP Core Complete**

---

## What "MVP Core Complete" Looks Like

Backend:
- ✅ All handlers authenticated and permission-checked
- ✅ All 40 tests pass
- ✅ WebSocket commands validated and channel-scoped
- ✅ Invite system secure (hashed, atomic, rate-limited)
- 🟡 Channel event broadcasting (needs update/delete events)

Frontend:
- ✅ Pages exist (login, lobby, spaces, channels, admin)
- ✅ Svelte-check clean, build passes
- 🟡 Channel list syncs in real-time
- 🟡 UI actions respect permissions
- 🟡 Error handling graceful

Infrastructure:
- ✅ Docker dev environment works
- ✅ PostgreSQL + Redis running
- ✅ `.env` documented

---

## Post-MVP (Don't Touch Yet)

- Voice/video (LiveKit)
- Mobile/desktop apps
- File storage (S3/R2/PikPak)
- Notifications
- Reactions/threads
- Production deployment

---

**Created:** 2026-05-05
**Next Decision:** Prioritize remaining 5 items for completion

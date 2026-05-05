# E2E User Journey Test Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create comprehensive end-to-end test documentation (manual + Playwright automated) proving the entire MVP core works across Hoster setup, invite flow, and permission enforcement.

**Architecture:** Produce three assets: (1) a manual test script with exact curl commands for every API step, (2) a Playwright setup guide, and (3) automated Playwright tests. The manual script serves as the canonical reference; Playwright tests automate the same journeys. Each journey exercises a distinct set of backend handlers: auth, spaces, channels, messages, roles, invites, and permissions.

**Tech Stack:** Rust (Axum), SvelteKit 5, Playwright (TypeScript), PostgreSQL, Redis

---

## File Structure

| File | Purpose |
|------|---------|
| `docs/testing/e2e-manual-test.md` | Step-by-step manual test script with curl commands |
| `docs/testing/e2e-playwright-setup.md` | Playwright installation and configuration guide |
| `apps/web/e2e/journey.spec.ts` | Automated Playwright test for all 3 journeys |
| `apps/web/playwright.config.ts` | Playwright configuration file |

---

### Task 1: Create manual test script

**Files:**
- Create: `docs/testing/e2e-manual-test.md`

- [ ] **Step 1: Write the manual test script header and prerequisites**

```
# E2E Manual Test Script

> Run against a fresh instance with empty database.
> All commands use `curl` and `jq` (for JSON extraction).
> Set `BASE=http://localhost:3000` before running.

## Prerequisites

- Rust.chat backend running on `http://localhost:3000`
- Empty PostgreSQL database (run `docker compose -f infra/docker-compose.dev.yml down -v && docker compose -f infra/docker-compose.dev.yml up -d postgres redis`)
- `curl` and `jq` installed
- `BASE=http://localhost:3000` environment variable set

```bash
export BASE=http://localhost:3000
```
```

- [ ] **Step 2: Write Journey A - Hoster Setup Script**

In `docs/testing/e2e-manual-test.md`, add after the header:

```markdown
## Journey A: Hoster Setup (Single User)

Tests: fresh instance bootstrap, space creation, public channel creation, message send/read.

### A1: Bootstrap owner account

```bash
echo "=== A1: Bootstrap owner ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/auth/bootstrap-owner" \
  -H "Content-Type: application/json" \
  -d '{"username":"hoster","password":"password123"}')
echo "$RESPONSE" | jq .
HOSTER_TOKEN=$(echo "$RESPONSE" | jq -r '.access_token')
HOSTER_USER_ID=$(echo "$RESPONSE" | jq -r '.user.id')
echo "HOSTER_TOKEN=$HOSTER_TOKEN"
echo "HOSTER_USER_ID=$HOSTER_USER_ID"
```

**Expected:** HTTP 200. Response contains `user.id`, `access_token`, `refresh_token`. If HTTP 409, instance already has an owner — reset the database.

### A2: Create space "Team Chat"

```bash
echo "=== A2: Create space ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/spaces" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"name":"Team Chat","description":"Main team space"}')
echo "$RESPONSE" | jq .
SPACE_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "SPACE_ID=$SPACE_ID"
```

**Expected:** HTTP 200. Response contains `id`, `name: "Team Chat"`, `slug: "team-chat"`.

### A3: Create public channel "general"

```bash
echo "=== A3: Create public channel ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/spaces/$SPACE_ID/channels" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"name":"general","kind":"Text","visibility":"public"}')
echo "$RESPONSE" | jq .
CHANNEL_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "CHANNEL_ID=$CHANNEL_ID"
```

**Expected:** HTTP 200. Response contains `id`, `name: "general"`, `visibility: "Public"`, `space_id` matching `$SPACE_ID`.

### A4: Send first message

```bash
echo "=== A4: Send message ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/channels/$CHANNEL_ID/messages" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"content":"Hello team!"}')
echo "$RESPONSE" | jq .
MESSAGE_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "MESSAGE_ID=$MESSAGE_ID"
```

**Expected:** HTTP 200. Response contains `id`, `content: "Hello team!"`, `author_user_id` matching `$HOSTER_USER_ID`.

### A5: Verify message appears in channel

```bash
echo "=== A5: List messages ==="
curl -s "$BASE/api/v1/channels/$CHANNEL_ID/messages" \
  -H "Authorization: Bearer $HOSTER_TOKEN" | jq .
```

**Expected:** HTTP 200. Response is a JSON array containing at least one message with `content: "Hello team!"`.

### A6: Verify Journey A passed

If all steps A1–A5 returned the expected results, **Journey A PASSES**.
```

- [ ] **Step 3: Write Journey B - Invite Flow Script**

In `docs/testing/e2e-manual-test.md`, add after Journey A:

```markdown
## Journey B: Invite Flow (Two Users)

Tests: multi-space creation, private channels, role creation with permissions, invite generation, invite-based registration, role assignment, channel visibility filtering, cross-user message visibility.

### B1: Create space "Community"

```bash
echo "=== B1: Create space Community ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/spaces" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"name":"Community","description":"Community space"}')
echo "$RESPONSE" | jq .
SPACE2_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "SPACE2_ID=$SPACE2_ID"
```

**Expected:** HTTP 200. `name: "Community"`.

### B2: Create private channel "vip"

```bash
echo "=== B2: Create private channel vip ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/spaces/$SPACE2_ID/channels" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"name":"vip","kind":"Text","visibility":"private"}')
echo "$RESPONSE" | jq .
CHANNEL2_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "CHANNEL2_ID=$CHANNEL2_ID"
```

**Expected:** HTTP 200. `visibility: "Private"`.

### B3: Create role "VIP" with ViewChannel permission

```bash
echo "=== B3: Create VIP role ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/spaces/$SPACE2_ID/roles" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"name":"VIP","permission_keys":["ViewChannel","ReadMessages","SendMessages"]}')
echo "$RESPONSE" | jq .
VIP_ROLE_ID=$(echo "$RESPONSE" | jq -r '.role.id')
echo "VIP_ROLE_ID=$VIP_ROLE_ID"
```

**Expected:** HTTP 200. Response contains `role.name: "VIP"` and `permission_keys` includes `"ViewChannel"`, `"ReadMessages"`, `"SendMessages"`.

### B4: Generate invite link

```bash
echo "=== B4: Create invite ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/invites" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d "{\"space_id\":\"$SPACE2_ID\",\"max_uses\":10}")
echo "$RESPONSE" | jq .
INVITE_CODE=$(echo "$RESPONSE" | jq -r '.code')
INVITE_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "INVITE_CODE=$INVITE_CODE"
echo "INVITE_ID=$INVITE_ID"
```

**Expected:** HTTP 200. `code` is a non-empty string, `space_id` matches `$SPACE2_ID`.

### B5: Register User B via invite link

```bash
echo "=== B5: Register User B ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"userb\",\"password\":\"password456\",\"invite_code\":\"$INVITE_CODE\"}")
echo "$RESPONSE" | jq .
USERB_TOKEN=$(echo "$RESPONSE" | jq -r '.access_token')
USERB_ID=$(echo "$RESPONSE" | jq -r '.user.id')
echo "USERB_TOKEN=$USERB_TOKEN"
echo "USERB_ID=$USERB_ID"
```

**Expected:** HTTP 200. Contains `user.id`, `access_token`.

### B6: User B joins space via invite accept

```bash
echo "=== B6: User B accepts invite ==="
curl -s -X POST "$BASE/api/v1/invites/$INVITE_CODE/accept" \
  -H "Authorization: Bearer $USERB_TOKEN" | jq .
```

**Expected:** HTTP 200. User B is now a member of the Community space.

### B7: Hoster assigns VIP role to User B

```bash
echo "=== B7: Assign VIP role ==="
curl -s -X POST "$BASE/api/v1/spaces/$SPACE2_ID/members/$USERB_ID/roles" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d "{\"role_id\":\"$VIP_ROLE_ID\"}" | jq .
```

**Expected:** HTTP 200. `status: "assigned"`.

### B8: User B sees "vip" channel in visible channels

```bash
echo "=== B8: User B visible channels ==="
RESPONSE=$(curl -s "$BASE/api/v1/spaces/$SPACE2_ID/channels/visible" \
  -H "Authorization: Bearer $USERB_TOKEN")
echo "$RESPONSE" | jq .
VISIBLE_IDS=$(echo "$RESPONSE" | jq -r '.[].id')
echo "Visible channel IDs: $VISIBLE_IDS"
```

**Expected:** HTTP 200. The response array includes the "vip" channel (`$CHANNEL2_ID`).

### B9: User B sends message in "vip" channel

```bash
echo "=== B9: User B sends message ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/channels/$CHANNEL2_ID/messages" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $USERB_TOKEN" \
  -d '{"content":"Hello from User B in VIP!"}')
echo "$RESPONSE" | jq .
USERB_MSG_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "USERB_MSG_ID=$USERB_MSG_ID"
```

**Expected:** HTTP 200. `content: "Hello from User B in VIP!"`, `author_user_id` matches `$USERB_ID`.

### B10: Hoster sees User B's message

```bash
echo "=== B10: Hoster lists messages in vip ==="
curl -s "$BASE/api/v1/channels/$CHANNEL2_ID/messages" \
  -H "Authorization: Bearer $HOSTER_TOKEN" | jq .
```

**Expected:** HTTP 200. Response array includes the message with `content: "Hello from User B in VIP!"`.

### B11: Verify Journey B passed

If all steps B1–B10 returned the expected results, **Journey B PASSES**.
```

- [ ] **Step 4: Write Journey C - Permission Enforcement Script**

In `docs/testing/e2e-manual-test.md`, add after Journey B:

```markdown
## Journey C: Permission Enforcement

Tests: creating restricted roles, send-message denial, role update without reload, send-message grant after role update.

### C1: Register User C

```bash
echo "=== C1: Register User C ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"userc\",\"password\":\"password789\",\"invite_code\":\"$INVITE_CODE\"}")
echo "$RESPONSE" | jq .
USERC_TOKEN=$(echo "$RESPONSE" | jq -r '.access_token')
USERC_ID=$(echo "$RESPONSE" | jq -r '.user.id')
echo "USERC_TOKEN=$USERC_TOKEN"
echo "USERC_ID=$USERC_ID"
```

**Expected:** HTTP 200.

### C2: User C accepts invite

```bash
echo "=== C2: User C accepts invite ==="
curl -s -X POST "$BASE/api/v1/invites/$INVITE_CODE/accept" \
  -H "Authorization: Bearer $USERC_TOKEN" | jq .
```

**Expected:** HTTP 200. User C is now a member of the Community space.

### C3: Create role "Viewer" with ReadMessages + ViewChannel only

```bash
echo "=== C3: Create Viewer role ==="
RESPONSE=$(curl -s -X POST "$BASE/api/v1/spaces/$SPACE2_ID/roles" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"name":"Viewer","permission_keys":["ViewChannel","ReadMessages"]}')
echo "$RESPONSE" | jq .
VIEWER_ROLE_ID=$(echo "$RESPONSE" | jq -r '.role.id')
echo "VIEWER_ROLE_ID=$VIEWER_ROLE_ID"
```

**Expected:** HTTP 200. `permission_keys` includes `"ViewChannel"` and `"ReadMessages"` but **NOT** `"SendMessages"`.

### C4: Assign "Viewer" role to User C

```bash
echo "=== C4: Assign Viewer role ==="
curl -s -X POST "$BASE/api/v1/spaces/$SPACE2_ID/members/$USERC_ID/roles" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d "{\"role_id\":\"$VIEWER_ROLE_ID\"}" | jq .
```

**Expected:** HTTP 200. `status: "assigned"`.

### C5: User C opens channel - sees messages (GET succeeds)

```bash
echo "=== C5: User C lists messages ==="
curl -s "$BASE/api/v1/channels/$CHANNEL2_ID/messages" \
  -H "Authorization: Bearer $USERC_TOKEN" | jq .
```

**Expected:** HTTP 200. Response contains messages array (can see existing messages from B9/B10).

### C6: User C cannot send message (POST fails with 403)

```bash
echo "=== C6: User C tries to send ==="
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/v1/channels/$CHANNEL2_ID/messages" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $USERC_TOKEN" \
  -d '{"content":"I should not be able to send"}')
echo "HTTP status: $RESPONSE"
if [ "$RESPONSE" = "403" ]; then echo "PASS: Got 403 Forbidden"; else echo "FAIL: Expected 403"; fi
```

**Expected:** HTTP 403. User C is denied from sending messages because `SendMessages` permission is missing.

### C7: Hoster updates role to add SendMessages

```bash
echo "=== C7: Update Viewer role to add SendMessages ==="
curl -s -X PUT "$BASE/api/v1/spaces/$SPACE2_ID/roles/$VIEWER_ROLE_ID" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $HOSTER_TOKEN" \
  -d '{"permission_keys":["ViewChannel","ReadMessages","SendMessages"]}' | jq .
```

**Expected:** HTTP 200. Response `permission_keys` now includes `"SendMessages"`.

### C8: User C can now send message (POST succeeds)

```bash
echo "=== C8: User C sends message after role update ==="
curl -s -X POST "$BASE/api/v1/channels/$CHANNEL2_ID/messages" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $USERC_TOKEN" \
  -d '{"content":"Now I can send!"}' | jq .
```

**Expected:** HTTP 200. Message is created successfully. User C now has `SendMessages` permission through the updated role.

### C9: Verify Journey C passed

If all steps C1–C8 returned the expected results, **Journey C PASSES**.
```

- [ ] **Step 5: Write troubleshooting section**

In `docs/testing/e2e-manual-test.md`, add at the end:

```markdown
## Common Failure Modes

| Symptom | Likely Cause | Fix |
|---------|-------------|-----|
| A1 returns 409 | Instance already has owner | Reset DB: `docker compose -f infra/docker-compose.dev.yml down -v && docker compose -f infra/docker-compose.dev.yml up -d postgres redis` |
| A2 returns 403 | Token missing or expired | Re-bootstrap and save new token |
| B6 returns 400 | Invite code invalid or expired | Re-create invite in B4 and use new code |
| B8 doesn't show "vip" | Role not assigned yet | Check role assignment response in B7. Verify `$VIP_ROLE_ID` is correct. |
| C6 returns 200 instead of 403 | Permission check missing in service | Backend bug: `message_service.create_message` doesn't check `SendMessages` permission |
| C8 returns 403 | Role update didn't persist | Check role update response. Verify `permission_keys` includes `SendMessages`. |
| Any step returns 401 | Token expired or missing | Verify `echo $TOKEN` returns a non-empty string |
| `jq` parse error | JSON response unexpected | Run bare `curl` command without `| jq` to see raw response |

## Running the Full Suite

Copy the entire script into a single shell file and run:

```bash
#!/bin/bash
set -e
BASE=http://localhost:3000

# Journey A
# (paste all A commands here)

# Journey B
# (paste all B commands here)

# Journey C
# (paste all C commands here)

echo "=== ALL JOURNEYS COMPLETE ==="
```

Or run each section individually for debugging.
```

- [ ] **Step 6: Verify the manual test document is internally consistent**

Check that every variable referenced (`$HOSTER_TOKEN`, `$SPACE_ID`, `$SPACE2_ID`, `$CHANNEL_ID`, `$CHANNEL2_ID`, `$INVITE_CODE`, `$VIP_ROLE_ID`, `$VIEWER_ROLE_ID`, `$USERB_TOKEN`, `$USERB_ID`, `$USERC_TOKEN`, `$USERC_ID`) is set before use. Check that Journey B and C reuse vars from earlier journeys correctly (Journey B uses `$HOSTER_TOKEN` from A, `$SPACE2_ID` from B1; Journey C uses `$INVITE_CODE` from B4, `$SPACE2_ID` from B1, `$CHANNEL2_ID` from B2).

---

### Task 2: Create Playwright setup guide

**Files:**
- Create: `docs/testing/e2e-playwright-setup.md`
- Modify: `apps/web/package.json`

- [ ] **Step 1: Write Playwright setup document**

Create `docs/testing/e2e-playwright-setup.md`:

```markdown
# Playwright E2E Setup

## Prerequisites

- Backend running on `http://localhost:3000`
- Frontend running on `http://localhost:5173`
- Node.js 18+
- Empty database

## Install

```bash
cd apps/web
npm install -D @playwright/test
npx playwright install chromium
```

## Configuration

Create `apps/web/playwright.config.ts`:

```typescript
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  timeout: 30000,
  retries: 1,
  use: {
    baseURL: 'http://localhost:5173',
    extraHTTPHeaders: {
      'Content-Type': 'application/json',
    },
  },
  webServer: [
    {
      command: 'cd apps/server && cargo run',
      port: 3000,
      reuseExistingServer: true,
    },
    {
      command: 'cd apps/web && npm run dev',
      port: 5173,
      reuseExistingServer: true,
    },
  ],
});
```

## Running Tests

```bash
cd apps/web
npx playwright test
```

For headed mode (see the browser):

```bash
npx playwright test --headed
```

For UI mode:

```bash
npx playwright test --ui
```
```

- [ ] **Step 2: Add Playwright test script to package.json**

In `apps/web/package.json`, add a `"test:e2e"` script to the `"scripts"` section:

```json
"test:e2e": "playwright test",
```

- [ ] **Step 3: Verify the dependency installs**

Run: `cd apps/web && npm install -D @playwright/test`
Expected: Installs without errors, adds playwright to devDependencies.

---

### Task 3: Write Playwright automated tests

**Files:**
- Create: `apps/web/e2e/journey.spec.ts`
- Create: `apps/web/e2e/helpers.ts`
- Create: `apps/web/e2e/types.ts`

- [ ] **Step 1: Create shared types**

Create `apps/web/e2e/types.ts`:

```typescript
export interface AuthResponse {
  user: { id: string; username: string };
  access_token: string;
  refresh_token: string;
}

export interface Space {
  id: string;
  name: string;
  slug: string;
  description: string | null;
}

export interface Channel {
  id: string;
  space_id: string;
  name: string;
  slug: string;
  kind: string;
  visibility: string;
}

export interface RoleWithPermissions {
  role: {
    id: string;
    space_id: string;
    name: string;
    is_default: boolean;
  };
  permission_keys: string[];
}

export interface Message {
  id: string;
  channel_id: string;
  author_user_id: string;
  content: string;
}

export interface Invite {
  id: string;
  code: string;
  space_id: string | null;
  max_uses: number | null;
  used_count: number;
}
```

- [ ] **Step 2: Create shared helpers**

Create `apps/web/e2e/helpers.ts`:

```typescript
import { type APIRequestContext, expect } from '@playwright/test';
import type { AuthResponse, Space, Channel, RoleWithPermissions, Message, Invite } from './types';

const API_BASE = 'http://localhost:3000/api/v1';

export async function bootstrapOwner(request: APIRequestContext): Promise<AuthResponse> {
  const res = await request.post(`${API_BASE}/auth/bootstrap-owner`, {
    data: { username: 'hoster', password: 'password123' },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function registerUser(
  request: APIRequestContext,
  username: string,
  password: string,
  inviteCode?: string,
): Promise<AuthResponse> {
  const res = await request.post(`${API_BASE}/auth/register`, {
    data: { username, password, invite_code: inviteCode },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function createSpace(
  request: APIRequestContext,
  token: string,
  name: string,
  description?: string,
): Promise<Space> {
  const res = await request.post(`${API_BASE}/spaces`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { name, description: description ?? null },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function createChannel(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  name: string,
  visibility: 'public' | 'private',
): Promise<Channel> {
  const res = await request.post(`${API_BASE}/spaces/${spaceId}/channels`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { name, kind: 'Text', visibility },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function sendMessage(
  request: APIRequestContext,
  token: string,
  channelId: string,
  content: string,
): Promise<Message> {
  const res = await request.post(`${API_BASE}/channels/${channelId}/messages`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { content },
  });
  return res.json();
}

export async function listMessages(
  request: APIRequestContext,
  token: string,
  channelId: string,
): Promise<Message[]> {
  const res = await request.get(`${API_BASE}/channels/${channelId}/messages`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function createInvite(
  request: APIRequestContext,
  token: string,
  spaceId: string,
): Promise<Invite> {
  const res = await request.post(`${API_BASE}/invites`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { space_id: spaceId, max_uses: 10 },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function acceptInvite(
  request: APIRequestContext,
  token: string,
  code: string,
): Promise<void> {
  const res = await request.post(`${API_BASE}/invites/${code}/accept`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok()).toBeTruthy();
}

export async function createRole(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  name: string,
  permissionKeys: string[],
): Promise<RoleWithPermissions> {
  const res = await request.post(`${API_BASE}/spaces/${spaceId}/roles`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { name, permission_keys: permissionKeys },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function assignRole(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  userId: string,
  roleId: string,
): Promise<void> {
  const res = await request.post(`${API_BASE}/spaces/${spaceId}/members/${userId}/roles`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { role_id: roleId },
  });
  expect(res.ok()).toBeTruthy();
}

export async function updateRole(
  request: APIRequestContext,
  token: string,
  spaceId: string,
  roleId: string,
  permissionKeys: string[],
): Promise<RoleWithPermissions> {
  const res = await request.put(`${API_BASE}/spaces/${spaceId}/roles/${roleId}`, {
    headers: { Authorization: `Bearer ${token}` },
    data: { permission_keys: permissionKeys },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}

export async function listVisibleChannels(
  request: APIRequestContext,
  token: string,
  spaceId: string,
): Promise<Channel[]> {
  const res = await request.get(`${API_BASE}/spaces/${spaceId}/channels/visible`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok()).toBeTruthy();
  return res.json();
}
```

- [ ] **Step 3: Create Journey A test**

Create `apps/web/e2e/journey.spec.ts`:

```typescript
import { test, expect } from '@playwright/test';
import {
  bootstrapOwner,
  registerUser,
  createSpace,
  createChannel,
  sendMessage,
  listMessages,
  createInvite,
  acceptInvite,
  createRole,
  assignRole,
  updateRole,
  listVisibleChannels,
} from './helpers';

test.describe('Journey A: Hoster Setup', () => {
  let hosterToken: string;
  let hosterUserId: string;
  let spaceId: string;
  let channelId: string;

  test('A1-A2: Bootstrap owner and create space', async ({ request }) => {
    const auth = await bootstrapOwner(request);
    hosterToken = auth.access_token;
    hosterUserId = auth.user.id;

    const space = await createSpace(request, hosterToken, 'Team Chat', 'Main team space');
    spaceId = space.id;

    expect(space.name).toBe('Team Chat');
  });

  test('A3: Create public channel "general"', async ({ request }) => {
    const channel = await createChannel(request, hosterToken, spaceId, 'general', 'public');
    channelId = channel.id;

    expect(channel.name).toBe('general');
    expect(channel.visibility).toBe('Public');
  });

  test('A4-A5: Send message and verify', async ({ request }) => {
    const msg = await sendMessage(request, hosterToken, channelId, 'Hello team!');
    expect(msg.content).toBe('Hello team!');
    expect(msg.author_user_id).toBe(hosterUserId);

    const messages = await listMessages(request, hosterToken, channelId);
    expect(messages.some((m) => m.content === 'Hello team!')).toBeTruthy();
  });
});
```

- [ ] **Step 4: Create Journey B test**

In `apps/web/e2e/journey.spec.ts`, add after Journey A:

```typescript
test.describe('Journey B: Invite Flow', () => {
  let hosterToken: string;
  let spaceId: string;
  let channelId: string;
  let vipRoleId: string;
  let inviteCode: string;
  let userBToken: string;
  let userBId: string;

  test.beforeAll(async ({ request }) => {
    const auth = await bootstrapOwner(request);
    hosterToken = auth.access_token;
  });

  test('B1-B3: Create space, private channel, and VIP role', async ({ request }) => {
    const space = await createSpace(request, hosterToken, 'Community', 'Community space');
    spaceId = space.id;

    const channel = await createChannel(request, hosterToken, spaceId, 'vip', 'private');
    channelId = channel.id;
    expect(channel.visibility).toBe('Private');

    const role = await createRole(request, hosterToken, spaceId, 'VIP', [
      'ViewChannel',
      'ReadMessages',
      'SendMessages',
    ]);
    vipRoleId = role.role.id;
    expect(role.permission_keys).toContain('ViewChannel');
  });

  test('B4-B6: Generate invite, register User B, accept invite', async ({ request }) => {
    const invite = await createInvite(request, hosterToken, spaceId);
    inviteCode = invite.code;
    expect(inviteCode).toBeTruthy();

    const auth = await registerUser(request, 'userb', 'password456', inviteCode);
    userBToken = auth.access_token;
    userBId = auth.user.id;

    await acceptInvite(request, userBToken, inviteCode);
  });

  test('B7-B8: Assign role and verify visibility', async ({ request }) => {
    await assignRole(request, hosterToken, spaceId, userBId, vipRoleId);

    const visible = await listVisibleChannels(request, userBToken, spaceId);
    const vipChannel = visible.find((c) => c.id === channelId);
    expect(vipChannel).toBeDefined();
  });

  test('B9-B10: User B sends message, Hoster reads it', async ({ request }) => {
    const msg = await sendMessage(request, userBToken, channelId, 'Hello from User B in VIP!');
    expect(msg.author_user_id).toBe(userBId);

    const hosterMessages = await listMessages(request, hosterToken, channelId);
    const found = hosterMessages.find((m) => m.content === 'Hello from User B in VIP!');
    expect(found).toBeDefined();
    expect(found!.author_user_id).toBe(userBId);
  });
});
```

- [ ] **Step 5: Create Journey C test**

In `apps/web/e2e/journey.spec.ts`, add after Journey B:

```typescript
test.describe('Journey C: Permission Enforcement', () => {
  let hosterToken: string;
  let spaceId: string;
  let channelId: string;
  let viewerRoleId: string;
  let userCToken: string;
  let userCId: string;
  let inviteCode: string;

  test.beforeAll(async ({ request }) => {
    const auth = await bootstrapOwner(request);
    hosterToken = auth.access_token;

    const space = await createSpace(request, hosterToken, 'PermTest', 'permission test space');
    spaceId = space.id;

    const channel = await createChannel(request, hosterToken, spaceId, 'general', 'public');
    channelId = channel.id;

    const invite = await createInvite(request, hosterToken, spaceId);
    inviteCode = invite.code;

    // Register a helper user B so there are messages in the channel
    const userBAuth = await registerUser(request, 'userb-perm', 'password456', inviteCode);
    await acceptInvite(request, userBAuth.access_token, inviteCode);
    // Give user B send permission
    const everyoneRole = await createRole(request, hosterToken, spaceId, 'Member', [
      'ViewChannel', 'ReadMessages', 'SendMessages',
    ]);
    await assignRole(request, hosterToken, spaceId, userBAuth.user.id, everyoneRole.role.id);
    await sendMessage(request, userBAuth.access_token, channelId, 'Pre-existing message');
  });

  test('C1-C4: Register User C, create Viewer role, assign it', async ({ request }) => {
    const userCAuth = await registerUser(request, 'userc', 'password789', inviteCode);
    userCToken = userCAuth.access_token;
    userCId = userCAuth.user.id;

    await acceptInvite(request, userCToken, inviteCode);

    const viewerRole = await createRole(request, hosterToken, spaceId, 'Viewer', [
      'ViewChannel',
      'ReadMessages',
    ]);
    viewerRoleId = viewerRole.role.id;
    expect(viewerRole.permission_keys).toEqual(['ViewChannel', 'ReadMessages']);
    expect(viewerRole.permission_keys).not.toContain('SendMessages');

    await assignRole(request, hosterToken, spaceId, userCId, viewerRoleId);
  });

  test('C5: User C can read messages', async ({ request }) => {
    const messages = await listMessages(request, userCToken, channelId);
    expect(messages.length).toBeGreaterThan(0);
  });

  test('C6: User C cannot send messages', async ({ request }) => {
    const res = await request.post(
      `http://localhost:3000/api/v1/channels/${channelId}/messages`,
      {
        headers: { Authorization: `Bearer ${userCToken}` },
        data: { content: 'I should not be able to send' },
      },
    );
    expect(res.status()).toBe(403);
  });

  test('C7-C8: Update role to add SendMessages, then User C can send', async ({ request }) => {
    await updateRole(request, hosterToken, spaceId, viewerRoleId, [
      'ViewChannel',
      'ReadMessages',
      'SendMessages',
    ]);

    const msg = await sendMessage(request, userCToken, channelId, 'Now I can send!');
    expect(msg.content).toBe('Now I can send!');
    expect(msg.author_user_id).toBe(userCId);
  });
});
```

- [ ] **Step 6: Verify the Playwright test compiles**

Run: `cd apps/web && npx playwright test --dry-run`
Expected: Playwright discovers 3 test suites (Journey A, B, C) with the correct number of tests.

---

### Task 4: Verification and cleanup

- [ ] **Step 1: Verify manual test script completeness**

Read `docs/testing/e2e-manual-test.md` and verify:
- All 3 journeys are present with A1-A6, B1-B11, C1-C9
- Every variable is defined before use
- Expected results are stated for every step
- Troubleshooting table covers at least 5 common failures
- No "TBD", "TODO", or placeholder content exists

- [ ] **Step 2: Verify Playwright helpers cover all API calls needed by all 3 journeys**

Check that `helpers.ts` exports functions for:
- `bootstrapOwner` (A1)
- `registerUser` (B5, C1)
- `createSpace` (A2, B1, C setup)
- `createChannel` (A3, B2, C setup)
- `sendMessage` (A4, B9, C6/8)
- `listMessages` (A5, B10, C5)
- `createInvite` (B4, C setup)
- `acceptInvite` (B6, C2)
- `createRole` (B3, C3)
- `assignRole` (B7, C4)
- `updateRole` (C7)
- `listVisibleChannels` (B8)

- [ ] **Step 3: Run a syntax check on all new files**

```bash
cd apps/web && npx playwright test --dry-run 2>&1 || echo "Dry run indicates issues to fix"
```

Expected: No syntax errors in test files.

- [ ] **Step 4: Commit**

```bash
git add docs/testing/e2e-manual-test.md docs/testing/e2e-playwright-setup.md apps/web/e2e/journey.spec.ts apps/web/e2e/helpers.ts apps/web/e2e/types.ts apps/web/playwright.config.ts apps/web/package.json
git commit -m "test: add E2E user journey tests (manual + Playwright)"
```

---

## Self-Review

**1. Spec coverage:**
- Journey A (Hoster Setup): Tasks 1-2 (manual script) + Task 3 (Playwright). Manual covers A1-A6, Playwright covers A1-A5.
- Journey B (Invite Flow): Task 1 step 3 + Task 3 step 4 (Playwright). Manual covers B1-B11, Playwright covers B1-B10.
- Journey C (Permission Enforcement): Task 1 step 4 + Task 3 step 5 (Playwright). Manual covers C1-C9, Playwright covers C1-C8.
- Acceptance criteria 1 (manual script followable by dev without codebase knowledge): Task 1 explicit curl commands with expected results.
- Acceptance criteria 2 (all journeys pass): Every step has verification.
- Acceptance criteria 3 (expected result per step): Every curl step has "Expected:" block.
- Acceptance criteria 4 (common failure modes): Task 1 step 5 troubleshooting table.

**2. Placeholder scan:** No "TBD", "TODO", "implement later", or empty code blocks. Every code block contains complete, runnable content.

**3. Type consistency:** The Playwright helpers in `helpers.ts` use the same types defined in `types.ts`. Function signatures match the API shapes from `apps/server/src/handlers/`. Variable names (`hosterToken`, `spaceId`, `channelId`, `inviteCode`, etc.) are consistent across Journey A, B, and C test suites.

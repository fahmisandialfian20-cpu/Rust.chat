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

**Expected:** HTTP 200. Response contains `user.id`, `access_token`, `refresh_token`. If HTTP 409, instance already has an owner -- reset the database.

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

If all steps A1--A5 returned the expected results, **Journey A PASSES**.

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

If all steps B1--B10 returned the expected results, **Journey B PASSES**.

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

If all steps C1--C8 returned the expected results, **Journey C PASSES**.

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

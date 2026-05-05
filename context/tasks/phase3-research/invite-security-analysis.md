# Invite System Security Analysis

## File Summaries

### `apps/server/src/services/invite_service.rs`
The `InviteService` orchestrates invite operations: creation, retrieval, validation, consumption, and acceptance. It delegates to `InviteRepository` for persistence and to space/channel/role repositories for membership management. It enforces expiration-at-creation-time (must be in the future) and delegates expiration/max_uses checks to the repository's `is_valid` method. Critically, `create_invite` accepts `Uuid::nil()` as the acting user from the handler, and `accept_invite` is the only method that receives a real authenticated user ID.

### `apps/server/src/repositories/invite_repository.rs`
The `InviteRepository` handles all database operations for invites. It generates a 32-character hex code from a UUID v4, stores both the plaintext `code` and a SHA-256 `code_hash`, and looks up invites by `code_hash`. It provides `is_valid` which checks `max_uses` against `used_count` and `expires_at` against the current UTC time. It also increments `used_count` via a separate `UPDATE` query. The schema migration stores both `code` and `code_hash`, with a unique constraint on `code`.

### `apps/server/src/handlers/invites.rs`
The HTTP handlers expose invite functionality via REST. **Only `accept_invite` requires authentication** (uses `AuthUser`). All other endpoints—`create_invite`, `get_invite`, `get_invite_by_code`, `validate_invite`, `consume_invite`, `delete_invite`, and `list_space_invites`—are **unauthenticated**. The `create_invite` handler passes `Uuid::nil()` as the acting user to the service. Responses from get/list endpoints include the plaintext `code` field, leaking the invite secret.

### `apps/server/src/domain/invite.rs`
Defines the `Invite` and `CreateInvite` structs. `Invite` includes the plaintext `code` field, which is serialized into API responses. `CreateInvite` allows specifying `space_id`, `channel_id`, `max_uses`, and `expires_at`.

---

## Specific Questions

### 1. How is the invite token/code stored? Is it hashed?
- **Storage**: The code is stored **both as plaintext** (`code text UNIQUE NOT NULL`) **and as a hash** (`code_hash text NOT NULL`) in the database.
- **Hash function**: `hash_code` uses **SHA-256** (unsalted) on the raw code string.
- **Lookup**: `find_by_code` queries by `code_hash`, which is correct.
- **Leakage**: Despite hashing for lookup, the `Invite` domain struct retains the plaintext `code`, and API responses (e.g., `get_invite`, `get_invite_by_code`, `list_space_invites`) serialize and return it, defeating the purpose of hashing.

### 2. Does the invite have expiration? How is it enforced?
- **Yes**. `expires_at` is an optional `timestamptz` field.
- **Enforcement**: `InviteRepository::is_valid` checks `expires_at < OffsetDateTime::now_utc()` and returns `false` if expired. `InviteService::validate_invite` then returns `AppError::BadRequest("Invite is invalid or expired")`.
- **Creation validation**: `InviteService::create_invite` also rejects `expires_at` values in the past at creation time.

### 3. Does the invite have max_uses? How is used_count incremented?
- **Yes**. `max_uses` is an optional `int`.
- **Enforcement**: `is_valid` checks `used_count >= max_uses` and returns `false` if exceeded.
- **Increment**: `increment_used_count` runs a separate SQL `UPDATE invites SET used_count = used_count + 1 WHERE id = $1`. This is called by `consume_invite` and `accept_invite` **after** validation.

### 4. What happens when an expired invite is used? What error is returned?
- `validate_invite` calls `is_valid`, which returns `false` for expired invites.
- `validate_invite` then returns **`AppError::BadRequest("Invite is invalid or expired")`**.
- The same error message is used for both expired and max-uses-exceeded invites.

### 5. What happens when max_uses is reached? What error is returned?
- `is_valid` returns `false` when `used_count >= max_uses`.
- `validate_invite` returns **`AppError::BadRequest("Invite is invalid or expired")`**.
- Same error as expired invites; no distinction is made.

### 6. Is there any protection against brute-force guessing invite codes?
- **No explicit rate limiting** on any invite endpoint (`/invites/code/{code}`, `/invites/validate/{code}`, `/invites/consume/{code}`, `/invites/{code}/accept`).
- The code is a UUID v4 with dashes removed (32 hex characters = 128 bits of entropy), making random guessing practically infeasible.
- However, there is no API-level throttling to prevent automated scanning or enumeration attacks.

### 7. Are there tests for invite security?
- **Only one invite-related test exists**: `invite_accept_creates_membership` in `tests/permissions_test.rs`.
- This test verifies that accepting a valid invite creates a space membership and assigns the default role.
- It does **not** test expiration, max_uses, invalid codes, authentication requirements, or hash verification.
- No dedicated `invite_test.rs` file exists.

### 8. What security properties are NOT tested?
- ❌ Expired invite rejection
- ❌ Max uses exceeded / invite exhaustion
- ❌ Invalid/nonexistent invite code handling
- ❌ Hash verification (ensuring `code_hash` matches `code`)
- ❌ Authentication requirements on endpoints (anyone can create/delete/list invites)
- ❌ Race condition: concurrent `accept_invite` / `consume_invite` exceeding `max_uses`
- ❌ Plaintext code leakage in API responses
- ❌ `Uuid::nil()` used as acting user in `create_invite`

---

## Critical Vulnerabilities & Bugs

### A. Missing Authentication on Invite Endpoints
**Severity: High**
- `POST /api/v1/invites` — anyone can create invites for any space/channel.
- `GET /api/v1/invites/{invite_id}` — anyone can read invite details.
- `GET /api/v1/invites/code/{code}` — anyone can look up an invite by code and get the plaintext code back.
- `POST /api/v1/invites/consume/{code}` — anyone can consume an invite without being authenticated.
- `DELETE /api/v1/invites/{invite_id}` — anyone can delete any invite.
- `GET /api/v1/spaces/{space_id}/invites` — anyone can list all invites for a space.

### B. Plaintext Code Leakage in Responses
**Severity: High**
- The `Invite` struct includes `pub code: String` and is serialized directly into JSON responses.
- `get_invite`, `get_invite_by_code`, and `list_space_invites` all return the raw invite code, allowing anyone who can list or query invites to use them.

### C. Race Condition: max_uses Can Be Exceeded
**Severity: Medium**
- `validate_invite` and `increment_used_count` are separate, non-atomic operations.
- Under concurrent requests, multiple clients can pass validation when `used_count == max_uses - 1`, and all will increment, resulting in `used_count > max_uses`.
- The database does not enforce the `max_uses` constraint.

### D. Uuid::nil() Used as Acting User
**Severity: Medium**
- `create_invite` handler passes `Uuid::nil()` as the acting user, violating AGENTS.md Rule #6: "Real handlers must never use `Uuid::nil()` as the acting user."
- This breaks audit trails and could allow unauthorized invite creation.

### E. No Rate Limiting on Invite Validation/Consumption
**Severity: Low-Medium**
- While the code space is large (UUID v4), there is no rate limiting on `/invites/validate/{code}` or `/invites/consume/{code}`.
- Other handlers (auth, messages, files) use `rate_limit` middleware; invites do not.

---

## Recommendations

1. **Add authentication** to all invite handlers. `create_invite`, `delete_invite`, and `list_space_invites` must check the user's permissions (e.g., `ManageInvites`). `get_invite`, `get_invite_by_code`, `validate_invite`, and `consume_invite` should require authentication or be restricted.
2. **Remove `code` from API responses** for list/get endpoints. Only return the plaintext code upon initial creation.
3. **Fix `create_invite` handler** to extract the real `user_id` from `AuthUser` instead of `Uuid::nil()`.
4. **Make `consume_invite` / `accept_invite` atomic**. Use a single SQL statement that validates and increments in one transaction, or use `SELECT FOR UPDATE` to prevent the race condition.
5. **Add rate limiting** to invite validation/consumption endpoints.
6. **Add comprehensive invite security tests** covering expiration, max_uses, invalid codes, concurrent consumption, auth requirements, and code leakage.
7. **Return distinct error messages** for expired vs. max-uses-exceeded invites to improve client UX (optional, but helpful).

---

## Verdict

**Is the invite system secure? No.**

While the underlying code generation (UUID v4) and hashing (SHA-256 lookup) are sound, the system is critically undermined by:
- **No authentication** on most endpoints,
- **Plaintext code leakage** in API responses,
- **A race condition** allowing `max_uses` to be exceeded,
- **Use of `Uuid::nil()`** as the acting user in invite creation,
- **Zero security-focused tests** for negative cases (expired, exhausted, invalid, concurrent).

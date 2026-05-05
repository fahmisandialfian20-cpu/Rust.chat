# Task Context: Bugfix — 3 Failing Tests

## Goal

Fix **3 failing tests** discovered during Phase 2 verification. All tests compile; these are runtime logic bugs. After fixes, `cargo test` must show **0 failures**.

---

## Current Test Results (Docker PostgreSQL + Redis)

```bash
cargo test --test auth_test -- --test-threads=1       # 5/6 pass
cargo test --test permissions_test -- --test-threads=1 # 12/14 pass
```

**Total: 17/20 passing (85%). 3 failures remain.**

---

## Bug 1: Login Wrong Password Returns 500 (Not 401)

**Test:** `auth_test.rs` — `login_with_wrong_password_returns_unauthorized`

**Expected:** HTTP 401 Unauthorized
**Actual:** HTTP 500 Internal Server Error

**Likely Root Cause:**

In `services/auth_service.rs`, `login()` method:

```rust
let user = self.user_repo.find_by_username_or_email(&username_or_email).await?;
let password_hash = user.password_hash.as_deref().unwrap_or("");
let valid = verify_password(&password, &self.config.auth.password_pepper, password_hash)?;
```

- If `verify_password` returns `Err` (e.g., invalid hash format, empty hash) instead of `Ok(false)`, it propagates as `InternalServerError`.
- OR: `find_by_username_or_email` might return a user whose `password_hash` is `None` or malformed in a way that causes `verify_password` to error.

**Files to inspect:**
- `apps/server/src/services/auth_service.rs` (login method)
- `apps/server/src/auth/password.rs` (verify_password implementation)
- `apps/server/src/repositories/user_repository.rs` (find_by_username_or_email)

**Fix strategy:**
Ensure `verify_password` returns `Ok(false)` for wrong passwords (not `Err`). If it returns `Err`, catch it and return `AppError::Unauthorized` instead of propagating as 500.

---

## Bug 2: Feature Flag Disabled Does Not Deny `SendMessages`

**Test:** `permissions_test.rs` — `feature_flag_disabled_returns_denied`

**Expected:** `PermissionService::check(SendMessages)` returns `Forbidden` with message containing "Feature not enabled"
**Actual:** Test assertion fails (permission is granted)

**Root Cause (Confirmed):**

In `permissions/resolver.rs`, `check_layer5_feature_flags`:

```rust
let allowed = match permission {
    PermissionKey::SendFiles => flags.send_file_enabled,
    PermissionKey::JoinVoice | PermissionKey::StartVoice => flags.voice_group_enabled,
    PermissionKey::JoinVideo | PermissionKey::StartVideo | PermissionKey::ShareScreen => {
        flags.video_group_enabled
    }
    _ => true,  // ← SendMessages falls here, always true!
};
```

`PermissionKey::SendMessages` is NOT matched — it falls to `_ => true`, so feature flags never block `SendMessages`.

The test inserts `send_file_enabled = false` but checks `SendMessages`, which is the wrong mapping. Either:
- The resolver needs a `send_messages_enabled` flag check, OR
- The test should check `SendFiles` instead of `SendMessages`

**Check migration `0010_channel_feature_flags.sql`** to see what columns exist.

**Fix strategy:**
- Add `PermissionKey::SendMessages => flags.send_messages_enabled` (or equivalent column) to the match, OR
- If no such column exists, the test is checking the wrong permission key and should be updated to check `SendFiles` (which IS in the match).

**Files to inspect:**
- `apps/server/src/permissions/resolver.rs` (check_layer5_feature_flags)
- `apps/server/migrations/0010_channel_feature_flags.sql` (column names)
- `apps/server/src/permissions/repository.rs` (get_channel_feature_flags return type)

---

## Bug 3: Invite Accept Fails

**Test:** `permissions_test.rs` — `invite_accept_creates_membership`

**Expected:** `invite_service.accept_invite(code, new_user)` succeeds, creates membership
**Actual:** Test assertion panics at `assert!(result.is_ok())` — invite accept fails

**Likely Root Cause:**

The test creates an invite via raw SQL:
```rust
sqlx::query("INSERT INTO invites (id, space_id, code, code_hash, created_by, max_uses, used_count) VALUES ($1, $2, $3, $4, $5, 10, 0)")
    .bind(invite_id)
    .bind(space_id)
    .bind(code)          // "test-invite-123"
    .bind("hash")        // ← code_hash is "hash", not a real hash of the code!
    .bind(hoster)
    .execute(&pool)
    .await
    .unwrap();
```

`invite_service.validate_invite(code)` calls `repository.is_valid(code)`, which likely hashes the provided code and compares with `code_hash`. Since `code_hash` is the literal string `"hash"` (not a hash of `"test-invite-123"`), validation fails.

**OR:** The invite might be missing `expires_at` and `is_valid` checks something else that fails.

**Fix strategy:**
- Either generate a proper `code_hash` in the test (using the same hashing function as the repository), OR
- Create the invite via `InviteService::create_invite` instead of raw SQL (recommended — this ensures all fields are set correctly).

**Files to inspect:**
- `apps/server/src/services/invite_service.rs` (accept_invite, validate_invite)
- `apps/server/src/repositories/invite_repository.rs` (is_valid, find_by_code)
- `apps/server/tests/permissions_test.rs` (invite_accept_creates_membership test)

---

## Scope

Fix these 3 bugs so all tests pass. Minimal changes. Do not refactor unrelated code.

**Files allowed to change:**
```text
apps/server/src/services/auth_service.rs          # Bug 1: handle verify_password error
apps/server/src/auth/password.rs                  # Bug 1: if verify_password panics/returns Err
apps/server/src/permissions/resolver.rs           # Bug 2: add SendMessages to feature flag check
apps/server/tests/permissions_test.rs             # Bug 2: fix test if wrong permission used
apps/server/tests/permissions_test.rs             # Bug 3: create invite properly via service
```

**Possibly inspect (read-only):**
```text
apps/server/src/repositories/invite_repository.rs # Bug 3: understand is_valid logic
apps/server/migrations/0010_channel_feature_flags.sql # Bug 2: column names
```

---

## Non-Goals

- Do NOT add new features.
- Do NOT modify frontend.
- Do NOT change test infrastructure (`tests/common/mod.rs`).
- Do NOT add new tests.
- Do NOT change unrelated handlers or services.

---

## Expected Behavior After Fix

```bash
cd apps/server
cargo test --test auth_test -- --test-threads=1
# test result: ok. 6 passed; 0 failed

cargo test --test permissions_test -- --test-threads=1
# test result: ok. 14 passed; 0 failed
```

---

## Verification Commands

```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test --test auth_test -- --test-threads=1
cargo test --test permissions_test -- --test-threads=1
```

---

## Stop Conditions

Stop and ask for review if:
- Fixing one bug introduces a new failure.
- `verify_password` or `is_valid` logic is more complex than expected.
- A fix requires changing more than 2 files.
- Feature flag columns don't exist in the database schema.

---

## Success Criteria

- [ ] `cargo test --test auth_test` — 6/6 pass
- [ ] `cargo test --test permissions_test` — 14/14 pass
- [ ] `cargo clippy -- -D warnings` — 0 errors
- [ ] `cargo fmt --check` — clean
- [ ] `progress-tracker.md` updated: Permission Boundary Tests 🟢 Done (20/20 pass)

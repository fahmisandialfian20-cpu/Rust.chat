# Implementation Plan: Foundation Repair

## Task Reference

- **Context:** `context/tasks/foundation-repair.md`
- **Goal:** Fix test compilation and clippy errors to establish a stable backend foundation.
- **Phase:** MVP Core Stabilization — Step 1 of 3

---

## Overview

The backend compiles (`cargo check` ✅) but cannot run tests (`cargo test` ❌) and fails linting (`cargo clippy -- -D warnings` ❌). This plan fixes both issues in **two sequential batches** to minimize risk.

**Batch 1:** Fix test infrastructure (`tests/common/mod.rs`)
**Batch 2:** Fix clippy errors across `src/`

---

## Batch 1: Fix Test Infrastructure

### Step 1.1 — Update `AppConfig` in `tests/common/mod.rs`

Add missing `livekit` and `rate_limit` fields to match `config.rs`.

```rust
livekit: rust_chat_server::config::LiveKitConfig {
    enabled: false,
    url: "ws://localhost:7880".to_string(),
    api_key: "devkey".to_string(),
    api_secret: "secret".to_string(),
},
rate_limit: rust_chat_server::config::RateLimitConfig {
    login: 5,
    register: 5,
    message_send: 30,
    file_upload: 10,
    ws_connect: 20,
},
```

### Step 1.2 — Add missing repository instantiations

Instantiate `RoleRepository` and `AuditRepository` in `tests/common/mod.rs`.

```rust
let role_repo = Arc::new(RoleRepository::new(pool.clone()));
let audit_repo = AuditRepository::new(pool.clone());
```

### Step 1.3 — Fix `SpaceService::new` call

Change from 1 argument to 2:

```rust
// Before
space_service: SpaceService::new(space_repo),

// After
space_service: SpaceService::new(space_repo, role_repo.clone()),
```

### Step 1.4 — Fix `InviteService::new` call

Change from 1 argument to 4:

```rust
// Before
invite_service: InviteService::new(invite_repo),

// After
invite_service: InviteService::new(
    invite_repo,
    space_repo.clone(),
    channel_repo.clone(),
    role_repo.clone(),
),
```

### Step 1.5 — Instantiate missing services for `AppState`

```rust
let permission_service = PermissionService::new(pool.clone());
let audit_service = AuditService::new(audit_repo, permission_service.clone());
let rate_limiter = RateLimiter::new(redis_conn.clone());
let role_service = RoleService::new(role_repo, permission_service.clone());
```

### Step 1.6 — Complete `AppState` initialization

Add the 4 missing fields to `AppState { ... }`:

```rust
permission_service,
audit_service,
rate_limiter,
role_service,
```

### Step 1.7 — Verify Batch 1

```bash
cd apps/server
cargo check
cargo test --no-run   # compile tests without running
```

**Stop if:** New compilation errors appear outside `tests/common/mod.rs`.

---

## Batch 2: Fix Clippy Errors

### Step 2.1 — Remove unused imports

| File | Line | Action |
|------|------|--------|
| `src/permissions/repository.rs` | 4 | Remove `use super::keys::PermissionKey;` |
| `src/permissions/resolver.rs` | 3 | Remove `ChannelFeatureFlags` from import |

### Step 2.2 — Remove unused `mut`

| File | Line | Action |
|------|------|--------|
| `src/handlers/files.rs` | 63 | Change `mut field` to `field` |

### Step 2.3 — Prefix unused variables/fields with `_`

| File | Line | Action |
|------|------|--------|
| `src/realtime/hub.rs` | 25 | `channel_id` → `_channel_id` |
| `src/auth/session.rs` | 40 | `db_pool` → `_db_pool` |
| `src/handlers/media.rs` | 20 | `client_type` → `_client_type` |
| `src/handlers/profile.rs` | 149-154 | `user_id`, `created_at`, `updated_at` → prefix with `_` |
| `src/permissions/repository.rs` | 203 | `channel_id` → `_channel_id` |
| `src/repositories/invite_repository.rs` | 205 | `code_hash` → `_code_hash` |

### Step 2.4 — Derive `Default` for enums

| File | Enum | Change |
|------|------|--------|
| `src/domain/space.rs` | `SpaceVisibility` | Add `#[derive(Default)]`, mark `Private` with `#[default]`, remove manual `impl Default` |
| `src/domain/channel.rs` | `ChannelKind` | Add `#[derive(Default)]`, mark `Text` with `#[default]`, remove manual `impl Default` |
| `src/domain/channel.rs` | `ChannelVisibility` | Add `#[derive(Default)]`, mark `Public` with `#[default]`, remove manual `impl Default` |

### Step 2.5 — Collapsible match/if fixes

| File | Action |
|------|--------|
| `src/handlers/profile.rs:99-104` | Flatten nested `if let` into single pattern match |
| `src/permissions/resolver.rs:115-118` | Collapse nested `if` into single condition with `&&` |

### Step 2.6 — Needless borrows in `.bind()`

Remove `&` prefix from `.bind()` arguments where the generic trait is satisfied by value:

| File | Lines |
|------|-------|
| `src/repositories/channel_repository.rs` | 38 (`&parent_id`), 267 (`&parent_id`) |
| `src/repositories/invite_repository.rs` | 37 (`&space_id`), 38 (`&channel_id`), 40 (`&max_uses`), 41 (`&expires_at`) |
| `src/repositories/message_repository.rs` | 37 (`&reply_to_message_id`) |
| `src/repositories/file_repository.rs` | 36 (`&space_id`), 37 (`&channel_id`) |

### Step 2.7 — Double-ended iterator fix

| File | Line | Change |
|------|------|--------|
| `src/services/typing_service.rs` | 67 | `.last()` → `.next_back()` |

### Step 2.8 — Redundant closures

| File | Lines | Change |
|------|-------|--------|
| `src/services/file_service.rs` | 69, 93, 109 | `|e| AppError::InternalServerError(e)` → `AppError::InternalServerError` |

### Step 2.9 — Too many arguments

**Option A (preferred for repository create methods):** Add `#[allow(clippy::too_many_arguments)]` to the following functions:

| File | Function |
|------|----------|
| `src/repositories/channel_repository.rs` | `create` (10 args) |
| `src/repositories/channel_repository.rs` | `update_feature_flags` (8 args) |
| `src/repositories/file_repository.rs` | `create` (8 args) |
| `src/services/audit_service.rs` | `log` (9 args) |

**Rationale:** These are data-mapping functions that mirror database tables. Refactoring into structs adds boilerplate without clarity. The lint is explicitly allowed.

### Step 2.10 — Verify Batch 2

```bash
cd apps/server
cargo clippy -- -D warnings
cargo test
cargo fmt --check
```

**Stop if:** Any fix introduces a new error or changes runtime behavior.

---

## Verification Checklist

After both batches:

```bash
# Backend
cd apps/server
cargo fmt --check        # must pass
cargo clippy -- -D warnings  # must pass
cargo test               # must compile and run

# Frontend
cd apps/web
npm run check            # must still pass
```

---

## Rollback Strategy

If any step introduces regressions:

1. Revert the specific file to its original state using git.
2. Re-run `cargo check` to confirm baseline.
3. Re-apply the step more carefully.
4. Do NOT proceed to the next batch until the current batch is clean.

---

## Time Estimate

| Batch | Estimated Time |
|-------|---------------|
| Batch 1: Test infrastructure | 15-20 minutes |
| Batch 2: Clippy errors | 30-45 minutes |
| Verification & cleanup | 10-15 minutes |
| **Total** | **~1 hour** |

---

## Post-Completion Actions

1. Update `context/progress-tracker.md`:
   - Backend Unit Tests: 🔴 → 🟡 Partial (compiles, may need expansion)
   - Clippy/Linting: 🔴 → 🟢 Done

2. Do NOT commit unless explicitly requested by the user.

3. Report completion with:
   - Files changed
   - Commands run and results
   - Known limitations (e.g., tests require TEST_DATABASE_URL and Redis)

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Constructor signatures change again before tests run | Low | High | Verify against `main.rs` before each edit |
| `cargo test` requires running PostgreSQL/Redis | Medium | Medium | User must ensure `TEST_DATABASE_URL` is set; tests will skip if DB unreachable |
| Clippy fix changes behavior | Low | Medium | Review each change; prefer `#[allow]` over risky refactoring |
| Too many files touched | Medium | Low | Group by batch; verify after each batch |

---

## Context7-Verified Practices Applied

- **Rust testing organization:** Integration tests use `tests/common/mod.rs` for shared setup (Rust standard pattern).
- **Axum state injection:** `Router::with_state(state)` passes `AppState` to handlers; state must be fully initialized.
- **SQLx migrations:** `Migrator::new(migrations_dir).await?.run(&pool).await?` is the standard approach for test database setup.
- **Clippy conventions:** `#[derive(Default)]` on enums, removing needless borrows, and collapsing nested conditionals are all idiomatic Rust practices enforced by clippy.

---

## Next Phase (After This Plan)

Once this plan completes:

**Phase 2:** Permission Boundary Tests — Write the 10 security scenario tests defined in `03-domain-permissions.md` to prove unauthorized access is rejected.

**Phase 3:** End-to-End Feature Hardening — Frontend channel visibility, WebSocket permission review, Hoster bootstrap E2E test.

**Do not start Phase 2 until this plan is fully verified.**

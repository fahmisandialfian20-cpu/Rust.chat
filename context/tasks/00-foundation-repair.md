# Task Context: Foundation Repair

## Goal

Repair the backend's **test infrastructure** and **code quality** to establish a stable foundation for all future MVP Core Stabilization work. The backend must compile cleanly, pass clippy with `-D warnings`, and run its test suite without errors.

**Verdict from verification (Context7-validated):**
- `cargo check` ✅ passes
- `cargo test` ❌ **FAIL** — 4 compilation errors in `tests/common/mod.rs`
- `cargo clippy -- -D warnings` ❌ **FAIL** — 31 errors across src/
- `npm run check` ✅ passes (frontend is clean)

---

## Scope

### 1. Fix Test Infrastructure (`tests/common/mod.rs`)

The test helper `setup_test_app()` is outdated and does not match the current production constructor signatures. Four categories of errors exist:

**a. `AppConfig` incomplete**
- Missing `livekit: LiveKitConfig`
- Missing `rate_limit: RateLimitConfig`
- Must match `config.rs` exactly.

**b. `SpaceService::new` signature changed**
- Old: `SpaceService::new(space_repo)` — 1 arg
- Current: `SpaceService::new(space_repo, role_repo)` — 2 args (verified in `services/space_service.rs:17`)

**c. `InviteService::new` signature changed**
- Old: `InviteService::new(invite_repo)` — 1 arg
- Current: `InviteService::new(invite_repo, space_repo, channel_repo, role_repo)` — 4 args (verified in `services/invite_service.rs:20-25`)

**d. `AppState` incomplete**
- Missing `audit_service: AuditService`
- Missing `permission_service: PermissionService`
- Missing `rate_limiter: RateLimiter`
- Missing `role_service: RoleService`
- Must mirror `main.rs:110-128` exactly.

**Context7 reference:** Rust integration tests should use a shared `common` module for setup. Axum `Router` uses `.with_state(state)` to inject `AppState` into handlers. SQLx migrations can be run via `sqlx::migrate::Migrator` before test execution.

### 2. Fix Clippy Errors (31 errors across src/)

Categories of clippy errors found:

| Category | Count | Files Affected | Fix Strategy |
|----------|-------|----------------|--------------|
| Unused imports | 2 | `permissions/repository.rs`, `permissions/resolver.rs` | Remove |
| Unused `mut` | 1 | `handlers/files.rs:63` | Remove `mut` |
| Unused variables/fields | 5 | `realtime/hub.rs`, `auth/session.rs`, `handlers/media.rs`, `handlers/profile.rs`, `permissions/repository.rs`, `repositories/invite_repository.rs` | Prefix with `_` or remove |
| Derivable impls | 3 | `domain/space.rs`, `domain/channel.rs` (×2) | Add `#[derive(Default)]` |
| Collapsible match/if | 2 | `handlers/profile.rs`, `permissions/resolver.rs` | Flatten nested blocks |
| Too many arguments | 3 | `repositories/channel.rs` (×2), `repositories/file.rs`, `services/audit.rs` | Refactor to struct params OR allow lint |
| Needless borrows | 6 | `repositories/channel.rs`, `repositories/invite.rs`, `repositories/message.rs`, `repositories/file.rs` | Remove `&` in `.bind(&x)` |
| Double-ended iterator `last()` | 1 | `services/typing_service.rs:67` | Use `.next_back()` |
| Redundant closures | 3 | `services/file_service.rs` (×3) | Pass variant directly |

**Context7 reference:** Rust clippy enforces idiomatic code. The `derivable_impls` lint suggests `#[derive(Default)]` on enums with a clear default variant. `needless_borrows_for_generic_args` indicates that `sqlx::query!().bind(&x)` can be simplified to `.bind(x)` when the trait bound is satisfied by value.

---

## Non-Goals

- Do NOT add new features or business logic.
- Do NOT modify frontend code (already passing).
- Do NOT refactor service architecture (only fix constructor signatures to match).
- Do NOT write new tests beyond fixing existing compilation.
- Do NOT add LiveKit, mobile, desktop, themes, or any out-of-scope work.

---

## Files to Inspect

```text
apps/server/tests/common/mod.rs           ← primary target for test fixes
apps/server/src/main.rs                   ← reference for AppState construction
apps/server/src/config.rs                 ← reference for AppConfig fields
apps/server/src/state.rs                  ← reference for AppState fields
apps/server/src/services/space_service.rs ← verify SpaceService::new signature
apps/server/src/services/invite_service.rs ← verify InviteService::new signature
apps/server/src/services/audit_service.rs ← verify AuditService::new signature
apps/server/src/services/role_service.rs  ← verify RoleService::new signature
apps/server/src/permissions/service.rs    ← verify PermissionService::new signature
apps/server/src/middleware/rate_limit.rs  ← verify RateLimiter::new signature
apps/server/src/repositories/role_repository.rs ← needed for role_repo instantiation
apps/server/src/repositories/audit_repository.rs ← needed for audit_repo instantiation
```

---

## Files Allowed to Change

```text
apps/server/tests/common/mod.rs           ← must fix to compile
apps/server/src/permissions/repository.rs ← remove unused import
apps/server/src/permissions/resolver.rs   ← remove unused import, fix collapsible-if
apps/server/src/handlers/files.rs         ← remove needless mut
apps/server/src/realtime/hub.rs           ← prefix unused param
apps/server/src/auth/session.rs           ← prefix or remove dead field
apps/server/src/handlers/media.rs         ← prefix dead field
apps/server/src/handlers/profile.rs       ← prefix dead fields, fix collapsible-match
apps/server/src/permissions/repository.rs ← prefix dead field
apps/server/src/repositories/invite_repository.rs ← prefix dead field
apps/server/src/domain/space.rs           ← derive Default
apps/server/src/domain/channel.rs         ← derive Default (×2 enums)
apps/server/src/repositories/channel_repository.rs ← fix needless borrows, too-many-args
apps/server/src/repositories/invite_repository.rs ← fix needless borrows
apps/server/src/repositories/message_repository.rs ← fix needless borrows
apps/server/src/repositories/file_repository.rs ← fix needless borrows, too-many-args
apps/server/src/services/typing_service.rs ← fix double-ended iterator
apps/server/src/services/file_service.rs  ← fix redundant closures
apps/server/src/services/audit_service.rs ← too-many-args (allow lint or refactor)
```

**Note:** For `too_many_arguments` lint, prefer adding `#[allow(clippy::too_many_arguments)]` only if the function is a repository `create` that maps 1:1 to a database table. Otherwise, refactor into a struct parameter.

---

## Expected Behavior

After changes:

1. `cd apps/server && cargo test` compiles and runs all tests.
2. `cd apps/server && cargo clippy -- -D warnings` reports zero errors.
3. `cd apps/server && cargo fmt --check` reports zero formatting issues.
4. `cd apps/web && npm run check` continues to pass (0 errors).
5. No behavioral changes to production code — only compilation and lint fixes.

---

## Tests

The existing tests will serve as validation once compilation is fixed:

- `tests/auth_test.rs` — bootstrap, login, logout, token access
- `tests/permissions_test.rs` — hoster bypass, non-member denial, role allow/deny, feature flags, `has_any_permission`

No new tests needed for this task.

---

## Verification Commands

Run these in order after every change batch:

```bash
cd apps/server
cargo check
cargo clippy -- -D warnings
cargo test --no-run          # compile tests without running
cargo fmt --check
```

**Note:** `cargo test` (runtime) requires `TEST_DATABASE_URL` to be set and PostgreSQL to be reachable.
For Supabase: set `TEST_DATABASE_URL=postgresql://postgres:<password>@db.<ref>.supabase.co:5432/postgres`

And verify frontend is still clean:

```bash
cd apps/web
npm run check
```

---

## Stop Conditions

Stop and ask for review if:
- Fixing one error introduces new errors in unrelated modules.
- A constructor signature change requires updating more than `tests/common/mod.rs`.
- The `cargo test` environment requires `TEST_DATABASE_URL` or Redis and cannot be satisfied.
- Any fix would change production behavior (not just compilation).
- You are tempted to refactor beyond the scope of compilation/lint fixes.

---

## Context7 References Used

- **Rust testing**: Integration tests use `tests/common/mod.rs` for shared setup; `#[cfg(test)]` modules for unit tests.
- **Axum state**: `Router::with_state(state)` injects `AppState`; state is cloned per request; use `Arc` for expensive types.
- **SQLx tests**: `#[sqlx::test(migrator = "...")]` can auto-run migrations; `sqlx::migrate::Migrator::new(path)` for manual migration runs.
- **Tokio tests**: `#[tokio::test]` macro executes async functions in the Tokio runtime.
- **Clippy**: `derivable_impls`, `needless_borrows_for_generic_args`, `collapsible_if`, `redundant_closure` are standard lints for idiomatic Rust.

---

## Success Criteria

- [ ] `tests/common/mod.rs` compiles and matches production constructor patterns.
- [ ] `cargo clippy -- -D warnings` exits with code 0.
- [ ] `cargo test` compiles and runs (tests may pass or fail logically; compilation must succeed).
- [ ] `cargo fmt --check` exits with code 0.
- [ ] `progress-tracker.md` updated: Testing & Quality → Backend Unit Tests 🟡 Partial, Clipping 🟢 Done.

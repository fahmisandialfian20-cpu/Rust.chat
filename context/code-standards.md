# Code Standards

## Philosophy

- Backend is the source of truth.
- Handlers stay thin; business logic lives in services.
- Permission checks happen on the backend, never trust the frontend.
- Keep changes small and scoped.
- Follow existing patterns; do not introduce new conventions without reason.

---

## Rust Backend

### Architecture

```text
handler      → receives request, extracts auth, calls service
  ↓
service      → business logic, permission checks, coordinates repositories
  ↓
repository   → SQL queries, database access
  ↓
domain       → structs, enums, validation rules
```

### Rules

1. **Handlers must be thin.**
   - Parse input.
   - Extract authenticated user from request extensions.
   - Call service method.
   - Return response.
   - No business logic in handlers.

2. **Never use `Uuid::nil()` as the acting user in real handlers.**
   - Every protected endpoint must resolve the real authenticated user.

3. **Permission checks belong in services.**
   - Use `PermissionService` or equivalent.
   - Check permissions before mutations.
   - Check both role permissions and channel feature flags.

4. **Return structured errors.**
   - Use the project's `error.rs` module.
   - Map domain errors to HTTP status codes consistently.

5. **Database access through repositories.**
   - Do not write raw SQL in handlers.
   - Do not write raw SQL in services.
   - Repositories own all SQLx queries.

6. **Use transactions for multi-step mutations.**
   - Creating a space + adding membership.
   - Accepting an invite + creating membership.

7. **Migrations are additive.**
   - Never edit a deployed migration file.
   - Create a new migration for schema changes.
   - Name migrations descriptively: `0013_messages.sql`.

8. **Feature flags are not permissions.**
   - Correct rule: `permission allows AND feature flag allows`.
   - Check both independently.

### Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and resolve issues.
- Prefer `thiserror` for error enums.
- Prefer ` anyhow` for early-return / bubbling in handlers.
- Use `tracing` for structured logging; include `trace_id` and `user_id` where relevant.
- Keep functions under 50 lines when possible.
- Module layout:
  ```text
  src/
    main.rs
    lib.rs
    config.rs
    state.rs
    error.rs
    auth/
    domain/
    handlers/
    services/
    repositories/
    permissions/
    realtime/
    storage/
    middleware/
    routes/
    docs/
    telemetry.rs
  ```

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Structs | PascalCase | `SpaceMember`, `ChannelOverride` |
| Functions / methods | snake_case | `create_space`, `can_send_messages` |
| Modules / files | snake_case | `space_service.rs`, `auth_middleware.rs` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_INVITE_USES` |
| Environment variables | SCREAMING_SNAKE_CASE | `DATABASE_URL` |
| Database tables | snake_case, plural | `space_memberships`, `role_permissions` |
| Database columns | snake_case | `created_at`, `space_id` |

---

## SvelteKit Frontend

### Architecture

- The frontend renders UI and calls backend APIs.
- The frontend must not talk directly to PostgreSQL, Redis, or storage secrets.
- Frontend permission checks are UI convenience only.

### Rules

1. **Call backend for all data.**
   - Do not mock or fake data that hides backend issues.
   - Show empty states honestly.

2. **Hide unauthorized actions in UI, but do not rely on it for security.**
   - If the user lacks `SendMessages`, hide the input box.
   - The backend will reject it anyway if they try.

3. **Use Svelte 5 runes.**
   - `$state`, `$derived`, `$effect` for reactivity.
   - Avoid legacy `$:` reactivity where possible.

4. **TypeScript everywhere.**
   - Strict mode enabled.
   - Define API response types in a shared location.
   - Validate API responses with `zod` when crossing trust boundaries.

5. **Keep pages simple.**
   - Extract components for repeated UI.
   - Keep server load functions in `+page.server.ts` or `+page.ts`.
   - Handle errors gracefully; show user-friendly messages.

6. **Use Tailwind CSS for styling.**
   - Prefer utility classes.
   - Keep custom CSS minimal.
   - Avoid inline styles.

### Code Style

- Run `npm run check` before committing.
- Run `npm run build` to verify production build.
- Prefer `async/await` over raw promises.
- Use `lucide-svelte` for icons.
- Keep Svelte components under 200 lines; extract if growing larger.
- Group imports: Svelte → libraries → project modules → types.

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Components | PascalCase | `ChannelList.svelte`, `UserAvatar.svelte` |
| Routes / files | kebab-case | `+page.svelte`, `+layout.svelte` |
| Functions | camelCase | `sendMessage`, `joinSpace` |
| Stores / state | camelCase | `currentUser`, `activeChannel` |
| Types / interfaces | PascalCase | `ApiError`, `ChannelResponse` |
| Constants | SCREAMING_SNAKE_CASE | `API_BASE_URL` |

---

## Database

### Schema Rules

1. Use `uuid` primary keys (`gen_random_uuid()` or `uuid_generate_v4()`).
2. Use `timestamptz` for all timestamp columns.
3. Name foreign key columns `{table}_id` (e.g., `space_id`, `user_id`).
4. Add indexes on foreign keys and frequently queried columns.
5. Use `ON DELETE` behavior explicitly (`CASCADE`, `SET NULL`, `RESTRICT`).
6. Store enums as `TEXT` with CHECK constraints or as lookup tables.
7. Keep migrations reversible where possible (additions preferred over destructive changes).

### Example Migration Template

```sql
-- {timestamp}_{description}.sql
-- Up
CREATE TABLE example_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_example_items_name ON example_items(name);

-- Down
-- DROP TABLE example_items;
```

---

## API & WebSocket

### REST

- Use plural nouns for resource paths: `/spaces`, `/channels`, `/messages`.
- Use nested paths for relationships: `/spaces/{id}/channels`.
- Return consistent error shape:
  ```json
  {
    "error": "ErrorCode",
    "message": "Human-readable description"
  }
  ```
- Use appropriate HTTP status codes: `200`, `201`, `204`, `400`, `401`, `403`, `404`, `409`, `422`, `500`.

### WebSocket

- WebSocket events must follow the same permission rules as REST endpoints.
- Authenticate the connection before accepting events.
- Reject unauthorized events silently or with a discrete error event.
- Do not broadcast sensitive data to unauthorized subscribers.

---

## Security Checklist

Before merging any protected feature:

- [ ] Backend validates authenticated user context.
- [ ] `PermissionService` or equivalent checks the action.
- [ ] Private channels are not visible to unauthorized users.
- [ ] Message read/send/edit/delete is permission-checked.
- [ ] WebSocket events use the same permission rules as REST.
- [ ] No `Uuid::nil()` used as real acting user.
- [ ] No secrets logged or returned to frontend.
- [ ] Rate limiting applies to auth and invite endpoints.

---

## Verification Commands

Run these before claiming completion:

**Backend:**
```bash
cd apps/server
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

**Frontend:**
```bash
cd apps/web
npm install
npm run check
npm run build
```

**Infrastructure:**
```bash
docker compose -f infra/docker-compose.dev.yml config
```

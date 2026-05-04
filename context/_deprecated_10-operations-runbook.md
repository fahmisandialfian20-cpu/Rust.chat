# Operations Runbook

## 1. Local development goal

A new developer or agent should be able to run the app locally with:

```bash
docker compose -f infra/docker-compose.dev.yml up
```

Then:

- backend on `http://localhost:8080`;
- frontend on `http://localhost:3000`;
- PostgreSQL on `localhost:5432`;
- Redis on `localhost:6379`.

## 2. Environment variables

```env
APP_ENV=development
APP_PUBLIC_URL=http://localhost:3000

SERVER_HOST=0.0.0.0
SERVER_PORT=8080

DATABASE_URL=postgres://chatapp:chatapp@postgres:5432/chatapp
REDIS_URL=redis://redis:6379

SESSION_SECRET=change-me-min-64-chars
PASSWORD_PEPPER=change-me

STORAGE_PROVIDER=local
LOCAL_STORAGE_DIR=/data/uploads

LIVEKIT_ENABLED=false
LIVEKIT_URL=ws://livekit:7880
LIVEKIT_API_KEY=devkey
LIVEKIT_API_SECRET=secret

PIKPAK_ADAPTER_ENABLED=false
PIKPAK_ADAPTER_URL=http://pikpak-adapter:9000
PIKPAK_ADAPTER_SECRET=change-me
```

## 3. Backend commands

```bash
cd apps/server
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run
```

SQLx:

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
sqlx database create
sqlx migrate run
```

## 4. Frontend commands

```bash
cd apps/web
pnpm install
pnpm check
pnpm lint
pnpm test
pnpm dev
pnpm build
```

## 5. Health endpoints

```http
GET /healthz
GET /readyz
```

`/healthz` checks process health.

`/readyz` checks:

- PostgreSQL connection;
- Redis connection;
- storage provider configuration if enabled.

## 6. Bootstrap first owner

```bash
curl -X POST http://localhost:8080/api/v1/auth/bootstrap-owner   -H "content-type: application/json"   -d '{
    "username": "hoster",
    "email": "hoster@example.com",
    "password": "change-this-password",
    "instance_name": "Rust.chat Local"
  }'
```

This endpoint must fail after the owner exists.

## 7. Reverse proxy requirements

WebSocket must support upgrade headers.

Nginx example:

```nginx
location /api/v1/ws {
  proxy_pass http://server:8080;
  proxy_http_version 1.1;
  proxy_set_header Upgrade $http_upgrade;
  proxy_set_header Connection "upgrade";
  proxy_set_header Host $host;
}
```

## 8. Production checklist

- [ ] TLS enabled.
- [ ] Strong `SESSION_SECRET`.
- [ ] Strong `PASSWORD_PEPPER`.
- [ ] Postgres not exposed publicly.
- [ ] Redis not exposed publicly.
- [ ] CORS locked to frontend origin.
- [ ] Bootstrap owner endpoint disabled after setup.
- [ ] Upload limits configured.
- [ ] Object storage configured.
- [ ] Audit logs enabled.
- [ ] Rate limits enabled.
- [ ] LiveKit secrets server-only.
- [ ] Backups configured.

## 9. Backup

Minimum backup plan:

- PostgreSQL daily dump;
- `.env` stored safely outside repo;
- storage provider backup/retention policy;
- audit logs retained;
- Redis does not need full durable backup for MVP.

## 10. Logs

Production logs should be structured JSON.

Fields:

- timestamp;
- level;
- request_id;
- user_id if available;
- action;
- route;
- status_code;
- latency_ms;
- error_code.

## 11. Failure diagnosis

### User cannot see channel

Check:

1. user is member of space;
2. membership status is active;
3. channel is not archived;
4. role has `view_channel`;
5. channel override does not deny;
6. private channel has explicit allow or accepted invite.

### Message not sent

Check:

1. WebSocket connected;
2. user can view channel;
3. user has `send_messages`;
4. channel `text_enabled` is true;
5. user is not muted/banned;
6. rate limit not exceeded.

### File upload fails

Check:

1. user has `send_files`;
2. channel `send_file_enabled` is true;
3. file size below limit;
4. MIME allowed;
5. storage provider healthy;
6. attachment metadata inserted.

### Voice/video fails

Check:

1. LiveKit enabled;
2. LiveKit URL reachable;
3. server has API key/secret;
4. channel feature enabled;
5. user has media permission;
6. token generated server-side.

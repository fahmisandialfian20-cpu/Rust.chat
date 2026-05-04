# Operations Runbook

## 1. Local development services

Use Docker Compose for:

- PostgreSQL;
- Redis;
- optional LiveKit;
- optional PikPak adapter.

## 2. Local development clients

Run separately:

```text
apps/server   -> Rust API
apps/web      -> SvelteKit web
apps/desktop  -> Tauri desktop
apps/mobile   -> Flutter mobile
```

## 3. Environment variables

```env
APP_ENV=development
APP_PUBLIC_URL=http://localhost:3000
API_PUBLIC_URL=http://localhost:8080

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
```

## 4. Backend commands

```bash
cd apps/server
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run
```

## 5. Web commands

```bash
cd apps/web
pnpm install
pnpm dev
pnpm build
```

## 6. Desktop commands

```bash
cd apps/desktop
pnpm install
pnpm tauri dev
pnpm tauri build
```

Exact command names depend on the generated Tauri project.

## 7. Mobile commands

```bash
cd apps/mobile
flutter pub get
flutter run
flutter test
flutter build apk
```

iOS builds require macOS/Xcode.

## 8. Health endpoints

```http
GET /healthz
GET /readyz
```

## 9. Deployment

Server deployment is separate from client distribution.

Deploy:

- Rust server;
- PostgreSQL;
- Redis;
- reverse proxy;
- storage provider;
- LiveKit optional.

Distribute:

- web through browser URL;
- desktop as installer;
- mobile as APK/TestFlight/App Store later.

## 10. Important operational decision

Do not require the desktop/mobile app to run the server locally.

The normal model is:

```text
Native client -> self-hosted Rust.chat server
```

A future “local server bundle” can be explored later.

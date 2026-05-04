# Agent Execution Plan

## 1. Corrected mission

Build Rust.chat as a **multi-client self-hosted chat platform**, not a web-only app.

The Rust server is the core product. Web, desktop, and mobile are clients.

## 2. Implementation order

### Phase 0: Contracts and layout

Create:

```text
apps/server
apps/web
apps/desktop
apps/mobile
packages/api-contracts
infra
context
```

Desktop/mobile can be placeholders initially.

### Phase 1: Rust server

Build:

- config;
- health;
- DB;
- Redis;
- OpenAPI;
- auth foundation;
- permission foundation.

### Phase 2: Web reference client

Build SvelteKit web UI first because it is fastest for admin and product iteration.

### Phase 3: Stable contracts

Stabilize:

- REST DTOs;
- error codes;
- WebSocket events;
- auth flows for both cookie and bearer token.

### Phase 4: Desktop

Build Tauri wrapper around Svelte UI or shared web UI.

### Phase 5: Mobile

Build Flutter app after API contracts are stable.

## 3. Do not do

- Do not rewrite server because of mobile.
- Do not make SvelteKit the backend authority.
- Do not duplicate permission rules in clients.
- Do not store secrets in mobile/desktop.
- Do not build all clients fully at once.

## 4. MVP practical recommendation

For the first working MVP:

1. Rust server.
2. SvelteKit web/admin client.
3. Keep API contract native-ready.
4. Add Tauri desktop next.
5. Add Flutter mobile after server flow is proven.

This avoids overloading the first build while preserving the correct architecture.

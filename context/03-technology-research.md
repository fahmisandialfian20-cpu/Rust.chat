# Technology Research

## 1. Backend

### Axum

Use Axum for REST and WebSocket. Axum provides a WebSocket extractor and `WebSocketUpgrade`, making it suitable for realtime chat endpoints.

### Tokio

Use Tokio as the async runtime for the Rust server. The server needs async I/O, task scheduling, timers, Redis, PostgreSQL, WebSocket handling, and background jobs.

### SQLx

Use SQLx for PostgreSQL queries and migrations. The domain model is relational and permission-heavy, so explicit SQL is preferable.

### Redis

Use Redis for ephemeral state:

- presence;
- typing indicators;
- rate limit counters;
- pub/sub;
- temporary access state.

## 2. Web client

Use SvelteKit for the browser client and admin UI.

Why:

- routing;
- layouts;
- TypeScript;
- fast iteration;
- reusable UI with Tauri desktop.

## 3. Desktop client

Use Tauri for desktop packaging.

Why:

- uses web technologies for UI;
- compatible with Svelte;
- Rust-based shell;
- smaller than bundling a full Chromium runtime;
- good fit for Windows/macOS/Linux.

Important correction:

Tauri desktop is native-packaged, but it still uses a WebView for UI. That is acceptable for Rust.chat unless the requirement is “no WebView at all.” If the requirement becomes “fully native widgets only,” consider Flutter desktop, Slint, or platform-native UI later.

## 4. Mobile client

Use Flutter for Android/iOS.

Why:

- mature mobile framework;
- single codebase for Android/iOS;
- can also target desktop/web if needed;
- LiveKit provides a Flutter SDK for audio/video use cases.

Mobile should not be built before backend contracts stabilize.

## 5. Voice/video

Use LiveKit.

Why:

- group voice/video requires SFU behavior;
- client SDKs exist for multiple platforms;
- server can generate room tokens after permission checks;
- avoids building custom WebRTC infrastructure.

## 6. Storage

Use a `FileStorage` abstraction.

Recommended providers:

- LocalStorage for development;
- object storage for production;
- optional PikPak adapter as experiment.

PikPakAPI should remain optional because the referenced implementation is Python-based and unofficial.

## 7. Recommended dependencies

### Rust backend

- `axum`;
- `tokio`;
- `tower-http`;
- `sqlx`;
- `redis`;
- `serde`;
- `argon2`;
- `jsonwebtoken`;
- `tower-sessions`;
- `utoipa`;
- `tracing`;
- `object_store`;
- `livekit-api`.

### Web/Desktop UI

- SvelteKit;
- TypeScript;
- Tailwind;
- shadcn-svelte style components;
- Bits UI;
- Zod;
- openapi-typescript;
- LiveKit JS SDK if media is added.

### Mobile

- Flutter;
- Dart;
- `http` or generated API client;
- WebSocket channel package;
- secure storage package;
- LiveKit Flutter SDK;
- local cache package later.

## 8. Contract-first requirement

Because there will be multiple clients, REST and WebSocket contracts are critical.

Agents must:

- define DTOs clearly;
- generate OpenAPI;
- version WebSocket event types;
- avoid frontend-only assumptions;
- write compatibility tests for API contracts.

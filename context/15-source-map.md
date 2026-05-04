# Source Map

## Axum

- Axum WebSocket module: https://docs.rs/axum/latest/axum/extract/ws/index.html
- Axum WebSocketUpgrade: https://docs.rs/axum/latest/axum/extract/ws/struct.WebSocketUpgrade.html

Conclusion: Axum is suitable for REST and WebSocket chat endpoints.

## Tokio

- Tokio runtime documentation: https://docs.rs/tokio/latest/tokio/runtime/index.html

Conclusion: Tokio is the correct async runtime for the Rust server.

## SQLx

- SQLx migrate macro: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html

Conclusion: SQLx is suitable for migrations and explicit PostgreSQL queries.

## Redis

- Redis async module: https://docs.rs/redis/latest/redis/aio/

Conclusion: Redis is suitable for async ephemeral state, pub/sub, presence, typing, and rate limits.

## SvelteKit

- Svelte package ecosystem: https://svelte.dev/packages
- SvelteKit adapter-node package: https://www.npmjs.com/package/%40sveltejs/adapter-node

Conclusion: SvelteKit remains useful for web/admin and desktop UI reuse.

## Tauri

- Tauri official start page: https://tauri.app/start/
- Tauri GitHub organization: https://github.com/tauri-apps

Conclusion: Tauri is appropriate for native desktop packaging with a web frontend and Rust-based shell.

## Flutter

- Flutter official site: https://flutter.dev/
- Flutter documentation: https://docs.flutter.dev/
- Flutter API docs: https://api.flutter.dev/index.html

Conclusion: Flutter is appropriate for Android/iOS mobile clients and can also target desktop/web if needed.

## LiveKit

- LiveKit SDK platforms: https://docs.livekit.io/transport/sdk-platforms
- LiveKit Flutter quickstart: https://docs.livekit.io/transport/sdk-platforms/flutter/
- LiveKit Flutter SDK reference: https://docs.livekit.io/reference/client-sdk-flutter/

Conclusion: LiveKit has SDK support across web/native/mobile platforms and is a better choice than building custom group voice/video.

## Object storage

- Rust object_store crate: https://docs.rs/object_store/latest/object_store/

Conclusion: File storage should use a provider abstraction.

## PikPak

- PikPakAPI GitHub: https://github.com/Quan666/PikPakAPI
- PikPakAPI PyPI: https://pypi.org/project/PikPakAPI/

Conclusion: PikPak should remain optional and isolated behind an adapter service.

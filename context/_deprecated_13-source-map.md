# Source Map

This file lists the external references used to shape architecture and dependency recommendations.

## Backend

- Axum WebSocket module: https://docs.rs/axum/latest/axum/extract/ws/index.html
- Axum WebSocketUpgrade: https://docs.rs/axum/latest/axum/extract/ws/struct.WebSocketUpgrade.html
- Tokio runtime: https://docs.rs/tokio/latest/tokio/runtime/index.html
- SQLx migrations: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
- Redis async module: https://docs.rs/redis/latest/redis/aio/

## Frontend

- Svelte package ecosystem: https://svelte.dev/packages
- SvelteKit adapter-node package: https://www.npmjs.com/package/%40sveltejs/adapter-node
- shadcn-svelte documentation: https://tw3.shadcn-svelte.com/docs

## Storage and media

- Rust object_store crate: https://docs.rs/object_store/latest/object_store/
- LiveKit tokens and grants: https://docs.livekit.io/frontends/authentication/tokens
- LiveKit server token documentation: https://docs.livekit.io/home/server/generating-tokens/

## PikPak

- PikPakAPI PyPI: https://pypi.org/project/PikPakAPI/
- PikPakAPI GitHub: https://github.com/Quan666/PikPakAPI

## Research conclusions

- Axum is suitable for REST and WebSocket because it provides WebSocket extractors behind the `ws` feature.
- Tokio is the correct async runtime for the Rust server because the app needs non-blocking I/O, task scheduling, timers, and async network/database work.
- SQLx is appropriate because this app needs explicit, auditable relational queries and migrations.
- Redis is appropriate for ephemeral state such as presence, typing indicators, pub/sub, and rate limits.
- SvelteKit is preferred over plain TS/CSS/JS because the app needs structured routing, layouts, and a maintainable frontend architecture.
- shadcn-svelte should be treated as copy-owned component source, not a normal component dependency.
- LiveKit should handle group voice/video because building a custom SFU/WebRTC stack is outside MVP scope.
- PikPakAPI should remain optional and isolated because it is an unofficial Python implementation and PyPI lists it under GPLv3.

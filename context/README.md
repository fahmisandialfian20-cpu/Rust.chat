# Rust.chat Multi-Client Context Pack

This folder contains the agent context for building **Rust.chat**, a serious self-hosted chat platform with **web, desktop, and mobile clients**.

This is not a web-only application.

## Product identity

Rust.chat is a self-hosted Discord/Telegram-like communication platform.

The person who deploys the application is the **Hoster**, the highest authority for that deployment. The app starts empty. The Hoster creates spaces, channels, roles, feature toggles, invite links, and moderation rules.

## Multi-client direction

The Rust backend is the product core. Clients are replaceable surfaces.

Target clients:

- **Web client**: SvelteKit, useful for browser access and admin panels.
- **Desktop client**: Tauri + Svelte UI, useful for Windows/macOS/Linux native packaging while reusing the web UI.
- **Mobile client**: Flutter, useful for Android/iOS native apps and future desktop/web expansion if needed.

All clients must use the same backend contracts:

- REST API for CRUD and auth;
- WebSocket for realtime chat, typing, and presence;
- short-lived media tokens for LiveKit voice/video;
- signed/proxied file URLs for attachments;
- server-authoritative permission checks.

## Core rule

Never design the backend as if it belongs to one UI.

The backend must work for:

```text
SvelteKit Web
Tauri Desktop
Flutter Mobile
Future CLI/Bot Client
```

## Read order

1. `01-product-requirements.md`
2. `02-multiclient-architecture.md`
3. `03-technology-research.md`
4. `04-domain-model-database.md`
5. `05-permissions-rbac.md`
6. `06-api-websocket-contract.md`
7. `07-module-responsibilities.md`
8. `08-storage-and-media.md`
9. `09-client-strategy.md`
10. `10-frontend-web-desktop-guide.md`
11. `11-mobile-flutter-guide.md`
12. `12-operations-runbook.md`
13. `13-security-observability.md`
14. `14-agent-execution-plan.md`
15. `15-source-map.md`

## MVP definition

The MVP is complete when:

1. The Hoster can bootstrap the first account.
2. The Hoster can create spaces, public channels, and private channels.
3. Members can register through invite links.
4. Members can only see allowed spaces/channels.
5. Realtime text chat works through WebSocket.
6. Permissions and feature flags are enforced by the server.
7. At least one primary client works end-to-end.
8. The architecture is ready for desktop and mobile clients without rewriting backend contracts.

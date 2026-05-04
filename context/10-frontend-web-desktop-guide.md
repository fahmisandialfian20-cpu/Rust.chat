# Web and Desktop Guide

## 1. Web client

Use SvelteKit for:

- browser client;
- admin interface;
- reference implementation;
- permission UI;
- channel management.

## 2. Desktop client

Use Tauri for:

- Windows/macOS/Linux packaging;
- system tray;
- desktop notifications;
- local app settings;
- filesystem picker;
- auto-update later.

## 3. Shared UI strategy

Recommended layout:

```text
apps/
  web/
  desktop/

packages/
  web-ui/
  api-client-ts/
```

The desktop app can either:

1. embed the same Svelte app; or
2. use a shared Svelte component package.

## 4. Desktop API behavior

Desktop should call the same server URL as web.

Examples:

- self-hosted server URL configured during first launch;
- local LAN URL;
- public domain URL through reverse proxy.

Do not bundle the server inside the desktop app for MVP.

## 5. Tauri-specific responsibilities

Tauri side should handle only platform-specific features:

- notifications;
- tray;
- file picker;
- auto-start later;
- secure local token storage later;
- deep links later.

Business logic stays in the Rust server.

## 6. Web/Desktop auth

Initial options:

- cookie session for web;
- bearer token for desktop;
- or both.

Recommended:

- support both from the backend early;
- Tauri can store token more like a native app;
- browser can use secure cookie.

## 7. UI components

Use Svelte components for:

- chat shell;
- channel list;
- message list;
- composer;
- permissions editor;
- role editor;
- settings panels.

## 8. WebSocket behavior

Both web and desktop use the same WebSocket event schema.

Desktop may add native notifications when `message.created` arrives and the app is unfocused.

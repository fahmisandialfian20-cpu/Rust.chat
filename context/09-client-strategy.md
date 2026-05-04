# Client Strategy

## 1. Corrected product direction

Rust.chat is not only a web app.

The correct direction is:

```text
Rust Core Server
  ├─ SvelteKit Web Client
  ├─ Tauri Desktop Client
  └─ Flutter Mobile Client
```

## 2. Why still keep SvelteKit

SvelteKit is still valuable because:

- it gives the fastest path to admin UI;
- it can be reused in Tauri desktop;
- it helps test backend flows quickly;
- it can serve as the reference client.

The mistake would be making the server depend on SvelteKit-specific assumptions.

## 3. Desktop recommendation

Use Tauri for desktop.

Pros:

- native app packaging;
- uses Rust;
- can reuse Svelte UI;
- smaller than Electron-style bundles;
- supports desktop integration through plugins.

Cons:

- UI is WebView-based, not native widgets;
- complex media permissions may need platform testing.

Use Tauri unless the requirement becomes “no WebView at all.”

## 4. Mobile recommendation

Use Flutter for mobile.

Pros:

- one codebase for Android and iOS;
- strong mobile UI ecosystem;
- native performance;
- LiveKit Flutter SDK support;
- can share concepts from API contracts.

Cons:

- cannot directly reuse Svelte components;
- needs separate UI implementation;
- push notifications and background behavior require platform-specific work.

## 5. Alternative: Flutter for all clients

Possible but not recommended for first step.

Flutter can target mobile, desktop, and web, but Rust.chat already benefits from SvelteKit/Tauri for fast admin and desktop UI iteration.

Recommended split:

- Web/admin/desktop UI: Svelte + Tauri.
- Mobile: Flutter.

## 6. Client build order

### Step 1: Web reference client

Build all server logic and admin UI.

### Step 2: Desktop shell

Wrap the web UI with Tauri, add desktop notifications and tray later.

### Step 3: Mobile app

Build Flutter app using stable API/WebSocket contracts.

## 7. Shared contracts

Store contracts in:

```text
packages/api-contracts/
  openapi.json
  websocket-events.schema.json
  errors.md
```

Use generated clients:

- TypeScript for web/desktop;
- Dart for mobile.

## 8. Client feature matrix

| Feature | Web | Desktop | Mobile |
|---|---:|---:|---:|
| Login/register | yes | yes | yes |
| Lobby | yes | yes | yes |
| Text chat | yes | yes | yes |
| Admin role editor | yes | optional | later |
| File upload | yes | yes | yes |
| Voice | yes | yes | later |
| Video | yes | yes | later |
| Push notifications | browser limited | desktop notification | yes |
| Offline cache | later | later | later |

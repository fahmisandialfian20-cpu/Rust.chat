# Module Responsibilities

## 1. Server modules

```text
src/
  config/
  domain/
  routes/
  handlers/
  services/
  repositories/
  auth/
  permissions/
  realtime/
  storage/
  media/
  audit/
  devices/
  notifications/
  telemetry/
```

## 2. New modules for multi-client support

### `devices`

Responsibilities:

- register client device;
- update last seen;
- store push token;
- revoke device/session;
- list active devices.

### `notifications`

Responsibilities:

- prepare notification payloads;
- avoid leaking private content;
- route to device-specific push providers later;
- store notification preferences.

## 3. `auth`

Must support:

- browser session;
- native bearer token;
- refresh token;
- device registration.

## 4. `realtime`

Must support:

- browser WebSocket;
- desktop WebSocket;
- mobile WebSocket;
- reconnect;
- event versioning;
- stable error codes.

## 5. `media`

Must support multiple client SDKs:

- JS SDK for web/desktop;
- Flutter SDK for mobile.

Server only generates LiveKit tokens. It does not implement client-specific media logic.

## 6. `storage`

Must return URLs/metadata that are usable by all clients.

Do not assume browser-only behavior such as opening a new tab.

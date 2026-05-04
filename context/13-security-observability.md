# Security and Observability

## 1. Multi-client security principles

- All clients are untrusted.
- Server is the source of truth.
- Permissions are server-side.
- Client tokens must be revocable.
- Native clients need secure token storage.
- Push notifications must respect privacy.

## 2. Auth

Support:

- session cookie for browser;
- bearer tokens for native clients;
- refresh tokens;
- device revocation.

## 3. Device security

Track:

- client type;
- platform;
- device name;
- last seen;
- push token;
- revocation.

Allow users to log out devices.

## 4. WebSocket security

- authenticate connection;
- validate event version;
- limit frame size;
- heartbeat;
- rate limit;
- bounded queue.

## 5. File security

Same rules for all clients:

- MIME sniff;
- size limit;
- server-mediated upload;
- permission check before download;
- no permanent public private-file URLs.

## 6. Notification privacy

Default notification behavior for private channels should be conservative.

Example:

```text
New message in a private channel
```

instead of full message content, unless user explicitly allows previews.

## 7. Observability

Include `client_type`, `platform`, and `app_version` in logs when available.

Example fields:

- request_id;
- user_id;
- client_type;
- platform;
- route/event;
- error_code;
- latency_ms.

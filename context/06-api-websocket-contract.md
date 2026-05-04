# API and WebSocket Contract

This contract must support web, desktop, mobile, and future bot clients.

## 1. API base

```text
/api/v1
```

## 2. Authentication modes

Support at least two auth modes:

### Browser/Tauri session mode

- secure cookie;
- CSRF protection for browser-like flows.

### Native bearer token mode

- access token;
- refresh token;
- secure storage on device;
- revocation support;
- device registration.

Do not design authentication only for browser cookies.

## 3. Auth endpoints

```http
POST /api/v1/auth/bootstrap-owner
POST /api/v1/auth/register
POST /api/v1/auth/login
POST /api/v1/auth/logout
POST /api/v1/auth/refresh
GET  /api/v1/auth/me
GET  /api/v1/auth/devices
DELETE /api/v1/auth/devices/{device_id}
```

Login request should include client metadata:

```json
{
  "username_or_email": "hoster",
  "password": "secret",
  "client": {
    "client_type": "mobile",
    "platform": "android",
    "device_name": "Pixel"
  }
}
```

## 4. REST resources

Same resources for all clients:

```http
/api/v1/spaces
/api/v1/channels
/api/v1/messages
/api/v1/invites
/api/v1/roles
/api/v1/files
/api/v1/profile
/api/v1/devices
```

## 5. WebSocket endpoint

```http
GET /api/v1/ws
```

Supported auth:

- session cookie;
- bearer token;
- short-lived WebSocket token.

## 6. WebSocket event envelope

```json
{
  "version": 1,
  "type": "message.send",
  "request_id": "client-generated-id",
  "payload": {},
  "sent_at": "2026-05-04T00:00:00Z"
}
```

Use `version` because mobile clients may lag behind web deployments.

## 7. Client metadata in hello

```json
{
  "version": 1,
  "type": "hello",
  "request_id": "abc",
  "payload": {
    "client_type": "mobile",
    "platform": "android",
    "app_version": "0.1.0",
    "supports": ["message.v1", "presence.v1"]
  }
}
```

## 8. Server events

- `hello.ok`
- `message.created`
- `message.updated`
- `message.deleted`
- `typing.updated`
- `presence.updated`
- `channel.created`
- `channel.updated`
- `permission.updated`
- `member.joined`
- `member.left`
- `notification.created`
- `media.room.updated`

## 9. Error shape

```json
{
  "error": {
    "code": "permission_denied",
    "message": "You cannot perform this action",
    "details": {}
  }
}
```

Error codes must be stable across clients.

## 10. OpenAPI

Generate OpenAPI for REST.

Clients should generate or validate API types from OpenAPI:

- TypeScript client for web/desktop;
- Dart client for Flutter;
- future SDKs for bots.

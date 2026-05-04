# Storage and Media

## 1. Storage

Storage remains client-agnostic.

All clients upload through the server in MVP:

```text
Web/Desktop/Mobile -> Rust API -> FileStorage Provider
```

## 2. Why not direct-to-storage first

Direct-to-storage uploads are useful later, but MVP should use server-mediated upload because:

- permissions are complex;
- channel feature flags must be checked;
- mobile/desktop/web should behave the same;
- easier audit logging;
- easier malware/MIME checks.

## 3. Providers

- LocalStorage for development.
- Object storage for production.
- Optional PikPak adapter.

## 4. Mobile upload considerations

Mobile uploads require:

- camera/photo permission;
- background upload later;
- retry/resume later;
- compression later;
- offline queue later.

Do not add these before MVP server contracts are stable.

## 5. Media

Use LiveKit for voice/video.

Clients:

- Web: LiveKit JS SDK.
- Desktop/Tauri: LiveKit JS SDK inside webview.
- Mobile: LiveKit Flutter SDK.

Server:

- checks permission;
- checks channel feature flags;
- generates token;
- returns room name, URL, token.

## 6. Media token endpoint

```http
POST /api/v1/channels/{channel_id}/media-token
```

Request:

```json
{
  "mode": "voice",
  "intent": "join",
  "client_type": "mobile"
}
```

Response:

```json
{
  "provider": "livekit",
  "url": "wss://livekit.example.com",
  "room": "space-{space_id}-channel-{channel_id}",
  "token": "jwt"
}
```

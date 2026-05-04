# Mobile Flutter Guide

## 1. Purpose

The mobile app is a native Android/iOS client for Rust.chat.

It must use the same backend contracts as web and desktop.

## 2. Recommended stack

- Flutter;
- Dart;
- generated Dart API client from OpenAPI if possible;
- WebSocket client;
- secure token storage;
- LiveKit Flutter SDK for future voice/video;
- local cache later.

## 3. Mobile app structure

```text
apps/mobile/
  lib/
    main.dart
    app.dart
    core/
      api/
      auth/
      realtime/
      storage/
      errors/
    features/
      auth/
      lobby/
      spaces/
      channels/
      chat/
      profile/
      settings/
      media/
```

## 4. Mobile auth

Mobile should use:

- access token;
- refresh token;
- secure storage;
- device registration;
- revoke device endpoint.

Do not rely on browser cookie-only auth.

## 5. Mobile realtime

Use WebSocket:

- reconnect on network change;
- exponential backoff;
- resubscribe active channel;
- avoid duplicate messages using request id;
- show offline state.

## 6. Mobile chat UX

MVP mobile screens:

1. Server URL setup.
2. Login/register through invite.
3. Lobby.
4. Space list.
5. Channel list.
6. Chat screen.
7. Profile settings.

Admin UI can be delayed on mobile.

## 7. Mobile files

MVP:

- pick image/file;
- upload via server;
- show progress;
- handle file size errors.

Later:

- compression;
- background upload;
- download cache;
- offline draft queue.

## 8. Mobile notifications

Do not implement full push notification first unless needed.

Prepare backend with `client_devices` and `push_token`, but implement push later.

Important privacy rule:

Push notifications for private channels should not leak message content unless user settings allow it.

## 9. Mobile voice/video

Use LiveKit Flutter SDK later.

Flow:

1. mobile requests media token from Rust server;
2. server checks permission and channel feature flag;
3. server returns LiveKit token;
4. Flutter client connects to LiveKit.

## 10. What not to do

- Do not duplicate backend permission logic in Flutter.
- Do not directly connect mobile app to PostgreSQL/Redis.
- Do not put storage provider secrets in mobile app.
- Do not expose LiveKit API secret.
- Do not build mobile before API contracts are stable unless prototyping.

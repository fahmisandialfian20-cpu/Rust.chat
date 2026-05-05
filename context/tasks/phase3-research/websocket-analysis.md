# WebSocket / Realtime Module Security Analysis

**Date:** 2026-05-05  
**Analyst:** Rust Backend Security Researcher  
**Files analyzed:**
- `apps/server/src/realtime/mod.rs`
- `apps/server/src/realtime/events.rs`
- `apps/server/src/realtime/hub.rs`
- `apps/server/src/realtime/gateway.rs`
- `apps/server/src/permissions/service.rs`
- `apps/server/src/handlers/messages.rs`
- `apps/server/src/services/message_service.rs`
- `apps/server/tests/permissions_test.rs`

---

## File Summaries

### 1. `realtime/mod.rs`
Exports the three realtime submodules (`events`, `gateway`, `hub`) and re-exports `ws_upgrade` and `RealtimeHub` at the crate root level. It is a thin module facade with no logic.

**Connection to other parts:** Serves as the public API surface for the rest of the application (e.g., `main.rs` imports `RealtimeHub` and `ws_upgrade` from here).

### 2. `realtime/events.rs`
Defines the `WsEvent` enum (tagged serialization) and its associated data structs: `HelloData`, `MessageCreatedData`, `MessageEditedData`, `MessageDeletedData`, `TypingData`, `PresenceData`, `ErrorData`. Provides a `to_json()` helper.

**Connection to other parts:** This is the wire protocol for WebSocket. Both gateway and hub operate on these types. The `Message` domain struct is reused directly from `crate::domain::message`.

### 3. `realtime/hub.rs`
Implements a simple `RealtimeHub` backed by a `tokio::sync::broadcast` channel. `publish()` sends the JSON-serialized event to **all** subscribers. `publish_to_channel()` is currently a no-op wrapper that calls `publish()` without any channel filtering.

**Connection to other parts:** The hub is stored in `AppState` and cloned into every WebSocket connection task. It is also available to services/handlers that wish to push realtime updates.

### 4. `realtime/gateway.rs`
Handles the HTTP upgrade to WebSocket (`ws_upgrade`) and the socket lifecycle (`handle_socket`). It authenticates the user via the `AuthUser` extractor (JWT + Redis session validation), sends a `Hello` event, subscribes to the broadcast hub, and then runs two tasks:
- **Send task:** Forwards hub broadcast messages to the client.
- **Receive task:** Reads incoming text frames, deserializes them as `WsEvent`, and immediately calls `hub.publish(event)`.

**Connection to other parts:** Registered in `main.rs` at `GET /api/v1/ws`. Uses `AppState` (for hub and presence service) and `AuthUser` (for auth).

---

## Specific Questions

### 1. Does the WebSocket gateway check permissions before allowing message send?

**No.**

The `receive_task` in `gateway.rs` performs zero permission checks:

```rust
Ok(Message::Text(text)) => {
    if let Ok(event) = serde_json::from_str::<WsEvent>(&text) {
        hub.publish(event);
    }
}
```

There is no call to `PermissionService`, no check for `SendMessages`, `ViewChannel`, space membership, or even whether the channel exists. The gateway only verifies that the client is authenticated (via `AuthUser`).

### 2. How does a message sent via WebSocket get validated? Is there a permission check for SendMessages, ViewChannel, etc.?

**It is not validated at all.**

The WebSocket receive path:
1. Deserializes the raw JSON into a `WsEvent`.
2. Calls `hub.publish(event)`.

It does **not**:
- Call `MessageService::create_message()` or any service method.
- Verify that the `channel_id` in the event exists or is accessible to the user.
- Check `SendMessages`, `ViewChannel`, `ReadMessages`, or any permission key.
- Validate rate limits.
- Persist the message to PostgreSQL.

An authenticated client can craft a `MessageCreated` event with arbitrary `message` content (including fake `author_user_id`, `channel_id`, or timestamp) and it will be broadcast to **all** connected WebSocket clients.

### 3. What events can flow through WebSocket?

Per `events.rs`:

| Event | Direction | Notes |
|-------|-----------|-------|
| `Hello` | Server → Client | Sent once on connection. Contains `user_id` and `session_id`. |
| `message.created` | Bidirectional (intended) | Currently blindly echoed by server. No persistence. |
| `message.edited` | Bidirectional (intended) | Currently blindly echoed by server. No ownership check. |
| `message.deleted` | Bidirectional (intended) | Currently blindly echoed by server. No ownership check. |
| `typing.updated` | Client → Server (intended) | Currently blindly echoed to all clients. |
| `presence.updated` | Server → Client (intended) | Sent by presence service, but client can spoof it too. |
| `error` | Server → Client | Not currently used in gateway logic. |

### 4. Is there ANY difference between REST permission checks and WebSocket permission checks?

**Yes — WebSocket checks are strictly weaker and behave like a pure echo server.**

REST message handlers (`handlers/messages.rs`):
- `create_message`: Checks **rate limit** only. Does **not** check `SendMessages`, `ViewChannel`, or space membership. Calls `MessageService::create_message()` which persists to DB.
- `update_message`: Calls `MessageService::update_message()` which enforces **ownership** (can only edit own message) but does **not** check `EditOwnMessage` or `EditAnyMessage` permissions.
- `delete_message`: Calls `MessageService::delete_message()` which enforces **ownership** but does **not** check `DeleteOwnMessage` or `DeleteAnyMessage` permissions.
- `list_messages` / `get_message`: No permission checks at all.

WebSocket (`gateway.rs`):
- No rate limiting.
- No persistence.
- No ownership checks.
- No permission checks.
- Client can forge any event data and it is broadcast verbatim to all connected clients.

**Summary:** REST at least goes through the service layer and persists data (with basic ownership checks for edit/delete). WebSocket bypasses the service layer entirely and acts as an unfiltered broadcast proxy.

### 5. What is the testing strategy for WebSocket? Are there existing tests?

There are **no automated WebSocket tests**.

In `tests/permissions_test.rs`, line 545:

```rust
#[tokio::test]
async fn websocket_respects_permission() {
    // This test requires a running WebSocket server and connection.
    // It is documented as a known gap for automated integration tests.
    // Manual verification: connect WS as member without SendMessages,
    // attempt to send message event, verify rejection.
}
```

This is an **empty placeholder** documenting the gap. No other test files reference WebSocket, realtime, or gateway logic.

---

## Critical Security Findings

1. **Arbitrary Event Forgery:** Any authenticated user can send a `message.created` event with a fake `author_user_id` and have it broadcast to every connected client.
2. **No Persistence / Ghost Messages:** Messages sent via WebSocket never touch the database. Clients may see messages that do not exist in message history (REST `list_messages`).
3. **No Permission Enforcement:** The WebSocket path completely ignores the `PermissionService`, violating rule #8 from `03-domain-permissions.md`: *"WebSocket actions must use the same rules as REST actions."*
4. **No Rate Limiting:** WebSocket message sending bypasses the `RateLimiter` entirely.
5. **Global Broadcast Leakage:** `hub.publish()` broadcasts to **all** subscribers regardless of channel or space. A message intended for a private channel is sent to every connected user.
6. **No Edit/Delete Authorization:** A user can send `message.edited` or `message.deleted` for messages they do not own.

---

## Recommendations

1. **Do not trust inbound WebSocket events.** The receive path should parse a thin **client command** enum (e.g., `SendMessage { channel_id, content }`) rather than accepting full `WsEvent` objects.
2. **Route all mutating WS actions through services.** `SendMessage` should call `MessageService::create_message()` (or a new `RealtimeService`) after permission checks.
3. **Apply `PermissionService` checks on WS actions.** At minimum: `SendMessages`, `ViewChannel`, space membership, and channel feature flags.
4. **Implement channel-scoped broadcast.** `publish_to_channel` must actually filter subscribers by channel membership/visibility.
5. **Add WebSocket integration tests.** Use `tokio_tungstenite` or `axum`'s test utilities to connect a WS client and assert rejection of unauthorized events.
6. **Unify REST and WS validation logic.** Extract permission validation into a shared helper so both transports enforce identical rules.

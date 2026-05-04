# Security and Observability

## 1. Security principles

- Deny by default.
- Server is the source of truth.
- Do not leak private channel metadata.
- Do not expose secrets to the browser.
- Do not trust browser-provided MIME.
- Do not allow arbitrary user CSS in the MVP.
- Do not store raw invite tokens.
- Do not store raw passwords.
- Do not expose Redis/PostgreSQL publicly.

## 2. Auth security

Use:

- Argon2id for password hashing;
- password pepper from environment;
- secure session cookies;
- session rotation after login;
- rate limit login/register;
- optional email verification later.

Cookie settings in production:

- `HttpOnly`;
- `Secure`;
- `SameSite=Lax` or stricter;
- reasonable expiration.

## 3. Invite security

Rules:

- store token hash only;
- show raw token once;
- support expiration;
- support max usage;
- support revoke;
- accept invite in DB transaction;
- audit invite creation and revocation.

## 4. Permission security

Every sensitive action must check permission.

Examples:

- create channel;
- update channel features;
- update role permissions;
- send message;
- upload file;
- download attachment;
- delete message;
- generate media token;
- view audit logs.

## 5. WebSocket security

Implement:

- auth before event processing;
- max frame/message size;
- heartbeat;
- bounded outbound queue;
- idle timeout;
- per-user/channel rate limits;
- schema validation;
- graceful disconnect;
- resubscribe permission validation after reconnect.

## 6. File security

Implement:

- upload size limit;
- MIME sniffing;
- extension allowlist;
- random object keys;
- no direct permanent public links for private files;
- disallow SVG in MVP;
- `Content-Disposition: attachment` for risky file types;
- per-channel upload feature flag.

## 7. XSS protection

- render user messages as escaped text by default;
- if Markdown is added, use a safe Markdown renderer;
- do not use raw HTML;
- do not allow arbitrary CSS;
- whitelist theme tokens;
- avoid external avatar URLs for MVP.

## 8. CSRF

If browser auth uses cookies, add CSRF protection for mutating routes or ensure an equivalent strategy is used.

## 9. Rate limits

Initial suggestions:

| Action | Limit |
|---|---:|
| login | 5/min/IP |
| register | 5/min/IP |
| invite preview | 30/min/IP |
| message send | 30/min/user/channel |
| file upload | 10/min/user |
| websocket connect | 20/min/IP |
| role update | 30/min/admin |

## 10. Audit logs

Audit these actions:

- bootstrap owner;
- login failure threshold reached;
- role created/updated/deleted;
- permission changed;
- member role assigned/removed;
- invite created/revoked;
- channel created/updated/deleted;
- channel visibility changed;
- channel feature flags changed;
- message deleted by moderator;
- member kicked/banned/muted;
- storage provider changed;
- media token requested for voice/video.

## 11. Observability

Use `tracing` spans:

- `http_request`;
- `ws_connection`;
- `permission_check`;
- `message_send`;
- `file_upload`;
- `storage_put`;
- `media_token_create`;
- `invite_accept`.

Metrics to add later:

- active WebSocket connections;
- messages per second;
- upload bytes;
- storage failures;
- permission denied count;
- login failures;
- Redis latency;
- DB pool usage.

## 12. Threat model

Main threats:

- unauthorized private channel access;
- permission escalation;
- invite token leakage;
- storage exhaustion;
- XSS through messages/themes;
- WebSocket spam;
- file abuse;
- exposed LiveKit API secret;
- exposed database/cache ports.

The architecture must mitigate these from the first MVP.

# 04 — Client and UI Experience

## Client Direction

Rust.chat should eventually support:

- web client
- native desktop client
- mobile client

Current priority:

```text
web client first
```

Desktop and mobile should come after backend/API contracts are stable.

---

## UI Product Direction

The UI should feel:

- clean
- natural
- friendly
- modern
- not overloaded
- not AI-generated
- easy to understand
- suitable for daily chat use

Avoid long unnecessary text in the product UI.

Use short labels, clear actions, and simple hierarchy.

---

## Main UI Areas

MVP web UI should eventually include:

1. Login/register
2. Lobby / entry screen
3. Space list
4. Channel list
5. Chat panel
6. Member/role admin panels later
7. Channel settings later
8. User profile/preferences later

---

## Empty App Behavior

A fresh deployment starts empty.

Expected first-run flow:

1. Hoster opens the app.
2. Hoster bootstraps the first account.
3. Hoster creates a space.
4. Hoster creates channels.
5. Hoster creates invite links.
6. Members join through invites.

The UI must not assume that a default room already exists.

---

## Login and Register

The app needs:

- login
- register
- invite-aware registration
- authenticated session handling

Registration may require invite depending on instance settings.

---

## Lobby Behavior

The lobby is the authenticated entry area.

The lobby can show:

- spaces the user can access
- pending invites or join actions
- empty state if no access exists

Lobby access does not mean channel access.

---

## Space and Channel UI

Users should only see spaces and channels returned by the backend.

Frontend must not guess hidden/private channels.

Frontend can hide actions for convenience, but backend remains the authority.

---

## Personal Customization

Future user customization may include:

- avatar
- display name
- personal theme/skin
- font preference
- compact/comfortable density
- color preference

This is not part of the first MVP backend stabilization.

Do not prioritize UI customization before permissions and message safety.

---

## Channel Feature UI

Channel settings should eventually allow the Hoster/admin to enable or disable features:

- send messages
- send files
- voice group later
- video group later
- read-only mode later

Backend must enforce these settings.

Frontend only reflects them.

---

## Design Rules

1. Avoid unnecessary long copy.
2. Prefer clear action labels.
3. Avoid fake data that hides backend issues.
4. Show empty states honestly.
5. Do not expose controls the user obviously cannot use, but still rely on backend enforcement.
6. Keep admin settings understandable.
7. Prioritize function before polish during MVP.

---

## Non-Goals for Current Backend Work

Do not work on these unless a task explicitly asks for them:

- advanced theme editor
- custom skins
- animation polish
- full responsive redesign
- native desktop UI
- mobile UI
- voice/video UI
- reaction UI
- thread UI

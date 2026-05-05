# 03 — Domain and Permissions

## Core Domain Objects

The MVP core should focus on these domain objects:

```text
User
Session
Space
SpaceMember
Role
RolePermission
Channel
ChannelAccess / ChannelOverride
Invite
Message
FileObject later
```

Do not add unnecessary future domain objects until the MVP core is stable.

---

## User

A user is an account in the instance.

A user can:

- register or be invited
- log in
- enter the lobby
- join spaces through membership/invites
- receive roles
- use channels only when authorized

---

## Hoster

The Hoster is the first owner/deployer account.

The Hoster has instance-level authority.

Hoster bypass may exist, but it must be explicit and tested.

---

## Space

A space is a server/community/workspace-like container.

The app starts with no spaces.

The Hoster can create many spaces.

A user must be a member of a space, or have an explicit rule, to access space resources.

---

## Channel

A channel belongs to a space.

A channel can be public or private.

Public does not mean internet-public. Public means visible to allowed space members.

Private means extra access is required.

Channels can have feature flags such as:

- send messages enabled
- send files enabled
- voice group enabled later
- video group enabled later
- read-only mode later

Feature flags do not replace permissions.

Correct action rule:

```text
user has permission AND channel feature allows action
```

---

## Role

Roles are permission groups.

The Hoster can create roles and assign them to members.

Roles should be configurable through checklist-style UI.

Roles can represent:

- admin
- moderator
- trusted member
- guest
- private-channel member

---

## Permission Model

The backend must decide permissions.

A practical MVP permission flow:

```text
1. Is the user the Hoster?
2. Is the user a member of the target space?
3. What roles does the user have?
4. What permissions do those roles grant?
5. Are there channel-specific overrides?
6. Do channel feature flags allow the action?
```

---

## Important Permission Keys

MVP permission keys should include:

```text
ManageInstance
ManageSpaces
ManageMembers
ManageRoles
ManageChannels
ManageInvites
ViewSpace
ViewChannel
ReadMessages
SendMessages
EditOwnMessage
DeleteOwnMessage
EditAnyMessage
DeleteAnyMessage
SendFiles
```

Future keys can include:

```text
JoinVoice
StartVoice
JoinVideo
StartVideo
ShareScreen
UseWebhooks
CreateThreads
ManageThreads
AddReactions
```

Future keys must not distract from MVP core.

---

## Message Permission Rules

### List messages

A user can list messages only if:

```text
authenticated
AND member of the space
AND can view the channel
AND has ReadMessages permission
```

### Send message

A user can send messages only if:

```text
authenticated
AND member of the space
AND can view the channel
AND has SendMessages permission
AND channel allows message sending
```

### Edit message

A user can edit a message only if:

```text
authenticated
AND message is in an accessible channel
AND (
  user owns the message AND has EditOwnMessage
  OR user has EditAnyMessage
)
```

### Delete message

A user can delete a message only if:

```text
authenticated
AND message is in an accessible channel
AND (
  user owns the message AND has DeleteOwnMessage
  OR user has DeleteAnyMessage
)
```

---

## Non-Negotiable Security Rules

1. Do not trust frontend permissions.
2. Do not bypass backend permission checks.
3. Do not use `Uuid::nil()` as the real acting user.
4. Do not expose private channels to unauthorized users.
5. Do not let users read hidden channel messages.
6. Do not let users send messages without permission.
7. Do not let users edit/delete other users' messages without proper permission.
8. WebSocket actions must use the same rules as REST actions.

---

## Invite Rules

Invite behavior must be defined before implementation.

MVP questions:

- Who can create invites?
- Does the invite target the lobby, a space, or a channel?
- Does the invite grant membership only, or also a role?
- Does it expire?
- Does it have max uses?
- Is the invite token stored raw or hashed?

Security direction:

- Do not store raw invite tokens if possible.
- Store HMAC/hash and compare securely.
- Accepting an invite should create explicit membership/access records.

---

## Testing Direction

Permission tests must prove:

1. Unauthenticated user is rejected.
2. Member cannot see private channel without access.
3. Member cannot read messages in hidden channel.
4. Member cannot send without `SendMessages`.
5. Member can send with `SendMessages`.
6. User cannot edit another user's message without `EditAnyMessage`.
7. User cannot delete another user's message without `DeleteAnyMessage`.
8. Hoster bypass works and is explicit.
9. Invite accept creates correct membership.
10. WebSocket send respects permission rules.

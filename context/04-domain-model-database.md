# Domain Model and Database

The domain model remains server-owned and client-agnostic.

## 1. Core entities

- `users`
- `user_profiles`
- `instance_settings`
- `spaces`
- `space_memberships`
- `roles`
- `role_permissions`
- `member_roles`
- `channels`
- `channel_feature_flags`
- `channel_permission_overrides`
- `invites`
- `messages`
- `file_objects`
- `message_attachments`
- `audit_logs`
- `user_theme_preferences`
- `client_devices`

## 2. New table: client_devices

Because native desktop and mobile clients exist, the backend should track devices.

```sql
CREATE TABLE client_devices (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  client_type text NOT NULL CHECK (client_type IN ('web', 'desktop', 'mobile', 'bot')),
  platform text,
  device_name text,
  push_token text,
  last_seen_at timestamptz,
  created_at timestamptz NOT NULL DEFAULT now()
);
```

Uses:

- push notifications;
- session/device management;
- security review;
- logout device;
- mobile notification routing.

## 3. Core SQL sketch

### users

```sql
CREATE TABLE users (
  id uuid PRIMARY KEY,
  username citext UNIQUE NOT NULL,
  email citext UNIQUE,
  password_hash text NOT NULL,
  status text NOT NULL DEFAULT 'active',
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);
```

### instance_settings

```sql
CREATE TABLE instance_settings (
  id smallint PRIMARY KEY DEFAULT 1 CHECK (id = 1),
  owner_user_id uuid NOT NULL REFERENCES users(id),
  instance_name text NOT NULL,
  registration_mode text NOT NULL DEFAULT 'invite_only',
  storage_provider text NOT NULL DEFAULT 'local',
  max_upload_bytes bigint NOT NULL DEFAULT 26214400,
  settings jsonb NOT NULL DEFAULT '{}',
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);
```

### spaces

```sql
CREATE TABLE spaces (
  id uuid PRIMARY KEY,
  name text NOT NULL,
  slug text UNIQUE NOT NULL,
  description text,
  icon_object_id uuid NULL,
  created_by uuid NOT NULL REFERENCES users(id),
  visibility text NOT NULL DEFAULT 'private',
  settings jsonb NOT NULL DEFAULT '{}',
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now()
);
```

### channels

```sql
CREATE TABLE channels (
  id uuid PRIMARY KEY,
  space_id uuid NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
  parent_id uuid NULL REFERENCES channels(id) ON DELETE SET NULL,
  name text NOT NULL,
  slug text NOT NULL,
  kind text NOT NULL DEFAULT 'text',
  visibility text NOT NULL DEFAULT 'public',
  position int NOT NULL DEFAULT 0,
  topic text,
  created_by uuid NOT NULL REFERENCES users(id),
  archived_at timestamptz NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz NOT NULL DEFAULT now(),
  UNIQUE(space_id, slug)
);
```

### messages

```sql
CREATE TABLE messages (
  id uuid PRIMARY KEY,
  channel_id uuid NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
  author_user_id uuid NOT NULL REFERENCES users(id),
  content text NOT NULL,
  kind text NOT NULL DEFAULT 'text',
  reply_to_message_id uuid NULL REFERENCES messages(id) ON DELETE SET NULL,
  edited_at timestamptz NULL,
  deleted_at timestamptz NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);
```

## 4. Client-agnostic rule

Do not store data in a way that assumes only a browser client exists.

Examples:

- sessions must support native bearer token flows;
- device table must support mobile push tokens;
- notification state should be per user/device where needed;
- attachment URLs should work across browser, desktop, and mobile.

## 5. Query rules

Every channel/message query must be permission-scoped.

Never return private channel data before access is verified.

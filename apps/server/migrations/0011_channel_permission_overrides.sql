-- 0011_channel_permission_overrides.sql
-- Channel permission overrides
CREATE TABLE IF NOT EXISTS channel_permission_overrides (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id uuid NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    role_id uuid NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_key text NOT NULL,
    denied bool NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(channel_id, role_id, permission_key)
);

CREATE INDEX IF NOT EXISTS idx_channel_permission_overrides_channel_id ON channel_permission_overrides(channel_id);
CREATE INDEX IF NOT EXISTS idx_channel_permission_overrides_role_id ON channel_permission_overrides(role_id);
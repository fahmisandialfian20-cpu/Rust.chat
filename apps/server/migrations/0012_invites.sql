-- 0012_invites.sql
-- Invites table
CREATE TABLE IF NOT EXISTS invites (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    code text UNIQUE NOT NULL,
    code_hash text NOT NULL,
    space_id uuid REFERENCES spaces(id) ON DELETE CASCADE,
    channel_id uuid REFERENCES channels(id) ON DELETE CASCADE,
    created_by uuid NOT NULL REFERENCES users(id),
    max_uses int,
    used_count int NOT NULL DEFAULT 0,
    expires_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_invites_code ON invites(code);
CREATE INDEX IF NOT EXISTS idx_invites_space_id ON invites(space_id);
CREATE INDEX IF NOT EXISTS idx_invites_channel_id ON invites(channel_id);
CREATE INDEX IF NOT EXISTS idx_invites_created_by ON invites(created_by);
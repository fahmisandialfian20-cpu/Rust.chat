-- 0005_space_memberships.sql
-- Space memberships
CREATE TABLE IF NOT EXISTS space_memberships (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    space_id uuid NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    nickname text,
    settings jsonb NOT NULL DEFAULT '{}',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(space_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_space_memberships_space_id ON space_memberships(space_id);
CREATE INDEX IF NOT EXISTS idx_space_memberships_user_id ON space_memberships(user_id);
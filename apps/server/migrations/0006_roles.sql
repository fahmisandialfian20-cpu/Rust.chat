-- 0006_roles.sql
-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    space_id uuid NOT NULL REFERENCES spaces(id) ON DELETE CASCADE,
    name text NOT NULL,
    is_default bool NOT NULL DEFAULT false,
    permissions jsonb NOT NULL DEFAULT '{}',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(space_id, name)
);

CREATE INDEX IF NOT EXISTS idx_roles_space_id ON roles(space_id);
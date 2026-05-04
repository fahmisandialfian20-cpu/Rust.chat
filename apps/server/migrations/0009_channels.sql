-- 0009_channels.sql
-- Channels table
CREATE TABLE IF NOT EXISTS channels (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
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

CREATE INDEX IF NOT EXISTS idx_channels_space_id ON channels(space_id);
CREATE INDEX IF NOT EXISTS idx_channels_parent_id ON channels(parent_id);
CREATE INDEX IF NOT EXISTS idx_channels_created_by ON channels(created_by);
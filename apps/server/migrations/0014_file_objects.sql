-- 0014_file_objects.sql
-- File objects (metadata only, actual files in storage)
CREATE TABLE IF NOT EXISTS file_objects (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    space_id uuid REFERENCES spaces(id) ON DELETE SET NULL,
    channel_id uuid REFERENCES channels(id) ON DELETE SET NULL,
    uploader_user_id uuid NOT NULL REFERENCES users(id),
    filename text NOT NULL,
    content_type text NOT NULL,
    size_bytes bigint NOT NULL,
    storage_key text NOT NULL,
    metadata jsonb NOT NULL DEFAULT '{}',
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_file_objects_space_id ON file_objects(space_id);
CREATE INDEX IF NOT EXISTS idx_file_objects_channel_id ON file_objects(channel_id);
CREATE INDEX IF NOT EXISTS idx_file_objects_uploader_user_id ON file_objects(uploader_user_id);
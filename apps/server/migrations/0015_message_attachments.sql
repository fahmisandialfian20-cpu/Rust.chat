-- 0015_message_attachments.sql
-- Message attachments
CREATE TABLE IF NOT EXISTS message_attachments (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id uuid NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    file_object_id uuid NOT NULL REFERENCES file_objects(id) ON DELETE CASCADE,
    position int NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_message_attachments_message_id ON message_attachments(message_id);
CREATE INDEX IF NOT EXISTS idx_message_attachments_file_object_id ON message_attachments(file_object_id);
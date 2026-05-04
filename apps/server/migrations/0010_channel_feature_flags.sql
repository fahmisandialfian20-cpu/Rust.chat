-- 0010_channel_feature_flags.sql
-- Channel feature flags
CREATE TABLE IF NOT EXISTS channel_feature_flags (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id uuid NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    text_enabled bool NOT NULL DEFAULT true,
    send_file_enabled bool NOT NULL DEFAULT true,
    voice_group_enabled bool NOT NULL DEFAULT false,
    video_group_enabled bool NOT NULL DEFAULT false,
    threads_enabled bool NOT NULL DEFAULT true,
    reactions_enabled bool NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_channel_feature_flags_channel_id ON channel_feature_flags(channel_id);
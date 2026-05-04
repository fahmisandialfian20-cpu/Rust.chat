-- 0003_instance_settings.sql
-- Single instance settings
CREATE TABLE IF NOT EXISTS instance_settings (
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
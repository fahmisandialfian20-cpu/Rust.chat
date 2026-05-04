-- 0018_client_devices.sql
-- Client devices
CREATE TABLE IF NOT EXISTS client_devices (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_type text NOT NULL CHECK (client_type IN ('web', 'desktop', 'mobile', 'bot')),
    platform text,
    device_name text,
    push_token text,
    last_seen_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_client_devices_user_id ON client_devices(user_id);
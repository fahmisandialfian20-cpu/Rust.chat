-- 0016_audit_logs.sql
-- Audit logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    space_id uuid REFERENCES spaces(id) ON DELETE SET NULL,
    action text NOT NULL,
    target_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
    target_space_id uuid REFERENCES spaces(id) ON DELETE SET NULL,
    target_channel_id uuid REFERENCES channels(id) ON DELETE SET NULL,
    metadata jsonb NOT NULL DEFAULT '{}',
    ip_address text,
    user_agent text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_space_id ON audit_logs(space_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at DESC);
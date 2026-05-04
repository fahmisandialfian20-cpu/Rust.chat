-- 0008_member_roles.sql
-- Member role assignments
CREATE TABLE IF NOT EXISTS member_roles (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    membership_id uuid NOT NULL REFERENCES space_memberships(id) ON DELETE CASCADE,
    role_id uuid NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(membership_id, role_id)
);

CREATE INDEX IF NOT EXISTS idx_member_roles_membership_id ON member_roles(membership_id);
CREATE INDEX IF NOT EXISTS idx_member_roles_role_id ON member_roles(role_id);
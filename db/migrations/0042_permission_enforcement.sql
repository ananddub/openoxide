-- Existing accounts retain full access while the RBAC configuration UI and
-- organization-specific roles are rolled out. New users are not inserted here.
CREATE TABLE permission_legacy_full_access (
    user_id INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    granted_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

INSERT OR IGNORE INTO permission_legacy_full_access (user_id)
SELECT id FROM users;

CREATE UNIQUE INDEX IF NOT EXISTS idx_group_policy_unique
ON group_policy(group_id, policy_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_organization_members_unique
ON organization_members(user_id, organization_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_resource_access_unique
ON resource_access(user_id, org_id, resource_type, resource_id);

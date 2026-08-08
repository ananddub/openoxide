ALTER TABLE groups ADD COLUMN organization_id INTEGER REFERENCES organization(id) ON DELETE CASCADE;
ALTER TABLE organization_members ADD COLUMN group_id INTEGER REFERENCES groups(id) ON DELETE SET NULL;
UPDATE organization_members SET group_id = (SELECT group_id FROM users WHERE users.id = organization_members.user_id) WHERE group_id IS NULL;
CREATE INDEX IF NOT EXISTS idx_groups_organization ON groups(organization_id);
CREATE INDEX IF NOT EXISTS idx_org_members_group ON organization_members(group_id);
INSERT OR IGNORE INTO policy(action) VALUES ('group:read'), ('group:create'), ('group:update'), ('group:delete'), ('member:read'), ('member:update');

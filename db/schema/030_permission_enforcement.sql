CREATE UNIQUE INDEX idx_group_policy_unique ON group_policy(group_id, policy_id);
CREATE UNIQUE INDEX idx_organization_members_unique ON organization_members(user_id, organization_id);
CREATE UNIQUE INDEX idx_resource_access_unique ON resource_access(user_id, org_id, resource_type, resource_id);

-- Extracted from src/db/repository/permission_groups.rs. Do not edit repository SQL directly.

-- name: PermissionGroupsSeedOrganizationDefaults1 :many
SELECT id FROM groups WHERE organization_id = ? AND name = 'Organization Admin';

-- name: PermissionGroupsSeedOrganizationDefaults2 :exec
INSERT INTO groups (name, is_system, organization_id) VALUES ('Organization Admin', 1, ?);

-- name: PermissionGroupsSeedOrganizationDefaults3 :many
SELECT id FROM policy WHERE action LIKE '%:read' OR action LIKE '%:create' OR action LIKE '%:update' OR action LIKE '%:delete';

-- name: PermissionGroupsSeedOrganizationDefaults4 :exec
INSERT OR IGNORE INTO group_policy (group_id, policy_id) VALUES (?, ?);

-- name: PermissionGroupsListGroups :many
SELECT id, name, is_system != 0 AS is_system, organization_id, created_at, updated_at FROM groups WHERE organization_id IS NULL OR organization_id = ? ORDER BY is_system DESC, name;

-- name: PermissionGroupsGroupActions :many
SELECT p.action FROM group_policy gp JOIN policy p ON p.id = gp.policy_id WHERE gp.group_id = ? ORDER BY p.action;

-- name: PermissionGroupsAllPolicies :many
SELECT id, action FROM policy ORDER BY action;

-- name: PermissionGroupsEffectiveActions :many
WITH overrides AS (
                SELECT p.action, up.effect
                FROM user_policy up
                JOIN policy p ON p.id = up.policy_id
                WHERE up.user_id = ? AND up.org_id = ?
            )
            SELECT DISTINCT action FROM (
                SELECT p.action
                FROM organization_members om
                JOIN group_policy gp ON gp.group_id = om.group_id
                JOIN policy p ON p.id = gp.policy_id
                WHERE om.user_id = ? AND om.organization_id = ?
                UNION ALL
                SELECT action FROM overrides WHERE effect = 'GRANT'
            )
            WHERE action NOT IN (SELECT action FROM overrides WHERE effect = 'DENY')
            ORDER BY action;

-- name: PermissionGroupsCreateGroup1 :exec
INSERT INTO groups (name, is_system, organization_id) VALUES (?, 0, ?);

-- name: PermissionGroupsCreateGroup2 :exec
INSERT INTO group_policy (group_id, policy_id) VALUES (?, ?);

-- name: PermissionGroupsUpdateGroup1 :exec
UPDATE groups SET name = ? WHERE id = ? AND organization_id = ? AND is_system = 0;

-- name: PermissionGroupsUpdateGroup2 :exec
DELETE FROM group_policy WHERE group_id = ?;

-- name: PermissionGroupsUpdateGroup3 :exec
INSERT INTO group_policy (group_id, policy_id) VALUES (?, ?);

-- name: PermissionGroupsDeleteGroup1 :one
SELECT EXISTS(SELECT 1 FROM organization_members WHERE organization_id = ? AND group_id = ?);

-- name: PermissionGroupsDeleteGroup2 :exec
DELETE FROM groups WHERE id = ? AND organization_id = ? AND is_system = 0;

-- name: PermissionGroupsAssignGroup1 :one
SELECT EXISTS(SELECT 1 FROM groups WHERE id = ? AND (organization_id IS NULL OR organization_id = ?));

-- name: PermissionGroupsAssignGroup2 :exec
UPDATE organization_members SET group_id = ? WHERE user_id = ? AND organization_id = ?;

-- name: PermissionGroupsReplaceUserPolicies1 :one
SELECT EXISTS(SELECT 1 FROM organization_members WHERE user_id = ? AND organization_id = ?);

-- name: PermissionGroupsReplaceUserPolicies2 :exec
DELETE FROM user_policy WHERE user_id = ? AND org_id = ?;

-- name: PermissionGroupsReplaceUserPolicies3 :exec
INSERT INTO user_policy (user_id, org_id, policy_id, effect) VALUES (?, ?, ?, ?);

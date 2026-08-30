-- Permission group management (RBAC): seed, list, group policies, assign,
-- user policy overrides. Multi-step operations are split into numbered
-- queries; callers run them in sequence.

-- Seed 'Organization Admin' system group with read/create/update/delete
-- policies. Steps: (1) existing check, (2) insert group, (3) matching
-- policy ids, (4) link each policy.

-- name: PermissionGroupsAdminGroupExists :one
SELECT COUNT(*) AS hit
FROM groups
WHERE organization_id = ? AND name = 'Organization Admin';

-- name: PermissionGroupsInsertAdminGroup :one
INSERT INTO groups (name, is_system, organization_id)
VALUES ('Organization Admin', 1, ?)
RETURNING id;

-- name: PermissionGroupsCrudPolicyIds :many
SELECT id AS policy_id
FROM policy
WHERE action LIKE '%:read' OR action LIKE '%:create' OR action LIKE '%:update' OR action LIKE '%:delete';

-- name: PermissionGroupsAddPolicyToGroup :exec
INSERT OR IGNORE INTO group_policy (group_id, policy_id)
VALUES (?, ?);

-- Groups visible to an organization: global (NULL org) + org-owned,
-- system groups first.

-- name: ListGroupsForOrganization :many
SELECT id AS id,
       name AS name,
       is_system != 0 AS is_system,
       organization_id AS organization_id,
       created_at AS created_at,
       updated_at AS updated_at
FROM groups
WHERE organization_id IS NULL OR organization_id = ?
ORDER BY is_system DESC, name;

-- Actions granted to a group through group_policy.

-- name: GroupActions :many
SELECT p.action AS action
FROM group_policy gp
JOIN policy p ON p.id = gp.policy_id
WHERE gp.group_id = ?
ORDER BY p.action;

-- Custom group create/update/delete (is_system = 0 only).

-- name: PermissionGroupsCreateGroup :one
INSERT INTO groups (name, is_system, organization_id)
VALUES (?, 0, ?)
RETURNING id;

-- name: PermissionGroupsUpdateGroupName :many
UPDATE groups
SET name = ?
WHERE id = ? AND organization_id = ? AND is_system = 0
RETURNING id;

-- name: PermissionGroupsClearGroupPolicies :exec
DELETE FROM group_policy WHERE group_id = ?;

-- name: PermissionGroupsDeleteGroupAssigned :one
SELECT COUNT(*) AS assigned
FROM organization_members
WHERE organization_id = ? AND group_id = ?;

-- name: PermissionGroupsDeleteGroup :many
DELETE FROM groups
WHERE id = ? AND organization_id = ? AND is_system = 0
RETURNING id;

-- Assign a group to an organization member: (1) the group must be global
-- or belong to the organization, (2) member row must exist.

-- name: PermissionGroupsValidGroupScope :one
SELECT COUNT(*) AS hit
FROM groups
WHERE id = ? AND (organization_id IS NULL OR organization_id = ?);

-- name: PermissionGroupsAssignGroup :many
UPDATE organization_members
SET group_id = ?, updated_at = unixepoch()
WHERE user_id = ? AND organization_id = ?
RETURNING id;

-- Replace all user_policy overrides for a member: (1) member check,
-- (2) wipe existing, (3) insert each override.

-- name: PermissionGroupsMemberExists :one
SELECT COUNT(*) AS hit
FROM organization_members
WHERE user_id = ? AND organization_id = ?;

-- name: PermissionGroupsDeleteUserPolicies :exec
DELETE FROM user_policy WHERE user_id = ? AND org_id = ?;

-- name: PermissionGroupsInsertUserPolicy :exec
INSERT INTO user_policy (user_id, org_id, policy_id, effect)
VALUES (?, ?, ?, ?);

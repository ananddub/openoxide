-- ============================================================
-- GROUPS
-- ============================================================

-- name: GetGroupByID :one
SELECT * FROM groups WHERE id = ?;

-- name: GetGroupByName :one
SELECT * FROM groups WHERE name = ?;

-- name: ListGroups :many
SELECT * FROM groups ORDER BY created_at DESC;

-- name: CreateGroup :one
INSERT INTO groups (name) VALUES (?) RETURNING *;

-- name: UpdateGroup :one
UPDATE groups SET name = ? WHERE id = ? RETURNING *;

-- name: DeleteGroup :exec
DELETE FROM groups WHERE id = ?;

-- ============================================================
-- POLICIES
-- ============================================================

-- name: GetPolicyByID :one
SELECT * FROM policy WHERE id = ?;

-- name: GetPolicyByAction :one
SELECT * FROM policy WHERE action = ?;

-- name: ListPolicies :many
SELECT * FROM policy ORDER BY action ASC;

-- name: CreatePolicy :one
INSERT INTO policy (action) VALUES (?) RETURNING *;

-- name: DeletePolicy :exec
DELETE FROM policy WHERE id = ?;

-- name: CreateSystemGroup :one
INSERT INTO groups (name, is_system) VALUES (?, 1) RETURNING *;

-- ============================================================
-- GROUP POLICIES
-- ============================================================

-- name: ListPoliciesByGroup :many
SELECT p.* FROM policy p
JOIN group_policy gp ON gp.policy_id = p.id
WHERE gp.group_id = ?
ORDER BY p.action ASC;

-- name: AddPolicyToGroup :one
INSERT INTO group_policy (group_id, policy_id) VALUES (?, ?) RETURNING *;

-- name: RemovePolicyFromGroup :exec
DELETE FROM group_policy WHERE group_id = ? AND policy_id = ?;

-- name: ClearGroupPolicies :exec
DELETE FROM group_policy WHERE group_id = ?;

-- ============================================================
-- USER PERMISSIONS & RBAC RESOLUTION
-- ============================================================

-- name: GroupsGetAll :many
SELECT id AS id, name AS name, created_at AS created_at, updated_at AS updated_at FROM groups;

-- name: GroupsGetById :one
SELECT id AS id, name AS name, created_at AS created_at, updated_at AS updated_at FROM groups WHERE id = ?;

-- name: GroupsCreate :exec
INSERT INTO groups (name, created_at, updated_at) VALUES (?, ?, ?);

-- name: GroupsUpdate :exec
UPDATE groups SET name = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: GroupsDelete :exec
DELETE FROM groups WHERE id = ?;

-- name: GroupsCreateOwnerGroupIfNotExists1 :exec
INSERT INTO groups (name, is_system, organization_id)
SELECT 'owner', 1, NULL
WHERE NOT EXISTS (SELECT 1 FROM groups WHERE name = 'owner');

-- name: GroupsCreateOwnerGroupIfNotExists2 :one
SELECT id FROM groups WHERE name = 'owner';

-- name: GroupsGetUserFinalPermissions :many
WITH user_permissions AS (
                SELECT p.action, up.effect
                FROM user_policy up
                JOIN policy p ON p.id = up.policy_id
                WHERE up.user_id = ? AND up.org_id = ?
            )
            SELECT DISTINCT action FROM (
                SELECT p.action
                FROM group_policy gp
                JOIN policy p ON p.id = gp.policy_id
                JOIN organization_members om ON om.group_id = gp.group_id
                JOIN groups g ON g.id = om.group_id AND g.is_active = 1
                WHERE om.user_id = ? AND om.organization_id = ?
                UNION ALL
                SELECT action FROM user_permissions WHERE effect = 'GRANT'
            ) perms
            WHERE action NOT IN (SELECT action FROM user_permissions WHERE effect = 'DENY');

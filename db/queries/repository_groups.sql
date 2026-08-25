-- Extracted from src/db/repository/groups.rs. Do not edit repository SQL directly.

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
INSERT OR IGNORE INTO groups (name) VALUES ('owner');

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
                WHERE om.user_id = ? AND om.organization_id = ?
                UNION ALL
                SELECT action FROM user_permissions WHERE effect = 'GRANT'
            ) perms
            WHERE action NOT IN (SELECT action FROM user_permissions WHERE effect = 'DENY');

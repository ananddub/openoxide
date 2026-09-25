-- Extracted from src/db/repository/organization_members.rs. Do not edit repository SQL directly.

-- name: OrganizationMembersFirstOrganizationId :one
SELECT organization_id FROM organization_members WHERE user_id=? ORDER BY id LIMIT 1;

-- name: OrganizationMembersRole :one
SELECT role FROM organization_members WHERE user_id=? AND organization_id=?;

-- name: OrganizationMembersListForOrganization :many
SELECT id AS id, role AS role, user_id AS user_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM organization_members WHERE organization_id = ? ORDER BY created_at ASC, id ASC;

-- name: OrganizationMembersListWithUserForOrganization :many
SELECT om.id AS id, om.role AS role, om.user_id AS user_id, om.organization_id AS organization_id, om.created_at AS created_at, om.updated_at AS updated_at, om.group_id AS group_id, u.email AS email, u.avatar AS avatar, COALESCE(g.name, '') AS group_name FROM organization_members om JOIN users u ON u.id = om.user_id LEFT JOIN groups g ON g.id = om.group_id WHERE om.organization_id = ? ORDER BY om.created_at ASC, om.id ASC;

-- name: OrganizationMembersUpdateRole :exec
UPDATE organization_members SET role = ?, updated_at = unixepoch() WHERE user_id = ? AND organization_id = ?;

-- name: OrganizationMembersRemoveMember :exec
DELETE FROM organization_members WHERE user_id = ? AND organization_id = ?;

-- name: OrganizationMembersGetAll :many
SELECT id AS id, role AS role, user_id AS user_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM organization_members;

-- name: OrganizationMembersGetById :one
SELECT id AS id, role AS role, user_id AS user_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM organization_members WHERE id = ?;

-- name: OrganizationMembersCreate :exec
INSERT INTO organization_members (role, user_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?);

-- name: OrganizationMembersUpdate :exec
UPDATE organization_members SET role = ?, user_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: OrganizationMembersDelete :exec
DELETE FROM organization_members WHERE id = ?;

-- name: OrganizationMembersAddMemberInTransaction :exec
INSERT INTO organization_members (role, user_id, organization_id) VALUES (?, ?, ?);

-- name: OrganizationMembersAddMemberWithGroupInTransaction :exec
INSERT INTO organization_members (role, user_id, organization_id, group_id) VALUES (?, ?, ?, ?);

-- name: OrganizationMembersAddMemberWithGroup :exec
INSERT INTO organization_members (role, user_id, organization_id, group_id) VALUES (?, ?, ?, ?);

-- name: OrganizationMembersClearGroup :many
UPDATE organization_members SET group_id = NULL, updated_at = unixepoch() WHERE user_id = ? AND organization_id = ? RETURNING id;

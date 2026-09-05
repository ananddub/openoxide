-- ============================================================
-- ORGANIZATION
-- ============================================================

-- name: GetOrganizationByID :one
SELECT * FROM organization WHERE id = ?;

-- name: GetOrganizationBySlug :one
SELECT * FROM organization WHERE slug = ?;

-- name: GetOrganizationByName :one
SELECT * FROM organization WHERE name = ?;

-- name: ListOrganizationsByOwner :many
SELECT * FROM organization WHERE owner_id = ? ORDER BY created_at DESC;

-- name: CreateOrganization :one
INSERT INTO organization (name, logo, slug, owner_id)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: UpdateOrganization :one
UPDATE organization
SET name = ?, logo = ?, slug = ?
WHERE id = ?
RETURNING *;

-- name: DeleteOrganization :exec
DELETE FROM organization WHERE id = ?;

-- ============================================================
-- ORGANIZATION MEMBERS
-- ============================================================

-- name: GetMemberByID :one
SELECT * FROM organization_members WHERE id = ?;

-- name: GetMemberByUserAndOrg :one
SELECT * FROM organization_members
WHERE user_id = ? AND organization_id = ?;

-- name: ListMembersByOrg :many
SELECT
    om.*,
    u.email, u.first_name, u.last_name, u.avatar, u.role AS user_role
FROM organization_members om
JOIN users u ON u.id = om.user_id
WHERE om.organization_id = ?
ORDER BY om.created_at DESC;

-- name: AddMember :one
INSERT INTO organization_members (role, user_id, organization_id)
VALUES (?, ?, ?)
RETURNING *;

-- name: UpdateMemberRole :exec
UPDATE organization_members SET role = ?
WHERE user_id = ? AND organization_id = ?;

-- name: RemoveMember :exec
DELETE FROM organization_members
WHERE user_id = ? AND organization_id = ?;

-- name: CountMembersByOrg :one
SELECT COUNT(*) FROM organization_members WHERE organization_id = ?;

-- ============================================================
-- ORGANIZATION INVITES
-- ============================================================

-- name: GetInviteByToken :one
SELECT * FROM organization_invites WHERE token = ?;

-- name: GetInviteByEmailAndOrg :one
SELECT * FROM organization_invites
WHERE email = ? AND organization_id = ? AND status = 'PENDING';

-- name: ListInvitesByOrg :many
SELECT * FROM organization_invites
WHERE organization_id = ?
ORDER BY created_at DESC;

-- name: CreateInvite :one
INSERT INTO organization_invites (
    email, role, token, group_id, organization_id, invited_by, expired_at
) VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateInviteStatus :exec
UPDATE organization_invites SET status = ? WHERE token = ?;

-- name: DeleteInvite :exec
DELETE FROM organization_invites WHERE id = ?;

-- name: DeleteExpiredInvites :exec
DELETE FROM organization_invites
WHERE expired_at < strftime('%s','now') AND status = 'PENDING';

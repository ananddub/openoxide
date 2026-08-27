-- Extracted from src/db/repository/organization_invites.rs. Do not edit repository SQL directly.

-- name: OrganizationInvitesGetAll :many
SELECT id AS id, email AS email, role AS role, status AS status, token AS token, group_id AS group_id, organization_id AS organization_id, invited_by AS invited_by, expired_at AS expired_at, created_at AS created_at FROM organization_invites;

-- name: OrganizationInvitesGetById :one
SELECT id AS id, email AS email, role AS role, status AS status, token AS token, group_id AS group_id, organization_id AS organization_id, invited_by AS invited_by, expired_at AS expired_at, created_at AS created_at FROM organization_invites WHERE id = ?;

-- name: OrganizationInvitesCreate :exec
INSERT INTO organization_invites (email, role, status, token, group_id, organization_id, invited_by, expired_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: OrganizationInvitesUpdate :exec
UPDATE organization_invites SET email = ?, role = ?, status = ?, token = ?, group_id = ?, organization_id = ?, invited_by = ?, expired_at = ?, created_at = ? WHERE id = ?;

-- name: OrganizationInvitesDelete :exec
DELETE FROM organization_invites WHERE id = ?;

-- name: OrganizationInvitesListPendingForOrganization :many
SELECT id AS id, email AS email, role AS role, status AS status, token AS token, group_id AS group_id, organization_id AS organization_id, invited_by AS invited_by, expired_at AS expired_at, created_at AS created_at FROM organization_invites WHERE organization_id = ? AND status = 'PENDING' AND expired_at > strftime('%s', 'now') ORDER BY created_at DESC;

-- name: OrganizationInvitesFindPendingByToken :one
SELECT id AS id, email AS email, role AS role, status AS status, token AS token, group_id AS group_id, organization_id AS organization_id, invited_by AS invited_by, expired_at AS expired_at, created_at AS created_at FROM organization_invites WHERE token = ? AND status = 'PENDING' AND expired_at > strftime('%s', 'now');

-- name: OrganizationInvitesSetStatus :exec
UPDATE organization_invites SET status = ? WHERE id = ? AND status = 'PENDING';

-- name: OrganizationInvitesPurgeExpired :exec
UPDATE organization_invites SET status = 'REJECTED' WHERE status = 'PENDING' AND expired_at <= strftime('%s', 'now');

-- Extracted from src/db/repository/organization.rs. Do not edit repository SQL directly.

-- name: OrganizationGetAll :many
SELECT id AS id, name AS name, logo AS logo, slug AS slug, owner_id AS owner_id, created_at AS created_at, updated_at AS updated_at FROM organization;

-- name: OrganizationGetById :one
SELECT id AS id, name AS name, logo AS logo, slug AS slug, owner_id AS owner_id, created_at AS created_at, updated_at AS updated_at FROM organization WHERE id = ?;

-- name: OrganizationCreate :exec
INSERT INTO organization (name, logo, slug, owner_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?);

-- name: OrganizationUpdate :exec
UPDATE organization SET name = ?, logo = ?, slug = ?, owner_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: OrganizationDelete :exec
DELETE FROM organization WHERE id = ?;

-- name: OrganizationListByOwner :many
SELECT id AS id, name, logo, slug, owner_id, created_at, updated_at
               FROM organization WHERE owner_id = ? ORDER BY created_at DESC, id DESC;

-- name: OrganizationCreateInTransaction :one
INSERT INTO organization (name, logo, slug, owner_id) VALUES (?, ?, ?, ?)
               RETURNING id AS id, name, logo, slug, owner_id, created_at, updated_at;

-- name: OrganizationUpdateAndReturn :one
UPDATE organization SET name = ?, logo = ?, slug = ? WHERE id = ?
               RETURNING id AS id, name, logo, slug, owner_id, created_at, updated_at;

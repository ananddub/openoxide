-- Extracted from src/db/repository/certificates.rs. Do not edit repository SQL directly.

-- name: CertificatesGetAll :many
SELECT id AS id, name AS name, certificate_data AS certificate_data, private_key AS private_key, certificate_path AS certificate_path, auto_renew AS auto_renew, server_id AS server_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM certificates WHERE organization_id = ? ORDER BY created_at DESC, id DESC;

-- name: CertificatesGetByIdForOrganization :one
SELECT id AS id, name AS name, certificate_data AS certificate_data, private_key AS private_key, certificate_path AS certificate_path, auto_renew AS auto_renew, server_id AS server_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM certificates WHERE id = ? AND organization_id = ?;

-- name: CertificatesGetById :one
SELECT id AS id, name AS name, certificate_data AS certificate_data, private_key AS private_key, certificate_path AS certificate_path, auto_renew AS auto_renew, server_id AS server_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM certificates WHERE id = ?;

-- name: CertificatesCreate :exec
INSERT INTO certificates (name, certificate_data, private_key, certificate_path, auto_renew, server_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: CertificatesUpdate :exec
UPDATE certificates SET name = ?, certificate_data = ?, private_key = ?, certificate_path = ?, auto_renew = ?, server_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: CertificatesDeleteForOrganization :exec
DELETE FROM certificates WHERE id = ? AND organization_id = ?;

-- name: CertificatesDelete :exec
DELETE FROM certificates WHERE id = ?;

-- ============================================================
-- CERTIFICATES
-- ============================================================

-- name: GetCertificateByID :one
SELECT * FROM certificates WHERE id = ?;

-- name: GetCertificateByPath :one
SELECT * FROM certificates WHERE certificate_path = ?;

-- name: ListCertificatesByOrg :many
SELECT * FROM certificates WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListCertificatesByServer :many
SELECT * FROM certificates WHERE server_id = ? ORDER BY created_at DESC;

-- name: CreateCertificate :one
INSERT INTO certificates (
    name, certificate_data, private_key,
    certificate_path, auto_renew, server_id, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateCertificate :one
UPDATE certificates
SET
    name             = ?,
    certificate_data = ?,
    private_key      = ?,
    certificate_path = ?,
    auto_renew       = ?
WHERE id = ?
RETURNING *;

-- name: DeleteCertificate :exec
DELETE FROM certificates WHERE id = ?;


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

-- Certificate renewal execution records.

-- name: CertificateRenewalsBegin :one
INSERT INTO certificate_renewals (certificate_id, organization_id, status, previous_expires_at)
VALUES (?, ?, 'RUNNING', ?)
RETURNING id;

-- name: CertificateRenewalsFinish :exec
UPDATE certificate_renewals
SET status = ?, new_expires_at = ?, error = ?, finished_at = unixepoch()
WHERE id = ?;

-- name: CertificateRenewalsList :many
SELECT id AS id, certificate_id AS certificate_id, organization_id AS organization_id, status AS status, previous_expires_at, new_expires_at, error, started_at AS started_at, finished_at FROM certificate_renewals WHERE certificate_id=? ORDER BY started_at DESC LIMIT 200;

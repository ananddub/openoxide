-- Backup execution records (attempt tracking for backups + volume backups).

-- name: BackupExecutionsNextAttempt :one
SELECT COALESCE(MAX(attempt), 0) + 1 FROM backup_executions WHERE backup_kind = ? AND operation = ? AND backup_id = ?;

-- name: BackupExecutionsStart :one
INSERT INTO backup_executions (backup_kind, operation, backup_id, object_key, attempt) VALUES (?, ?, ?, ?, ?) RETURNING id;

-- name: BackupExecutionsSucceed :exec
UPDATE backup_executions
SET status = 'SUCCEEDED', sha256_checksum = ?, size_bytes = ?, finished_at = unixepoch()
WHERE id = ?;

-- name: BackupExecutionsFail :exec
UPDATE backup_executions
SET status = 'FAILED', error = ?, finished_at = unixepoch()
WHERE id = ?;

-- name: BackupExecutionsList :many
SELECT id AS id, backup_kind AS backup_kind, operation AS operation, backup_id AS backup_id,
       status AS status, object_key AS object_key, sha256_checksum AS checksum_sha256,
       size_bytes AS size_bytes, attempt AS attempt, error AS error,
       started_at AS started_at, finished_at AS finished_at
FROM backup_executions
WHERE backup_kind = COALESCE(?, backup_kind)
  AND backup_id = COALESCE(?, backup_id)
ORDER BY started_at DESC, id DESC LIMIT ?;

-- name: BackupExecutionsGet :one
SELECT id AS id, backup_kind AS backup_kind, operation AS operation, backup_id AS backup_id,
       status AS status, object_key AS object_key, sha256_checksum AS checksum_sha256,
       size_bytes AS size_bytes, attempt AS attempt, error AS error,
       started_at AS started_at, finished_at AS finished_at
FROM backup_executions WHERE id = ?;

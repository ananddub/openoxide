-- Extracted from src/db/repository/backup_executions.rs. Do not edit repository SQL directly.

-- name: BackupExecutionsStart :exec
INSERT INTO backup_executions
               (backup_kind, operation, backup_id, object_key, attempt)
               SELECT ?, ?, ?, ?, COALESCE(MAX(attempt), 0) + 1
               FROM backup_executions
               WHERE backup_kind = ? AND operation = ?
                 AND ((backup_id IS NULL AND ? IS NULL) OR backup_id = ?);

-- name: BackupExecutionsSucceed :exec
UPDATE backup_executions
SET status = 'SUCCEEDED', sha256_checksum = ?, size_bytes = ?,
                   finished_at = strftime('%s', 'now')
               WHERE id = ?;

-- name: BackupExecutionsFail :exec
UPDATE backup_executions
               SET status = 'FAILED', error = ?, finished_at = strftime('%s', 'now')
               WHERE id = ?;

-- name: BackupExecutionsList :many
SELECT id AS id, backup_kind, operation, backup_id, status,
                      object_key, sha256_checksum, size_bytes, attempt, error,
                      started_at, finished_at
               FROM backup_executions
               WHERE (? IS NULL OR backup_kind = ?)
                 AND (? IS NULL OR backup_id = ?)
               ORDER BY started_at DESC, id DESC LIMIT ?;

-- name: BackupExecutionsGet :one
SELECT id AS id, backup_kind, operation, backup_id, status,
                      object_key, sha256_checksum, size_bytes, attempt, error,
                      started_at, finished_at
               FROM backup_executions WHERE id = ?;

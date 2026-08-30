-- Server infrastructure extras used by management/cleanup/lifecycle services.
-- Fixes retired repository semantics: unixepoch() timestamps (not
-- CURRENT_TIMESTAMP) and cleanup execution ids via RETURNING.

-- name: SrvInfManagementUpdate :one
UPDATE server_management SET maintenance_mode = ?, maintenance_message = ?,
               labels = ?, cleanup_policy = ?, gpu_enabled = ?, updated_at = unixepoch()
               WHERE server_id = ?
               RETURNING server_id, maintenance_mode, maintenance_message, labels, cleanup_policy, gpu_enabled, updated_at;

-- name: SrvInfStartCleanup :one
INSERT INTO server_cleanup_executions (server_id, policy) VALUES (?, ?)
               RETURNING id;

-- name: SrvInfFinishCleanup :exec
UPDATE server_cleanup_executions SET status = ?, stdout = ?, stderr = ?,
               finished_at = unixepoch() WHERE id = ?;

-- Extracted from src/db/repository/server_management.rs. Do not edit repository SQL directly.

-- name: ServerManagementGetOrCreate1 :exec
INSERT INTO server_management (server_id) VALUES (?) ON CONFLICT(server_id) DO NOTHING;

-- name: ServerManagementGetOrCreate2 :one
SELECT server_id AS server_id, maintenance_mode, maintenance_message,
                      labels, cleanup_policy, gpu_enabled, updated_at
               FROM server_management WHERE server_id = ?;

-- name: ServerManagementUpdate :exec
UPDATE server_management SET maintenance_mode = ?, maintenance_message = ?,
               labels = ?, cleanup_policy = ?, gpu_enabled = ?, updated_at = strftime('%s', 'now')
               WHERE server_id = ?;

-- name: ServerManagementStartCleanup :exec
INSERT INTO server_cleanup_executions (server_id, policy) VALUES (?, ?);

-- name: ServerManagementFinishCleanup :exec
UPDATE server_cleanup_executions SET status = ?, stdout = ?, stderr = ?,
               finished_at = strftime('%s', 'now') WHERE id = ?;

-- name: ServerManagementListCleanup :many
SELECT id AS id, server_id, status, policy, stdout, stderr,
                      started_at, finished_at
               FROM server_cleanup_executions WHERE server_id = ?
               ORDER BY started_at DESC, id DESC LIMIT 100;

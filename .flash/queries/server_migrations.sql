-- Extracted from src/db/repository/server_migrations.rs. Do not edit repository SQL directly.

-- name: ServerMigrationsBegin :exec
INSERT INTO server_migrations (id, source_server_id, target_server_id, status, service_ids, build_service_ids, certificate_ids, schedule_ids) VALUES (?, ?, ?, 'RUNNING', ?, ?, ?, ?);

-- name: ServerMigrationsFinishSuccess :exec
UPDATE server_migrations SET status = 'SUCCEEDED', queued_services = ?, queued_total = ?, error = NULL, updated_at = unixepoch() WHERE id = ?;

-- name: ServerMigrationsFinishFailure :exec
UPDATE server_migrations SET status = 'FAILED', queued_services = ?, queued_total = ?, error = ?, updated_at = unixepoch() WHERE id = ?;

-- name: ServerMigrationsGet :one
SELECT id, source_server_id, target_server_id, status, service_ids, build_service_ids, certificate_ids, schedule_ids, queued_services, queued_total, error, created_at, updated_at FROM server_migrations WHERE id = ?;

-- name: ServerMigrationsMarkRolledBack :exec
UPDATE server_migrations SET status = 'ROLLED_BACK', updated_at = unixepoch() WHERE id = ?;

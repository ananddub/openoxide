-- Extracted from src/db/repository/server_migrations.rs. Do not edit repository SQL directly.

-- name: ServerMigrationsBegin :exec
INSERT INTO server_migrations (id, source_server_id, target_server_id, status, application_ids, build_application_ids, compose_ids, certificate_ids, schedule_ids) VALUES (?, ?, ?, 'RUNNING', ?, ?, ?, ?, ?);

-- name: ServerMigrationsFinish :exec
UPDATE server_migrations SET status = ?, queued_applications = ?, queued_compose_projects = ?, error = ?, updated_at = unixepoch() WHERE id = ?;

-- name: ServerMigrationsGet :one
SELECT id, source_server_id, target_server_id, status, application_ids, build_application_ids, compose_ids, certificate_ids, schedule_ids, queued_applications, queued_compose_projects, error, created_at, updated_at FROM server_migrations WHERE id = ?;

-- name: ServerMigrationsMarkRolledBack :exec
UPDATE server_migrations SET status = 'ROLLED_BACK', updated_at = unixepoch() WHERE id = ?;

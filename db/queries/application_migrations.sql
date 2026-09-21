-- ============================================================
-- APPLICATION MIGRATIONS (server-to-server app transfer)
-- ============================================================

-- name: ApplicationMigrationsBegin :one
INSERT INTO application_migrations (
    id, organization_id, service_id, source_server_id, target_server_id,
    status, phase, progress, log_path, created_at, updated_at
) VALUES (?, ?, ?, ?, ?, 'RUNNING', 'PREFLIGHT', 0, ?, ?, ?)
RETURNING *;

-- name: ApplicationMigrationsUpdatePhase :exec
UPDATE application_migrations
SET phase = ?, progress = ?, updated_at = unixepoch()
WHERE id = ?;

-- name: ApplicationMigrationsRequestCancel :exec
UPDATE application_migrations
SET cancel_requested = 1, updated_at = unixepoch()
WHERE id = ? AND status = 'RUNNING';

-- name: ApplicationMigrationsFinishSuccess :exec
UPDATE application_migrations
SET status = 'SUCCEEDED', phase = 'DONE', progress = 100, error = NULL,
    updated_at = unixepoch()
WHERE id = ?;

-- name: ApplicationMigrationsFinishFailure :exec
UPDATE application_migrations
SET status = 'FAILED', error = ?, updated_at = unixepoch()
WHERE id = ?;

-- name: ApplicationMigrationsFinishCancelled :exec
UPDATE application_migrations
SET status = 'CANCELLED', updated_at = unixepoch()
WHERE id = ?;

-- name: ApplicationMigrationsGet :one
SELECT * FROM application_migrations WHERE id = ?;

-- name: ApplicationMigrationsListByService :many
SELECT * FROM application_migrations
WHERE service_id = ?
ORDER BY created_at DESC;

-- name: ApplicationMigrationsHasRunning :one
-- NOTE (flash v2.8.23): the correlated EXISTS subquery makes the generated
-- parameter `&str`; SQLite would compare it against a BIGINT column without
-- affinity coercion. Do NOT call this from byte-path-critical code — the
-- queue's QUIESCE gate is implemented in plain SQL inside
-- DeploymentsGetQueuedDeploymentsGrouped (NOT EXISTS join), and handlers use
-- ApplicationMigrationsListByService + an in-memory status filter.
SELECT EXISTS(
    SELECT 1 FROM application_migrations
    WHERE service_id = ? AND status = 'RUNNING'
) AS exists_val;

-- ============================================================
-- DEPLOYMENTS
-- ============================================================

-- name: GetDeploymentByID :one
SELECT * FROM deployments WHERE id = ?;

-- name: ListDeploymentsByApplication :many
SELECT * FROM deployments
WHERE application_id = ?
ORDER BY created_at DESC;

-- name: ListDeploymentsByCompose :many
SELECT * FROM deployments
WHERE compose_id = ?
ORDER BY created_at DESC;

-- name: ListDeploymentsByServer :many
SELECT * FROM deployments
WHERE server_id = ?
ORDER BY created_at DESC;

-- name: ListDeploymentsByStatus :many
SELECT * FROM deployments
WHERE status = ?
ORDER BY created_at DESC;

-- name: ListRecentDeployments :many
SELECT * FROM deployments
ORDER BY created_at DESC
LIMIT ?;

-- name: GetLatestDeploymentForApp :one
SELECT * FROM deployments
WHERE application_id = ? AND is_preview_deployment = 0
ORDER BY created_at DESC
LIMIT 1;

-- name: GetRunningDeploymentForApp :one
SELECT * FROM deployments
WHERE application_id = ? AND status = 'RUNNING'
ORDER BY created_at DESC
LIMIT 1;

-- name: CreateDeployment :one
INSERT INTO deployments (
    title, description, status, log_path,
    is_preview_deployment, started_at,
    application_id, compose_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateDeploymentStatus :exec
UPDATE deployments SET status = ? WHERE id = ?;

-- name: FinishDeployment :exec
UPDATE deployments
SET status = ?, finished_at = CURRENT_TIMESTAMP, error_message = ?
WHERE id = ?;

-- name: SetDeploymentPID :exec
UPDATE deployments SET pid = ? WHERE id = ?;

-- name: CancelDeployment :exec
UPDATE deployments
SET status = 'CANCELLED', finished_at = CURRENT_TIMESTAMP
WHERE id = ?;

-- name: DeleteDeployment :exec
DELETE FROM deployments WHERE id = ?;

-- name: DeleteOldDeployments :exec
DELETE FROM deployments
WHERE application_id = ? AND created_at < ?
  AND id NOT IN (
      SELECT id FROM deployments
      WHERE application_id = ?
      ORDER BY created_at DESC
      LIMIT ?
  );

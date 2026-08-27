-- Extracted from src/db/repository/rollbacks.rs. Do not edit repository SQL directly.

-- name: RollbacksGetAll :many
SELECT id AS id, deployment_id AS deployment_id, version AS version, image AS image, full_context AS full_context, created_at AS created_at FROM rollbacks;

-- name: RollbacksGetById :one
SELECT id AS id, deployment_id AS deployment_id, version AS version, image AS image, full_context AS full_context, created_at AS created_at FROM rollbacks WHERE id = ?;

-- name: RollbacksCreate :exec
INSERT INTO rollbacks (deployment_id, version, image, full_context, created_at) VALUES (?, ?, ?, ?, ?);

-- name: RollbacksUpdate :exec
UPDATE rollbacks SET deployment_id = ?, version = ?, image = ?, full_context = ?, created_at = ? WHERE id = ?;

-- name: RollbacksDelete :exec
DELETE FROM rollbacks WHERE id = ?;

-- name: RollbacksGetByDeploymentId :one
SELECT id AS id, deployment_id AS deployment_id, version AS version, image AS image, full_context AS full_context, created_at AS created_at FROM rollbacks WHERE deployment_id = ?;

-- name: RollbacksListByApplication :many
SELECT r.id AS id, r.deployment_id AS deployment_id, r.version AS version, r.image AS image, r.full_context AS full_context, r.created_at AS created_at
               FROM rollbacks r
               INNER JOIN deployments d ON d.id = r.deployment_id
               WHERE d.application_id = ?
               ORDER BY r.version DESC;

-- name: RollbacksGetNextVersionForApplication :one
SELECT COALESCE(MAX(r.version), 0) AS max_version
               FROM rollbacks r
               INNER JOIN deployments d ON d.id = r.deployment_id
               WHERE d.application_id = ?;

-- name: RollbacksGetForApplication :one
SELECT r.id AS id, r.deployment_id AS deployment_id,
                      r.version AS version, r.image AS image,
                      r.full_context AS full_context, r.created_at AS created_at
               FROM rollbacks r
               INNER JOIN deployments d ON d.id = r.deployment_id
               WHERE r.id = ? AND d.application_id = ?;

-- name: RollbacksDeleteForApplication :exec
DELETE FROM rollbacks
               WHERE id = ? AND deployment_id IN (
                   SELECT id FROM deployments WHERE application_id = ?
               );

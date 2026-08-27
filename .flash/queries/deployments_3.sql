-- Extracted from src/db/repository/deployments.rs. Do not edit repository SQL directly.

-- name: DeploymentsGetQueuedDeploymentsGrouped :many
SELECT id, server_id FROM deployments
             WHERE status = 'QUEUED'
             ORDER BY created_at ASC, id ASC;

-- name: DeploymentsClaimQueuedDeployment :one
UPDATE deployments
               SET status        = 'RUNNING',
                   state         = 'QUEUE',
                   started_at    = COALESCE(started_at, CURRENT_TIMESTAMP),
                   last_state_at = CURRENT_TIMESTAMP
               WHERE id = ? AND status = 'QUEUED'
               RETURNING id, application_id, compose_id, database_id, database_kind, operation;

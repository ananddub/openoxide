-- Extracted from src/db/repository/deployments.rs. Do not edit repository SQL directly.

-- name: DeploymentsRequestCancelDeployment :exec
UPDATE deployments SET status = 'CANCELLING', state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE application_id = ? AND status = 'RUNNING';

-- name: DeploymentsHasRunningComposeDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status IN ('QUEUED', 'RUNNING')) AS exists_val;

-- name: DeploymentsCreateQueuedComposeDeployment :exec
INSERT INTO deployments (title, description, status, state, log_path, operation, compose_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'));

-- name: DeploymentsHasRunningStatusComposeDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING') AS exists_val;

-- name: DeploymentsRequestCancelComposeDeployment :exec
UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE compose_id = ? AND status = 'RUNNING';

-- name: DeploymentsHasRunningDatabaseDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE database_id = ? AND status IN ('QUEUED', 'RUNNING')) AS exists_val;

-- name: DeploymentsCreateQueuedDatabaseDeployment :exec
INSERT INTO deployments (title, description, status, state, log_path, operation, database_id, database_kind, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, ?, strftime('%s', 'now'));

-- name: DeploymentsGetCancelInfo :one
SELECT application_id, compose_id, database_id, status FROM deployments WHERE id = ?;

-- name: DeploymentsCancelQueuedDeployment :exec
UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'QUEUED';

-- name: DeploymentsSetCancelRequested :exec
UPDATE deployments SET status = 'CANCELLING', state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'RUNNING';

-- name: DeploymentsListFiltered :many
SELECT id AS id, title AS title, description AS description, status AS status, state AS state, log_path AS log_path, pid AS pid, error_message AS error_message, operation AS operation, is_preview_deployment AS is_preview_deployment, started_at AS started_at, last_state_at AS last_state_at, finished_at AS finished_at, application_id AS application_id, compose_id AS compose_id, server_id AS server_id, created_at AS created_at, database_id AS database_id, database_kind AS database_kind
               FROM deployments
               WHERE (? IS NULL OR status = ?)
                 AND (? IS NULL OR state = ?)
                 AND (? IS NULL OR application_id = ?)
                 AND (? IS NULL OR compose_id = ?)
                 AND (? IS NULL OR database_id = ?)
                 AND (? IS NULL OR server_id = ?)
               ORDER BY COALESCE(started_at, created_at) DESC, id DESC
               LIMIT ? OFFSET ?;

-- name: DeploymentsGetStatus :one
SELECT status FROM deployments WHERE id = ?;

-- name: DeploymentsGetLatestApplicationDeploymentId :one
SELECT id AS id FROM deployments WHERE application_id = ? ORDER BY id DESC LIMIT 1;

-- name: DeploymentsGetLatestComposeDeploymentId :one
SELECT id AS id FROM deployments WHERE compose_id = ? ORDER BY id DESC LIMIT 1;

-- name: DeploymentsGetLatestDatabaseDeploymentId :one
SELECT id AS id FROM deployments WHERE database_id = ? ORDER BY id DESC LIMIT 1;

-- name: DeploymentsGetQueuedCount :one
SELECT COUNT(*) FROM deployments WHERE status = 'QUEUED';

-- name: DeploymentsGetQueuedIdsForApplication :many
SELECT id FROM deployments WHERE application_id = ? AND status = 'QUEUED';

-- name: DeploymentsCancelQueuedForApplication :exec
UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE application_id = ? AND status = 'QUEUED';

-- name: DeploymentsGetQueuedIdsForCompose :many
SELECT id FROM deployments WHERE compose_id = ? AND status = 'QUEUED';

-- name: DeploymentsCancelQueuedForCompose :exec
UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE compose_id = ? AND status = 'QUEUED';

-- name: DeploymentsGetQueuedIdsForDatabase :many
SELECT id FROM deployments WHERE database_id = ? AND status = 'QUEUED';

-- name: DeploymentsCancelQueuedForDatabase :exec
UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE database_id = ? AND status = 'QUEUED';

-- name: DeploymentsRequestCancelDatabaseDeployment :exec
UPDATE deployments SET status = 'CANCELLING', state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE database_id = ? AND status IN ('RUNNING', 'BUILDING', 'QUEUED');

-- name: DeploymentsMarkInterruptedAsRecovered :exec
UPDATE deployments
             SET status        = 'ERROR',
                 state         = 'RECOVERED_AFTER_RESTART',
                 error_message = COALESCE(error_message, 'server restarted before deployment completed'),
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE status IN ('QUEUED', 'RUNNING', 'CANCELLING');

-- name: DeploymentsRecoverStaleApplicationStatuses :exec
UPDATE applications SET app_status = 'ERROR'
             WHERE app_status IN ('QUEUED', 'STARTING', 'STOPPING', 'CANCELLING')
                OR EXISTS (
                 SELECT 1 FROM deployments d
                 WHERE d.application_id = applications.id
                   AND d.state = 'RECOVERED_AFTER_RESTART'
                   AND d.id = (
                       SELECT MAX(latest.id) FROM deployments latest
                       WHERE latest.application_id = applications.id
                   )
             );

-- name: DeploymentsRecoverStaleComposeStatuses :exec
UPDATE compose_projects SET compose_status = 'ERROR'
             WHERE compose_status IN ('QUEUED', 'STARTING', 'STOPPING', 'CANCELLING')
                OR EXISTS (
                 SELECT 1 FROM deployments d
                 WHERE d.compose_id = compose_projects.id
                   AND d.state = 'RECOVERED_AFTER_RESTART'
                   AND d.id = (
                       SELECT MAX(latest.id) FROM deployments latest
                       WHERE latest.compose_id = compose_projects.id
                   )
             );

-- name: DeploymentsGetRunningRemoteJobs :many
SELECT id, server_id, pid FROM deployments
             WHERE status = 'RUNNING' AND server_id IS NOT NULL AND pid IS NOT NULL;

-- name: DeploymentsGetRecoveredRemoteJobs :many
SELECT id, server_id, pid FROM deployments
             WHERE status = 'ERROR' AND state = 'RECOVERED_AFTER_RESTART'
             AND server_id IS NOT NULL AND pid IS NOT NULL;

-- name: DeploymentsSetPid :exec
UPDATE deployments SET pid = ? WHERE id = ?;

-- name: DeploymentsSetPidNull :exec
UPDATE deployments SET pid = NULL WHERE id = ?;

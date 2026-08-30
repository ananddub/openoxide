-- ============================================================
-- DEPLOYMENTS
-- ============================================================

-- name: GetDeploymentByID :one
SELECT * FROM deployments WHERE id = ?;

-- name: ListDeploymentsByApplication :many
SELECT * FROM deployments
WHERE service_id = ?
ORDER BY created_at DESC;

-- name: ListDeploymentsByCompose :many
SELECT * FROM deployments
WHERE service_id = ?
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
WHERE service_id = ? AND is_preview_deployment = 0
ORDER BY created_at DESC
LIMIT 1;

-- name: GetRunningDeploymentForApp :one
SELECT * FROM deployments
WHERE service_id = ? AND status = 'RUNNING'
ORDER BY created_at DESC
LIMIT 1;

-- name: CreateDeployment :one
INSERT INTO deployments (
    title, description, status, log_path,
    is_preview_deployment, started_at,
    service_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateDeploymentStatus :exec
UPDATE deployments SET status = ? WHERE id = ?;

-- name: FinishDeployment :exec
UPDATE deployments
SET status = ?, finished_at = unixepoch(), error_message = ?
WHERE id = ?;

-- name: SetDeploymentPID :exec
UPDATE deployments SET pid = ? WHERE id = ?;

-- name: CancelDeployment :exec
UPDATE deployments
SET status = 'CANCELLED', finished_at = unixepoch()
WHERE id = ?;

-- name: DeleteDeployment :exec
DELETE FROM deployments WHERE id = ?;

-- name: DeleteOldDeployments :exec
DELETE FROM deployments
WHERE service_id = ? AND created_at < ?
  AND id NOT IN (
      SELECT id FROM deployments
      WHERE service_id = ?
      ORDER BY created_at DESC
      LIMIT ?
  );

-- BEGIN GENERATED REPOSITORY QUERIES
-- Extracted from src/db/repository/deployments.rs. Do not edit this generated section directly.

-- name: DeploymentsRunningForApplication :one
SELECT id AS id, server_id AS server_id, pid AS pid
               FROM deployments
               WHERE service_id = ? AND status = 'RUNNING'
               ORDER BY id DESC LIMIT 1;

-- name: DeploymentsRunningForCompose :one
SELECT id AS id, server_id AS server_id, pid AS pid
               FROM deployments
               WHERE service_id = ? AND status = 'RUNNING'
               ORDER BY id DESC LIMIT 1;

-- name: DeploymentsCreateRunningDeployment :exec
INSERT INTO deployments
               (title, description, status, state, log_path, operation, service_id,
                server_id, started_at, last_state_at)
               VALUES (?, ?, 'RUNNING', 'RUNNING', '', ?, ?, ?, ?, unixepoch());

-- name: DeploymentsGetAll :many
SELECT id AS id, title AS title, description AS description, status AS status, state AS state, log_path AS log_path, pid AS pid, error_message AS error_message, operation AS operation, is_preview_deployment AS is_preview_deployment, started_at AS started_at, last_state_at AS last_state_at, finished_at AS finished_at, service_id AS service_id, server_id AS server_id, created_at AS created_at, database_kind AS database_kind FROM deployments;

-- name: DeploymentsGetById :one
SELECT id AS id, title AS title, description AS description, status AS status, state AS state, log_path AS log_path, pid AS pid, error_message AS error_message, operation AS operation, is_preview_deployment AS is_preview_deployment, started_at AS started_at, last_state_at AS last_state_at, finished_at AS finished_at, service_id AS service_id, server_id AS server_id, created_at AS created_at, database_kind AS database_kind FROM deployments WHERE id = ?;

-- name: DeploymentsCreate :exec
INSERT INTO deployments (title, description, status, state, log_path, pid, error_message, operation, is_preview_deployment, started_at, last_state_at, finished_at, service_id, server_id, created_at, database_kind) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: DeploymentsUpdate :exec
UPDATE deployments SET title = ?, description = ?, status = ?, state = ?, log_path = ?, pid = ?, error_message = ?, operation = ?, is_preview_deployment = ?, started_at = ?, last_state_at = ?, finished_at = ?, service_id = ?, server_id = ?, created_at = ?, database_kind = ? WHERE id = ?;

-- name: DeploymentsDelete :exec
DELETE FROM deployments WHERE id = ?;

-- name: DeploymentsHasRunningDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE service_id = ? AND status IN ('QUEUED', 'RUNNING')) AS exists_val;

-- name: DeploymentsListFinishedIdsForApplication :many
SELECT id AS id FROM deployments
               WHERE service_id = ? AND status NOT IN ('QUEUED', 'RUNNING')
               ORDER BY id;

-- name: DeploymentsDeleteFinishedForApplication :exec
DELETE FROM deployments
               WHERE service_id = ? AND status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsCancelQueuedForApplicationCount :exec
UPDATE deployments
               SET status = 'CANCELLED', state = 'CANCELLED',
                   finished_at = unixepoch(),
                   last_state_at = unixepoch()
               WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsListFinishedIdsForCompose :many
SELECT id AS id FROM deployments WHERE service_id = ? AND status NOT IN ('QUEUED', 'RUNNING') ORDER BY id;

-- name: DeploymentsDeleteFinishedForCompose :exec
DELETE FROM deployments WHERE service_id = ? AND status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsDeleteFilteredFinished1 :many
SELECT log_path FROM deployments WHERE status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsDeleteFilteredFinished2 :exec
DELETE FROM deployments WHERE status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsCancelQueuedForComposeCount :exec
UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = unixepoch(), last_state_at = unixepoch() WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsCreateQueuedDeployment :exec
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, unixepoch());

-- name: DeploymentsUpdateLogPath :exec
UPDATE deployments SET log_path = ? WHERE id = ?;

-- name: DeploymentsUpdateStateIfRunning :exec
UPDATE deployments
             SET state         = ?,
                 last_state_at = unixepoch()
             WHERE id = ? AND status = 'RUNNING';

-- name: DeploymentsUpdateStateIfRunningWithError :exec
UPDATE deployments
             SET state         = ?,
                 error_message = ?,
                 last_state_at = unixepoch()
             WHERE id = ? AND status = 'RUNNING';

-- name: DeploymentsUpdateFinalStatus :exec
UPDATE deployments
             SET status        = ?,
                 state         = ?,
                 error_message = ?,
                 finished_at   = unixepoch(),
                 last_state_at = unixepoch()
             WHERE id = ?;

-- name: DeploymentsFinalizeWithResource1 :exec
UPDATE deployments SET status=?, state=?, error_message=?, finished_at=unixepoch(), last_state_at=unixepoch(), pid=NULL WHERE id=?;


-- name: DeploymentsFinalizeWithResource3 :exec
UPDATE services SET app_status=?, last_error=? WHERE id=?;

-- name: DeploymentsFinalizeWithResource4 :exec
UPDATE services SET app_status=?, last_error=? WHERE id=?;

-- name: DeploymentsFinalizeWithResource6 :exec
UPDATE preview_deployments
               SET status = 'ERROR',
                   updated_at = unixepoch()
               WHERE last_deployment_id = ? AND status != 'CLOSED';

-- name: DeploymentsFinalizePreviewDone :exec
UPDATE preview_deployments
               SET status = 'ACTIVE',
                   updated_at = unixepoch()
               WHERE last_deployment_id = ? AND status != 'CLOSED';

-- name: DeploymentsSetApplicationStatus :exec
UPDATE services SET app_status = ?, last_error = NULL WHERE id = ?;

-- name: DeploymentsSetComposeStatus :exec
UPDATE services SET app_status = ?, last_error = NULL WHERE id = ?;



-- name: DeploymentsApplicationOrganizationId :one
SELECT p.organization_id AS organization_id
               FROM services s
               JOIN environments e ON e.id = s.environment_id
               JOIN projects p ON p.id = e.project_id
               WHERE s.id = ?;

-- name: DeploymentsComposeOrganizationId :one
SELECT p.organization_id AS organization_id
               FROM services s
               JOIN environments e ON e.id = s.environment_id
               JOIN projects p ON p.id = e.project_id
               WHERE s.id = ?;


-- name: DeploymentsHasRunningStatusDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE service_id = ? AND status = 'RUNNING') AS exists_val;

-- name: DeploymentsRequestCancelDeployment :exec
UPDATE deployments SET status = 'CANCELLING', state = 'CANCEL_REQUESTED', last_state_at = unixepoch() WHERE service_id = ? AND status = 'RUNNING';

-- name: DeploymentsHasRunningComposeDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE service_id = ? AND status IN ('QUEUED', 'RUNNING')) AS exists_val;

-- name: DeploymentsCreateQueuedComposeDeployment :exec
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, unixepoch());

-- name: DeploymentsHasRunningStatusComposeDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE service_id = ? AND status = 'RUNNING') AS exists_val;

-- name: DeploymentsRequestCancelComposeDeployment :exec
UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = unixepoch() WHERE service_id = ? AND status = 'RUNNING';

-- name: DeploymentsHasRunningDatabaseDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE service_id = ? AND status IN ('QUEUED', 'RUNNING')) AS exists_val;

-- name: DeploymentsCreateQueuedDatabaseDeployment :exec
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, database_kind, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, ?, unixepoch());

-- name: DeploymentsGetCancelInfo :one
SELECT service_id, status FROM deployments WHERE id = ?;

-- name: DeploymentsCancelQueuedDeployment :exec
UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = unixepoch(), last_state_at = unixepoch() WHERE id = ? AND status = 'QUEUED';

-- name: DeploymentsSetCancelRequested :exec
UPDATE deployments SET status = 'CANCELLING', state = 'CANCEL_REQUESTED', last_state_at = unixepoch() WHERE id = ? AND status = 'RUNNING';

-- name: DeploymentsListFiltered :many
SELECT id AS id, title AS title, description AS description, status AS status, state AS state, log_path AS log_path, pid AS pid, error_message AS error_message, operation AS operation, is_preview_deployment AS is_preview_deployment, started_at AS started_at, last_state_at AS last_state_at, finished_at AS finished_at, service_id AS service_id, server_id AS server_id, created_at AS created_at, database_kind AS database_kind
               FROM deployments
               WHERE status = COALESCE(?, status)
                 AND state = COALESCE(?, state)
                 AND service_id = COALESCE(?, service_id)
                 AND server_id = COALESCE(?, server_id)
               ORDER BY COALESCE(started_at, created_at) DESC, id DESC
               LIMIT ? OFFSET ?;

-- name: DeploymentsGetStatus :one
SELECT status FROM deployments WHERE id = ?;

-- name: DeploymentsGetLatestApplicationDeploymentId :one
SELECT id AS id FROM deployments WHERE service_id = ? ORDER BY id DESC LIMIT 1;

-- name: DeploymentsGetLatestComposeDeploymentId :one
SELECT id AS id FROM deployments WHERE service_id = ? ORDER BY id DESC LIMIT 1;

-- name: DeploymentsGetLatestDatabaseDeploymentId :one
SELECT id AS id FROM deployments WHERE service_id = ? ORDER BY id DESC LIMIT 1;

-- name: DeploymentsGetQueuedCount :one
SELECT COUNT(*) FROM deployments WHERE status = 'QUEUED';

-- name: DeploymentsGetQueuedIdsForApplication :many
SELECT id FROM deployments WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsCancelQueuedForApplication :exec
UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = unixepoch(),
                 last_state_at = unixepoch()
             WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsGetQueuedIdsForCompose :many
SELECT id FROM deployments WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsCancelQueuedForCompose :exec
UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = unixepoch(),
                 last_state_at = unixepoch()
             WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsGetQueuedIdsForDatabase :many
SELECT id FROM deployments WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsCancelQueuedForDatabase :exec
UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = unixepoch(),
                 last_state_at = unixepoch()
             WHERE service_id = ? AND status = 'QUEUED';

-- name: DeploymentsRequestCancelDatabaseDeployment :exec
UPDATE deployments SET status = 'CANCELLING', state = 'CANCEL_REQUESTED', last_state_at = unixepoch() WHERE service_id = ? AND status IN ('RUNNING', 'BUILDING', 'QUEUED');

-- name: DeploymentsMarkInterruptedAsRecovered :exec
UPDATE deployments
             SET status        = 'ERROR',
                 state         = 'RECOVERED_AFTER_RESTART',
                 error_message = COALESCE(error_message, 'server restarted before deployment completed'),
                 finished_at   = unixepoch(),
                 last_state_at = unixepoch()
             WHERE status IN ('QUEUED', 'RUNNING', 'CANCELLING');

-- name: DeploymentsRecoverStaleApplicationStatuses :exec
UPDATE services SET app_status = 'ERROR'
             WHERE app_status IN ('QUEUED', 'STARTING', 'STOPPING', 'CANCELLING')
                OR EXISTS (
                 SELECT 1 FROM deployments d
                 WHERE d.service_id = services.id
                   AND d.state = 'RECOVERED_AFTER_RESTART'
                   AND d.id = (
                       SELECT MAX(latest.id) FROM deployments latest
                       WHERE latest.service_id = services.id
                   )
             );

-- name: DeploymentsRecoverStaleComposeStatuses :exec
UPDATE services SET app_status = 'ERROR'
             WHERE app_status IN ('QUEUED', 'STARTING', 'STOPPING', 'CANCELLING')
                OR EXISTS (
                 SELECT 1 FROM deployments d
                 WHERE d.service_id = services.id
                   AND d.state = 'RECOVERED_AFTER_RESTART'
                   AND d.id = (
                       SELECT MAX(latest.id) FROM deployments latest
                       WHERE latest.service_id = services.id
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

-- name: DeploymentsGetQueuedDeploymentsGrouped :many
SELECT id, server_id FROM deployments
             WHERE status = 'QUEUED'
             ORDER BY created_at ASC, id ASC;

-- name: DeploymentsClaimQueuedDeployment :one
UPDATE deployments
               SET status        = 'RUNNING',
                   state         = 'QUEUE',
                   started_at    = COALESCE(started_at, unixepoch()),
                   last_state_at = unixepoch()
               WHERE id = ? AND status = 'QUEUED'
               RETURNING id, service_id, database_kind, operation;

-- name: DeploymentsClaimServiceType :one
SELECT service_type FROM services WHERE id = ?;

-- APPLICATION core helper (src/services/application/operations.rs).
-- Old DeploymentRepository::create_queued_deployment returned the new row id;
-- the generated DeploymentsCreateQueuedDeployment is :exec, so this variant
-- returns the id. Split With/NoServer keeps a NULL server_id insertable.

-- name: AppCoreQueueDeploymentWithServer :one
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, server_id, last_state_at)
VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, unixepoch())
RETURNING id;

-- name: AppCoreQueueDeploymentNoServer :one
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, last_state_at)
VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, unixepoch())
RETURNING id;

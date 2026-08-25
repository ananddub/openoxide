-- Extracted from src/db/repository/deployments.rs. Do not edit repository SQL directly.

-- name: DeploymentsRunningForApplication :one
SELECT id AS id, server_id AS server_id, pid AS pid
               FROM deployments
               WHERE application_id = ? AND status = 'RUNNING'
               ORDER BY id DESC LIMIT 1;

-- name: DeploymentsRunningForCompose :one
SELECT id AS id, server_id AS server_id, pid AS pid
               FROM deployments
               WHERE compose_id = ? AND status = 'RUNNING'
               ORDER BY id DESC LIMIT 1;

-- name: DeploymentsCreateRunningDeployment :exec
INSERT INTO deployments
               (title, description, status, state, log_path, operation, application_id,
                server_id, started_at, last_state_at)
               VALUES (?, ?, 'RUNNING', 'RUNNING', '', ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'));

-- name: DeploymentsGetAll :many
SELECT id AS id, title AS title, description AS description, status AS status, state AS state, log_path AS log_path, pid AS pid, error_message AS error_message, operation AS operation, is_preview_deployment AS is_preview_deployment, started_at AS started_at, last_state_at AS last_state_at, finished_at AS finished_at, application_id AS application_id, compose_id AS compose_id, server_id AS server_id, created_at AS created_at, database_id AS database_id, database_kind AS database_kind FROM deployments;

-- name: DeploymentsGetById :one
SELECT id AS id, title AS title, description AS description, status AS status, state AS state, log_path AS log_path, pid AS pid, error_message AS error_message, operation AS operation, is_preview_deployment AS is_preview_deployment, started_at AS started_at, last_state_at AS last_state_at, finished_at AS finished_at, application_id AS application_id, compose_id AS compose_id, server_id AS server_id, created_at AS created_at, database_id AS database_id, database_kind AS database_kind FROM deployments WHERE id = ?;

-- name: DeploymentsCreate :exec
INSERT INTO deployments (title, description, status, state, log_path, pid, error_message, operation, is_preview_deployment, started_at, last_state_at, finished_at, application_id, compose_id, server_id, created_at, database_id, database_kind) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: DeploymentsUpdate :exec
UPDATE deployments SET title = ?, description = ?, status = ?, state = ?, log_path = ?, pid = ?, error_message = ?, operation = ?, is_preview_deployment = ?, started_at = ?, last_state_at = ?, finished_at = ?, application_id = ?, compose_id = ?, server_id = ?, created_at = ?, database_id = ?, database_kind = ? WHERE id = ?;

-- name: DeploymentsDelete :exec
DELETE FROM deployments WHERE id = ?;

-- name: DeploymentsHasRunningDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status IN ('QUEUED', 'RUNNING')) AS exists_val;

-- name: DeploymentsListFinishedIdsForApplication :many
SELECT id AS id FROM deployments
               WHERE application_id = ? AND status NOT IN ('QUEUED', 'RUNNING')
               ORDER BY id;

-- name: DeploymentsDeleteFinishedForApplication :exec
DELETE FROM deployments
               WHERE application_id = ? AND status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsCancelQueuedForApplicationCount :exec
UPDATE deployments
               SET status = 'CANCELLED', state = 'CANCELLED',
                   finished_at = strftime('%s', 'now'),
                   last_state_at = strftime('%s', 'now')
               WHERE application_id = ? AND status = 'QUEUED';

-- name: DeploymentsListFinishedIdsForCompose :many
SELECT id AS id FROM deployments WHERE compose_id = ? AND status NOT IN ('QUEUED', 'RUNNING') ORDER BY id;

-- name: DeploymentsDeleteFinishedForCompose :exec
DELETE FROM deployments WHERE compose_id = ? AND status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsDeleteFilteredFinished1 :many
SELECT log_path FROM deployments WHERE status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsDeleteFilteredFinished2 :exec
DELETE FROM deployments WHERE status NOT IN ('QUEUED', 'RUNNING');

-- name: DeploymentsCancelQueuedForComposeCount :exec
UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE compose_id = ? AND status = 'QUEUED';

-- name: DeploymentsCreateQueuedDeployment :exec
INSERT INTO deployments (title, description, status, state, log_path, operation, application_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'));

-- name: DeploymentsUpdateLogPath :exec
UPDATE deployments SET log_path = ? WHERE id = ?;

-- name: DeploymentsUpdateStateIfRunning :exec
UPDATE deployments
             SET state         = ?,
                 error_message = COALESCE(?, error_message),
                 last_state_at = strftime('%s', 'now')
             WHERE id = ? AND status = 'RUNNING';

-- name: DeploymentsUpdateFinalStatus :exec
UPDATE deployments
             SET status        = ?,
                 state         = ?,
                 error_message = COALESCE(?, error_message),
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE id = ?;

-- name: DeploymentsFinalizeWithResource1 :exec
UPDATE deployments SET status=?, state=?, error_message=COALESCE(?, error_message), finished_at=strftime('%s','now'), last_state_at=strftime('%s','now'), pid=NULL WHERE id=?;

-- name: DeploymentsFinalizeWithResource2 :exec
UPDATE deployments SET logs = COALESCE(logs, '') || ? WHERE id = ?;

-- name: DeploymentsFinalizeWithResource3 :exec
UPDATE applications SET app_status=?, last_error=? WHERE id=?;

-- name: DeploymentsFinalizeWithResource4 :exec
UPDATE compose_projects SET compose_status=?, last_error=? WHERE id=?;

-- name: DeploymentsFinalizeWithResource6 :exec
UPDATE preview_deployments
               SET status = CASE WHEN ? = 'DONE' THEN 'ACTIVE' ELSE 'ERROR' END,
                   updated_at = strftime('%s', 'now')
               WHERE last_deployment_id = ? AND status != 'CLOSED';

-- name: DeploymentsSetApplicationStatus :exec
UPDATE applications SET app_status = ?, last_error = NULL WHERE id = ?;

-- name: DeploymentsSetComposeStatus :exec
UPDATE compose_projects SET compose_status = ?, last_error = NULL WHERE id = ?;

-- name: DeploymentsUpdateDatabaseStatus :exec
UPDATE {} SET app_status = ? WHERE id = ?;

-- name: DeploymentsGetDatabaseDeploymentContext :one
SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
             FROM {} d
             JOIN environments e ON e.id = d.environment_id
             WHERE d.id = ?;

-- name: DeploymentsApplicationOrganizationId :one
SELECT p.organization_id AS organization_id
               FROM applications a
               JOIN environments e ON e.id = a.environment_id
               JOIN projects p ON p.id = e.project_id
               WHERE a.id = ?;

-- name: DeploymentsComposeOrganizationId :one
SELECT p.organization_id AS organization_id
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               JOIN projects p ON p.id = e.project_id
               WHERE c.id = ?;

-- name: DeploymentsGetDatabaseAppName :one
SELECT app_name FROM {} WHERE id = ?;

-- name: DeploymentsHasRunningStatusDeployment :one
SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status = 'RUNNING') AS exists_val;

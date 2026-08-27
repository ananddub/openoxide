-- Extracted from src/db/repository/schedule_runtime.rs. Do not edit repository SQL directly.

-- name: ScheduleRuntimePolicy1 :exec
INSERT INTO schedule_runtime_policies (schedule_id) VALUES (?) ON CONFLICT(schedule_id) DO NOTHING;

-- name: ScheduleRuntimePolicy2 :one
SELECT schedule_id AS schedule_id, retry_count AS retry_count, retry_delay_seconds AS retry_delay_seconds, missed_run_policy AS missed_run_policy, concurrency_policy AS concurrency_policy, lease_seconds AS lease_seconds, notify_on_success AS notify_on_success, notify_on_failure AS notify_on_failure, last_scheduled_at, updated_at AS updated_at FROM schedule_runtime_policies WHERE schedule_id=?;

-- name: ScheduleRuntimeUpdatePolicy :exec
INSERT INTO schedule_runtime_policies (schedule_id, retry_count, retry_delay_seconds, missed_run_policy, concurrency_policy, lease_seconds, notify_on_success, notify_on_failure, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(schedule_id) DO UPDATE SET retry_count=excluded.retry_count, retry_delay_seconds=excluded.retry_delay_seconds, missed_run_policy=excluded.missed_run_policy, concurrency_policy=excluded.concurrency_policy, lease_seconds=excluded.lease_seconds, notify_on_success=excluded.notify_on_success, notify_on_failure=excluded.notify_on_failure, updated_at=excluded.updated_at;

-- name: ScheduleRuntimeAcquire :exec
INSERT INTO schedule_execution_locks (schedule_id, owner_id, acquired_at, expires_at) VALUES (?, ?, ?, ?) ON CONFLICT(schedule_id) DO UPDATE SET owner_id=excluded.owner_id, acquired_at=excluded.acquired_at, expires_at=excluded.expires_at WHERE schedule_execution_locks.expires_at <= ?;

-- name: ScheduleRuntimeRelease :exec
DELETE FROM schedule_execution_locks WHERE schedule_id=? AND owner_id=?;

-- name: ScheduleRuntimeMarkScheduled :exec
UPDATE schedule_runtime_policies SET last_scheduled_at=?, updated_at=CURRENT_TIMESTAMP WHERE schedule_id=?;

-- name: ScheduleRuntimeBegin :exec
INSERT INTO schedule_executions (schedule_id, organization_id, trigger_kind, status, attempt, scheduled_at) VALUES (?, ?, ?, 'RUNNING', ?, ?);

-- name: ScheduleRuntimeFinish1 :one
SELECT schedule_id FROM schedule_executions WHERE id=?;

-- name: ScheduleRuntimeFinish2 :exec
UPDATE schedule_executions SET status=?, message=?, stdout=?, stderr=?, finished_at=CURRENT_TIMESTAMP WHERE id=?;

-- name: ScheduleRuntimeList :many
SELECT id AS id, schedule_id AS schedule_id, organization_id, trigger_kind AS trigger_kind, status AS status, attempt AS attempt, scheduled_at AS scheduled_at, started_at AS started_at, finished_at, message, stdout, stderr FROM schedule_executions WHERE schedule_id=? ORDER BY started_at DESC LIMIT ?;

-- name: ScheduleRuntimeExecutionExists :one
SELECT COUNT(*) FROM schedule_executions WHERE id=? AND schedule_id=?;

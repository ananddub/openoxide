-- Schedule runtime: policies, execution locks, execution records.

-- name: ScheduleRuntimeEnsurePolicy :exec
-- The policy table has no column defaults, so the lazy-ensure row must bind
-- neutral values itself: no retries, 60s backoff, skip missed runs, skip
-- overlapping runs, 300s lease, failures-only notifications. Strings match
-- the MissedRunPolicy/ConcurrencyPolicy enums (read path expects them).
INSERT INTO schedule_runtime_policies (
    schedule_id, retry_count, retry_delay_seconds, missed_run_policy,
    concurrency_policy, lease_seconds, notify_on_success, notify_on_failure,
    updated_at
) VALUES (?, 0, 60, 'SKIP', 'SKIP', 300, 0, 1, unixepoch())
ON CONFLICT(schedule_id) DO NOTHING;

-- name: ScheduleRuntimeGetPolicy :one
SELECT schedule_id AS schedule_id, retry_count AS retry_count, retry_delay_seconds AS retry_delay_seconds, missed_run_policy AS missed_run_policy, concurrency_policy AS concurrency_policy, lease_seconds AS lease_seconds, notify_on_success AS notify_on_success, notify_on_failure AS notify_on_failure, last_scheduled_at, updated_at AS updated_at FROM schedule_runtime_policies WHERE schedule_id=?;

-- name: ScheduleRuntimeUpdatePolicy :exec
INSERT INTO schedule_runtime_policies (schedule_id, retry_count, retry_delay_seconds, missed_run_policy, concurrency_policy, lease_seconds, notify_on_success, notify_on_failure, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, unixepoch()) ON CONFLICT(schedule_id) DO UPDATE SET retry_count=excluded.retry_count, retry_delay_seconds=excluded.retry_delay_seconds, missed_run_policy=excluded.missed_run_policy, concurrency_policy=excluded.concurrency_policy, lease_seconds=excluded.lease_seconds, notify_on_success=excluded.notify_on_success, notify_on_failure=excluded.notify_on_failure, updated_at=excluded.updated_at;

-- name: ScheduleRuntimeAcquire :one
INSERT INTO schedule_execution_locks (schedule_id, owner_id, acquired_at, expires_at) VALUES (?, ?, ?, ?) ON CONFLICT(schedule_id) DO UPDATE SET owner_id=excluded.owner_id, acquired_at=excluded.acquired_at, expires_at=excluded.expires_at WHERE schedule_execution_locks.expires_at <= ? RETURNING schedule_id;

-- name: ScheduleRuntimeRelease :exec
DELETE FROM schedule_execution_locks WHERE schedule_id=? AND owner_id=?;

-- name: ScheduleRuntimeMarkScheduled :exec
UPDATE schedule_runtime_policies SET last_scheduled_at=?, updated_at=unixepoch() WHERE schedule_id=?;

-- name: ScheduleRuntimeBegin :one
-- The row is created in RUNNING state, so started_at is set at insert time.
INSERT INTO schedule_executions (schedule_id, organization_id, trigger_kind, status, attempt, scheduled_at, started_at) VALUES (?, ?, ?, 'RUNNING', ?, ?, unixepoch()) RETURNING id;

-- name: ScheduleRuntimeGetExecutionSchedule :one
SELECT schedule_id FROM schedule_executions WHERE id=?;

-- name: ScheduleRuntimeFinish :exec
UPDATE schedule_executions SET status=?, message=?, stdout=?, stderr=?, finished_at=unixepoch() WHERE id=?;

-- name: ScheduleRuntimeList :many
SELECT id AS id, schedule_id AS schedule_id, organization_id, trigger_kind AS trigger_kind, status AS status, attempt AS attempt, scheduled_at AS scheduled_at, started_at AS started_at, finished_at, message, stdout, stderr FROM schedule_executions WHERE schedule_id=? ORDER BY started_at DESC LIMIT ?;

-- name: ScheduleRuntimeExecutionExists :one
SELECT COUNT(*) FROM schedule_executions WHERE id=? AND schedule_id=?;

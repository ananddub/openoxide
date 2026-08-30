-- ============================================================
-- SCHEDULES
-- ============================================================

-- name: GetScheduleByID :one
SELECT * FROM schedules WHERE id = ?;

-- name: GetScheduleByAppName :one
SELECT * FROM schedules WHERE app_name = ?;

-- name: ListSchedulesByOrg :many
SELECT * FROM schedules WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListSchedulesByApplication :many
SELECT * FROM schedules WHERE service_id = ? ORDER BY created_at DESC;

-- name: ListSchedulesByCompose :many
SELECT * FROM schedules WHERE service_id = ? ORDER BY created_at DESC;

-- name: ListSchedulesByServer :many
SELECT * FROM schedules WHERE server_id = ? ORDER BY created_at DESC;

-- name: ListEnabledSchedules :many
SELECT * FROM schedules WHERE enabled = 1 ORDER BY created_at DESC;

-- name: CreateSchedule :one
INSERT INTO schedules (
    name, description, cron_expression, app_name,
    service_name, shell_type, schedule_type, schedule_action, command, script,
    timezone, enabled,
    service_id, server_id, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateSchedule :one
UPDATE schedules
SET
    name            = ?,
    description     = ?,
    cron_expression = ?,
    service_name    = ?,
    shell_type      = ?,
    schedule_action = ?,
    command         = ?,
    script          = ?,
    timezone        = ?
WHERE id = ?
RETURNING *;

-- name: ToggleSchedule :exec
UPDATE schedules SET enabled = ? WHERE id = ?;

-- name: DeleteSchedule :exec
DELETE FROM schedules WHERE id = ?;


-- name: SchedulesGetAll :many
SELECT id AS id, name AS name, description AS description, cron_expression AS cron_expression, app_name AS app_name, service_name AS service_name, shell_type AS shell_type, schedule_type AS schedule_type, command AS command, script AS script, timezone AS timezone, enabled AS enabled, service_id AS service_id, server_id AS server_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, schedule_action AS schedule_action FROM schedules;

-- name: SchedulesGetById :one
SELECT id AS id, name AS name, description AS description, cron_expression AS cron_expression, app_name AS app_name, service_name AS service_name, shell_type AS shell_type, schedule_type AS schedule_type, command AS command, script AS script, timezone AS timezone, enabled AS enabled, service_id AS service_id, server_id AS server_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, schedule_action AS schedule_action FROM schedules WHERE id = ?;

-- name: SchedulesCreate :exec
INSERT INTO schedules (name, description, cron_expression, app_name, service_name, shell_type, schedule_type, command, script, timezone, enabled, service_id, server_id, organization_id, created_at, updated_at, schedule_action) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: SchedulesUpdate :exec
UPDATE schedules SET name = ?, description = ?, cron_expression = ?, app_name = ?, service_name = ?, shell_type = ?, schedule_type = ?, command = ?, script = ?, timezone = ?, enabled = ?, service_id = ?, server_id = ?, organization_id = ?, created_at = ?, updated_at = ?, schedule_action = ? WHERE id = ?;

-- name: SchedulesDelete :exec
DELETE FROM schedules WHERE id = ?;

-- name: SchedulesListByApplication :many
SELECT id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at
               FROM schedules WHERE service_id = ?
               ORDER BY created_at DESC, id DESC;

-- name: SchedulesListByCompose :many
SELECT id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at
               FROM schedules WHERE service_id = ?
               ORDER BY created_at DESC, id DESC;

-- name: SchedulesListByServer :many
SELECT id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at
               FROM schedules WHERE server_id = ?
               ORDER BY created_at DESC, id DESC;

-- name: SchedulesListByOrganization :many
SELECT id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at
               FROM schedules WHERE organization_id = ?
               ORDER BY created_at DESC, id DESC;

-- name: SchedulesListEnabled :many
SELECT id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at
               FROM schedules WHERE enabled = 1
               ORDER BY created_at DESC, id DESC;

-- name: SchedulesCreateAndReturn :one
INSERT INTO schedules
               (name, description, cron_expression, app_name, service_name, shell_type,
                schedule_type, schedule_action, command, script, timezone, enabled, service_id,
                server_id, organization_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at;

-- name: SchedulesUpdateAndReturn :one
UPDATE schedules SET
               name = ?, description = ?, cron_expression = ?, app_name = ?, service_name = ?,
               shell_type = ?, schedule_type = ?, schedule_action = ?, command = ?, script = ?, timezone = ?,
               enabled = ?, service_id = ?, server_id = ?, organization_id = ?
               WHERE id = ?
               RETURNING id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at;

-- name: SchedulesSetEnabled :one
UPDATE schedules SET enabled = ? WHERE id = ?
               RETURNING id AS id, name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, service_id,
               server_id, organization_id, created_at, updated_at;

-- ============================================================
-- DESTINATIONS (Storage)
-- ============================================================

-- name: GetDestinationByID :one
SELECT * FROM destinations WHERE id = ?;

-- name: ListDestinationsByOrg :many
SELECT * FROM destinations WHERE organization_id = ? ORDER BY created_at DESC;

-- name: CreateDestination :one
INSERT INTO destinations (
    name, provider, access_key, secret_access_key,
    bucket, region, endpoint, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateDestination :one
UPDATE destinations
SET
    name              = ?,
    provider          = ?,
    access_key        = ?,
    secret_access_key = ?,
    bucket            = ?,
    region            = ?,
    endpoint          = ?
WHERE id = ?
RETURNING *;

-- name: DeleteDestination :exec
DELETE FROM destinations WHERE id = ?;

-- ============================================================
-- BACKUPS (Database Backup Jobs)
-- ============================================================

-- name: GetBackupByID :one
SELECT * FROM backups WHERE id = ?;

-- name: GetBackupByAppName :one
SELECT * FROM backups WHERE app_name = ?;

-- name: ListBackupsByOrg :many
SELECT * FROM backups WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListBackupsByDestination :many
SELECT * FROM backups WHERE destination_id = ? ORDER BY created_at DESC;

-- name: ListBackupsByType :many
SELECT * FROM backups WHERE backup_type = ? AND organization_id = ? ORDER BY created_at DESC;

-- name: CreateBackup :one
INSERT INTO backups (
    app_name, schedule, enabled, database_name, prefix,
    service_name, keep_latest_count, backup_type, database_type, metadata,
    service_id,
    destination_id, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateBackup :one
UPDATE backups
SET
    schedule          = ?,
    enabled           = ?,
    database_name     = ?,
    prefix            = ?,
    service_name      = ?,
    keep_latest_count = ?,
    metadata          = ?
WHERE id = ?
RETURNING *;

-- name: ToggleBackup :exec
UPDATE backups SET enabled = ? WHERE id = ?;

-- name: DeleteBackup :exec
DELETE FROM backups WHERE id = ?;

-- ============================================================
-- VOLUME BACKUPS
-- ============================================================

-- name: GetVolumeBackupByID :one
SELECT * FROM volume_backups WHERE id = ?;

-- name: GetVolumeBackupByAppName :one
SELECT * FROM volume_backups WHERE app_name = ?;

-- name: ListVolumeBackupsByOrg :many
SELECT * FROM volume_backups WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListVolumeBackupsByApplication :many
SELECT * FROM volume_backups WHERE service_id = ? ORDER BY created_at DESC;

-- name: CreateVolumeBackup :one
INSERT INTO volume_backups (
    name, volume_name, prefix, service_type, app_name,
    service_name, turn_off, cron_expression, keep_latest_count, enabled,
    destination_id, organization_id,
    service_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateVolumeBackup :one
UPDATE volume_backups
SET
    name              = ?,
    volume_name       = ?,
    prefix            = ?,
    service_name      = ?,
    turn_off          = ?,
    cron_expression   = ?,
    keep_latest_count = ?,
    enabled           = ?
WHERE id = ?
RETURNING *;

-- name: ToggleVolumeBackup :exec
UPDATE volume_backups SET enabled = ? WHERE id = ?;

-- name: DeleteVolumeBackup :exec
DELETE FROM volume_backups WHERE id = ?;


-- name: BackupsGetAll :many
SELECT id AS id, app_name AS app_name, schedule AS schedule, enabled AS enabled, database_name AS database_name, prefix AS prefix, service_name AS service_name, keep_latest_count AS keep_latest_count, backup_type AS backup_type, database_type AS database_type, metadata AS metadata, service_id AS service_id, destination_id AS destination_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM backups;

-- name: BackupsGetById :one
SELECT id AS id, app_name AS app_name, schedule AS schedule, enabled AS enabled, database_name AS database_name, prefix AS prefix, service_name AS service_name, keep_latest_count AS keep_latest_count, backup_type AS backup_type, database_type AS database_type, metadata AS metadata, service_id AS service_id, destination_id AS destination_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM backups WHERE id = ?;

-- name: BackupsCreate :exec
INSERT INTO backups (app_name, schedule, enabled, database_name, prefix, service_name, keep_latest_count, backup_type, database_type, metadata, service_id, destination_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: BackupsUpdate :exec
UPDATE backups SET app_name = ?, schedule = ?, enabled = ?, database_name = ?, prefix = ?, service_name = ?, keep_latest_count = ?, backup_type = ?, database_type = ?, metadata = ?, service_id = ?, destination_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: BackupsDelete :exec
DELETE FROM backups WHERE id = ?;

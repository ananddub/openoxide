-- Extracted from src/db/repository/volume_backups.rs. Do not edit repository SQL directly.

-- name: VolumeBackupsGetAll :many
SELECT id AS id, name AS name, volume_name AS volume_name, prefix AS prefix, service_type AS service_type, app_name AS app_name, service_name AS service_name, turn_off AS turn_off, cron_expression AS cron_expression, keep_latest_count AS keep_latest_count, enabled AS enabled, destination_id AS destination_id, organization_id AS organization_id, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM volume_backups;

-- name: VolumeBackupsGetById :one
SELECT id AS id, name AS name, volume_name AS volume_name, prefix AS prefix, service_type AS service_type, app_name AS app_name, service_name AS service_name, turn_off AS turn_off, cron_expression AS cron_expression, keep_latest_count AS keep_latest_count, enabled AS enabled, destination_id AS destination_id, organization_id AS organization_id, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM volume_backups WHERE id = ?;

-- name: VolumeBackupsCreate :exec
INSERT INTO volume_backups (name, volume_name, prefix, service_type, app_name, service_name, turn_off, cron_expression, keep_latest_count, enabled, destination_id, organization_id, service_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: VolumeBackupsUpdate :exec
UPDATE volume_backups SET name = ?, volume_name = ?, prefix = ?, service_type = ?, app_name = ?, service_name = ?, turn_off = ?, cron_expression = ?, keep_latest_count = ?, enabled = ?, destination_id = ?, organization_id = ?, service_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: VolumeBackupsDelete :exec
DELETE FROM volume_backups WHERE id = ?;

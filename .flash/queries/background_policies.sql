-- Extracted from src/db/repository/background_policies.rs. Do not edit repository SQL directly.

-- name: BackgroundPoliciesGet :one
SELECT panel_backup_cron, log_cleanup_cron, log_retention_days, panel_backup_enabled, log_cleanup_enabled FROM background_policies WHERE id = 1;

-- name: BackgroundPoliciesUpdate :exec
UPDATE background_policies SET panel_backup_cron = ?, log_cleanup_cron = ?, log_retention_days = ?, panel_backup_enabled = ?, log_cleanup_enabled = ?, updated_at = CURRENT_TIMESTAMP WHERE id = 1;

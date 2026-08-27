-- Extracted from src/db/repository/health_report.rs. Do not edit repository SQL directly.

-- name: HealthReportList :many
SELECT id AS id, target_id, target_type, status, response_time_ms AS response_time_ms, error_message, created_at FROM health_reports ORDER BY created_at DESC LIMIT ?;

-- name: HealthReportCreate :exec
INSERT INTO health_reports (target_id, target_type, status, response_time_ms, error_message, created_at) VALUES (?, ?, ?, ?, ?, ?);

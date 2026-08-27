-- Extracted from src/db/repository/alert_rule.rs. Do not edit repository SQL directly.

-- name: AlertRuleList :many
SELECT id AS id, name, target_type, target_id, metric_name, operator, threshold AS threshold, duration_seconds AS duration_seconds, notification_channel, enabled AS enabled, organization_id AS organization_id, created_at, updated_at FROM alert_rules ORDER BY created_at DESC;

-- name: AlertRuleListByOrganization :many
SELECT id AS id, name, target_type, target_id, metric_name, operator, threshold AS threshold, duration_seconds AS duration_seconds, notification_channel, enabled AS enabled, organization_id AS organization_id, created_at, updated_at FROM alert_rules WHERE organization_id = ? ORDER BY created_at DESC;

-- name: AlertRuleGetById :one
SELECT id AS id, name, target_type, target_id, metric_name, operator, threshold AS threshold, duration_seconds AS duration_seconds, notification_channel, enabled AS enabled, organization_id AS organization_id, created_at, updated_at FROM alert_rules WHERE id = ?;

-- name: AlertRuleGetByIdForOrganization :one
SELECT id AS id, name, target_type, target_id, metric_name, operator, threshold AS threshold, duration_seconds AS duration_seconds, notification_channel, enabled AS enabled, organization_id AS organization_id, created_at, updated_at FROM alert_rules WHERE id = ? AND organization_id = ?;

-- name: AlertRuleCreate :exec
INSERT INTO alert_rules (name, target_type, target_id, metric_name, operator, threshold, duration_seconds, notification_channel, enabled, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: AlertRuleUpdate :exec
UPDATE alert_rules SET name = ?, target_type = ?, target_id = ?, metric_name = ?, operator = ?, threshold = ?, duration_seconds = ?, notification_channel = ?, enabled = ?, updated_at = ? WHERE id = ? AND organization_id = ?;

-- name: AlertRuleDelete :exec
DELETE FROM alert_rules WHERE id = ? AND organization_id = ?;

-- name: AlertRuleRecordEvent :exec
INSERT INTO alert_events (alert_rule_id, organization_id, target_key, state, value, threshold, message, resolved_at) VALUES (?, ?, ?, ?, ?, ?, ?, CASE WHEN ? = 'RESOLVED' THEN CURRENT_TIMESTAMP END);

-- name: AlertRuleListEvents :many
SELECT id AS id, alert_rule_id AS alert_rule_id, organization_id AS organization_id, target_key AS target_key, state AS state, value, threshold, message AS message, created_at AS created_at, acknowledged_at, acknowledged_by, silenced_until, resolved_at, notification_correlation_id FROM alert_events WHERE organization_id=? ORDER BY created_at DESC LIMIT ?;

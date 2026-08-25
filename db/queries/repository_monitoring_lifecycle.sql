-- Extracted from src/db/repository/monitoring_lifecycle.rs. Do not edit repository SQL directly.

-- name: MonitoringLifecyclePolicy1 :exec
INSERT INTO monitoring_policy (organization_id) VALUES (?) ON CONFLICT(organization_id) DO NOTHING;

-- name: MonitoringLifecyclePolicy2 :one
SELECT organization_id AS organization_id, desired_agent_version, retention_days AS retention_days, updated_at AS updated_at FROM monitoring_policy WHERE organization_id = ?;

-- name: MonitoringLifecycleUpdatePolicy :exec
INSERT INTO monitoring_policy (organization_id, desired_agent_version, retention_days, updated_at) VALUES (?, ?, ?, unixepoch()) ON CONFLICT(organization_id) DO UPDATE SET desired_agent_version=excluded.desired_agent_version, retention_days=excluded.retention_days, updated_at=excluded.updated_at;

-- name: MonitoringLifecycleAcknowledgeEvent :exec
UPDATE alert_events SET acknowledged_at=unixepoch(), acknowledged_by=? WHERE id=? AND organization_id=?;

-- name: MonitoringLifecycleSilenceEvent :exec
UPDATE alert_events SET silenced_until=? WHERE id=? AND organization_id=?;

-- name: MonitoringLifecycleCreateWindow :exec
INSERT INTO monitoring_maintenance_windows (organization_id, server_id, starts_at, ends_at, reason) VALUES (?, ?, ?, ?, ?);

-- name: MonitoringLifecycleListWindows :many
SELECT id AS id, organization_id AS organization_id, server_id, starts_at AS starts_at, ends_at AS ends_at, reason AS reason, created_at AS created_at FROM monitoring_maintenance_windows WHERE organization_id=? ORDER BY starts_at DESC;

-- name: MonitoringLifecycleDeleteWindow :exec
DELETE FROM monitoring_maintenance_windows WHERE id=? AND organization_id=?;

-- name: MonitoringLifecycleIsInMaintenance :one
SELECT EXISTS(SELECT 1 FROM monitoring_maintenance_windows WHERE organization_id=? AND (server_id IS NULL OR server_id=?) AND starts_at <= unixepoch() AND ends_at > unixepoch());

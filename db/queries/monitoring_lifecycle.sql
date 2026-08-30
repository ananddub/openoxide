-- Monitoring lifecycle: policy, event acknowledgement, maintenance windows.

-- name: MonitoringLifecycleEnsurePolicy :exec
INSERT INTO monitoring_policy (organization_id) VALUES (?) ON CONFLICT(organization_id) DO NOTHING;

-- name: MonitoringLifecyclePolicy :one
SELECT organization_id AS organization_id, desired_agent_version, retention_days AS retention_days, updated_at AS updated_at FROM monitoring_policy WHERE organization_id = ?;

-- name: MonitoringLifecycleUpdatePolicy :one
INSERT INTO monitoring_policy (organization_id, desired_agent_version, retention_days, updated_at) VALUES (?, ?, ?, unixepoch()) ON CONFLICT(organization_id) DO UPDATE SET desired_agent_version=excluded.desired_agent_version, retention_days=excluded.retention_days, updated_at=excluded.updated_at
RETURNING organization_id AS organization_id, desired_agent_version, retention_days AS retention_days, updated_at AS updated_at;

-- name: MonitoringLifecycleAcknowledgeEvent :one
UPDATE alert_events SET acknowledged_at=unixepoch(), acknowledged_by=? WHERE id=? AND organization_id=?
RETURNING id;

-- name: MonitoringLifecycleSilenceEvent :one
UPDATE alert_events SET silenced_until=? WHERE id=? AND organization_id=?
RETURNING id;

-- name: MonitoringLifecycleCreateWindow :one
INSERT INTO monitoring_maintenance_windows (organization_id, server_id, starts_at, ends_at, reason) VALUES (?, ?, ?, ?, ?)
RETURNING id;

-- name: MonitoringLifecycleCreateGlobalWindow :one
INSERT INTO monitoring_maintenance_windows (organization_id, server_id, starts_at, ends_at, reason) VALUES (?, NULL, ?, ?, ?)
RETURNING id;

-- name: MonitoringLifecycleListWindows :many
SELECT id AS id, organization_id AS organization_id, server_id, starts_at AS starts_at, ends_at AS ends_at, reason AS reason, created_at AS created_at FROM monitoring_maintenance_windows WHERE organization_id=? ORDER BY starts_at DESC;

-- name: MonitoringLifecycleDeleteWindow :one
DELETE FROM monitoring_maintenance_windows WHERE id=? AND organization_id=?
RETURNING id;

-- name: MonitoringLifecycleIsInMaintenance :one
SELECT COUNT(*) AS hit FROM monitoring_maintenance_windows WHERE organization_id=? AND (server_id IS NULL OR server_id=?) AND starts_at <= unixepoch() AND ends_at > unixepoch();

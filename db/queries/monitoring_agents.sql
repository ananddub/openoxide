-- Extracted from src/db/repository/monitoring_agents.rs. Do not edit repository SQL directly.

-- name: MonitoringAgentsRotate :exec
INSERT INTO monitoring_agents (server_id, organization_id, token_hash, query_token, updated_at)
             VALUES (?, ?, ?, ?, unixepoch())
             ON CONFLICT(server_id) DO UPDATE SET organization_id=excluded.organization_id,
             token_hash=excluded.token_hash, query_token=excluded.query_token, updated_at=excluded.updated_at;

-- name: MonitoringAgentsServerLinkedServiceCount :one
SELECT COUNT(*) AS count
FROM services
JOIN environments ON environments.id = services.environment_id
JOIN projects ON projects.id = environments.project_id
WHERE services.server_id = ? AND projects.organization_id = ?;

-- name: MonitoringAgentsGetTokenHash :one
SELECT token_hash FROM monitoring_agents WHERE server_id = ?;

-- name: MonitoringAgentsGetOrganizationId :one
SELECT organization_id FROM monitoring_agents WHERE server_id=?;

-- name: MonitoringAgentsTouchSeen :exec
UPDATE monitoring_agents SET last_seen_at=unixepoch(), updated_at=unixepoch() WHERE server_id=?;

-- name: MonitoringAgentsQueryToken :one
SELECT query_token FROM monitoring_agents WHERE server_id=?;

-- name: MonitoringAgentsStatus :one
SELECT server_id AS server_id, organization_id AS organization_id, last_seen_at, agent_version FROM monitoring_agents WHERE server_id=?;

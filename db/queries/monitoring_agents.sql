-- Extracted from src/db/repository/monitoring_agents.rs. Do not edit repository SQL directly.

-- name: MonitoringAgentsRotate :exec
INSERT INTO monitoring_agents (server_id, organization_id, token_hash, query_token, updated_at)
             VALUES (?, ?, ?, ?, strftime('%s', 'now'))
             ON CONFLICT(server_id) DO UPDATE SET organization_id=excluded.organization_id,
             token_hash=excluded.token_hash, query_token=excluded.query_token, updated_at=excluded.updated_at;

-- name: MonitoringAgentsServerBelongsToOrganization1 :one
SELECT EXISTS(
                SELECT 1 FROM applications a JOIN environments e ON e.id=a.environment_id JOIN projects p ON p.id=e.project_id WHERE a.server_id=? AND p.organization_id=?
                UNION ALL
                SELECT 1 FROM compose_projects c JOIN environments e ON e.id=c.environment_id JOIN projects p ON p.id=e.project_id WHERE c.server_id=? AND p.organization_id=?
            );

-- name: MonitoringAgentsServerBelongsToOrganization2 :one
SELECT organization_id FROM monitoring_agents WHERE server_id=?;

-- name: MonitoringAgentsGetTokenHash :one
SELECT token_hash FROM monitoring_agents WHERE server_id = ?;

-- name: MonitoringAgentsGetOrganizationId :one
SELECT organization_id FROM monitoring_agents WHERE server_id=?;

-- name: MonitoringAgentsTouchSeen :exec
UPDATE monitoring_agents SET last_seen_at=strftime('%s', 'now'), updated_at=strftime('%s', 'now') WHERE server_id=?;

-- name: MonitoringAgentsQueryToken :one
SELECT query_token FROM monitoring_agents WHERE server_id=?;

-- name: MonitoringAgentsStatus :one
SELECT server_id AS server_id, organization_id AS organization_id, last_seen_at, agent_version FROM monitoring_agents WHERE server_id=?;

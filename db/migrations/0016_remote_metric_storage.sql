-- Raw metrics live only on rustploy-monitor agents and are pulled through the
-- authenticated gRPC proxy. The panel keeps configuration, tokens and alerts.
DROP TABLE IF EXISTS container_metrics;
DROP TABLE IF EXISTS server_metrics;

-- The panel initiates authenticated gRPC reads, so it needs the current
-- per-agent credential as well as the one-way hash used to verify identities.
ALTER TABLE monitoring_agents ADD COLUMN query_token TEXT;

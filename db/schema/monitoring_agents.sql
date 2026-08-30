CREATE TABLE monitoring_agents (
    server_id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    organization_id BIGINT NOT NULL,
    token_hash TEXT NOT NULL,
    last_seen_at BIGINT,
    agent_version TEXT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    query_token TEXT
);

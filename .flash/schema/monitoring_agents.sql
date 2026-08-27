CREATE TABLE monitoring_agents (
    server_id INTEGER NOT NULL PRIMARY KEY,
    organization_id INTEGER NOT NULL,
    token_hash TEXT NOT NULL,
    last_seen_at INTEGER,
    agent_version TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    query_token TEXT
);

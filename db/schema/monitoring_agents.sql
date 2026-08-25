CREATE TABLE monitoring_agents (
    server_id INTEGER PRIMARY KEY REFERENCES servers(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    last_seen_at INTEGER,
    agent_version TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
, query_token TEXT) STRICT;

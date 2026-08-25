CREATE TABLE rollbacks (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	deployment_id INTEGER NOT NULL REFERENCES deployments(id) ON DELETE CASCADE,
	version INTEGER NOT NULL DEFAULT 1,
	image TEXT,
	full_context TEXT, -- JSON snapshot of application configs
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;
CREATE INDEX idx_rollbacks_deployment_id ON rollbacks(deployment_id);

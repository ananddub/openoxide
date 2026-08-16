-- deployment queue support
PRAGMA foreign_keys = OFF;

CREATE TABLE deployments_new (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	title TEXT NOT NULL,
	description TEXT,
	status TEXT NOT NULL DEFAULT 'QUEUED',
	state TEXT NOT NULL DEFAULT 'QUEUE',
	log_path TEXT NOT NULL,
	pid TEXT,
	error_message TEXT,
	operation TEXT,
	is_preview_deployment INTEGER NOT NULL DEFAULT 0,
	started_at INTEGER,
	last_state_at INTEGER,
	finished_at INTEGER,
	application_id INTEGER REFERENCES applications(id) ON DELETE CASCADE,
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT deployment_status_check CHECK (status IN ('QUEUED', 'RUNNING', 'DONE', 'ERROR', 'CANCELLED'))
) STRICT;

INSERT INTO deployments_new (
	id, title, description, status, state, log_path, pid, error_message, operation,
	is_preview_deployment, started_at, last_state_at, finished_at,
	application_id, compose_id, server_id, created_at
)
SELECT
	id, title, description, status, 'QUEUE', log_path, pid, error_message, NULL,
	is_preview_deployment, started_at, NULL, finished_at,
	application_id, compose_id, server_id, created_at
FROM deployments;

DROP TABLE deployments;
ALTER TABLE deployments_new RENAME TO deployments;

CREATE INDEX idx_deployments_status ON deployments(status);
CREATE INDEX idx_deployments_state ON deployments(state);
CREATE INDEX idx_deployments_created_at ON deployments(created_at);
CREATE INDEX idx_deployments_compose_id ON deployments(compose_id);
CREATE INDEX idx_deployments_application_id ON deployments(application_id);
CREATE UNIQUE INDEX idx_deployments_one_active_application
	ON deployments(application_id)
	WHERE status IN ('QUEUED', 'RUNNING') AND application_id IS NOT NULL;
CREATE UNIQUE INDEX idx_deployments_one_active_compose
	ON deployments(compose_id)
	WHERE status IN ('QUEUED', 'RUNNING') AND compose_id IS NOT NULL;

PRAGMA foreign_keys = ON;

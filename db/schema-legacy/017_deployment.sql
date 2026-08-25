-- deployments (Execution logs)
CREATE TABLE deployments (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	title TEXT NOT NULL,
	description TEXT,
	-- status: QUEUED | RUNNING | DONE | ERROR | CANCELLED
	status TEXT NOT NULL DEFAULT 'QUEUED',
	-- Last live builder state, e.g. QUEUE | BUILDING | DEPLOYING | HEALTH_CHECK
	state TEXT NOT NULL DEFAULT 'QUEUE',
	log_path TEXT NOT NULL,
	pid TEXT,
	error_message TEXT,
	operation TEXT,
	is_preview_deployment INTEGER NOT NULL DEFAULT 0,
	started_at INTEGER,
	last_state_at INTEGER,
	finished_at INTEGER,
	-- Foreign keys
	application_id INTEGER REFERENCES applications(id) ON DELETE CASCADE,
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	database_id INTEGER,
	database_kind TEXT,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT deployment_status_check CHECK (status IN ('QUEUED', 'RUNNING', 'DONE', 'ERROR', 'CANCELLED'))
) STRICT;

-- rollbacks (Snapshots for reversion)
CREATE TABLE rollbacks (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	deployment_id INTEGER NOT NULL REFERENCES deployments(id) ON DELETE CASCADE,
	version INTEGER NOT NULL DEFAULT 1,
	image TEXT,
	full_context TEXT, -- JSON snapshot of application configs
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Indexes for rollbacks
CREATE INDEX idx_rollbacks_deployment_id ON rollbacks(deployment_id);
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

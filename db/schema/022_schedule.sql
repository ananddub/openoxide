CREATE TABLE schedules (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	description TEXT,
	cron_expression TEXT NOT NULL,
	app_name TEXT NOT NULL UNIQUE,
	service_name TEXT,
	-- shell_type: BASH | SH
	shell_type TEXT NOT NULL DEFAULT 'BASH',
	-- schedule_type: APPLICATION | COMPOSE | SERVER | DOKPANEL-SERVER
	schedule_type TEXT NOT NULL DEFAULT 'APPLICATION',
	-- schedule_action: EXEC | DEPLOY | REDEPLOY | REBUILD | RELOAD | START | STOP
	schedule_action TEXT NOT NULL DEFAULT 'EXEC',
	command TEXT NOT NULL,
	script TEXT,
	timezone TEXT,
	enabled INTEGER NOT NULL DEFAULT 1,
	-- Foreign keys
	application_id INTEGER REFERENCES applications(id) ON DELETE CASCADE,
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	organization_id INTEGER REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT schedule_shell_type_check CHECK (shell_type IN ('BASH', 'SH')),
	CONSTRAINT schedule_type_check CHECK (schedule_type IN ('APPLICATION', 'COMPOSE', 'SERVER', 'DOKPANEL-SERVER')),
	CONSTRAINT schedule_action_check CHECK (schedule_action IN ('EXEC', 'DEPLOY', 'REDEPLOY', 'REBUILD', 'RELOAD', 'START', 'STOP'))
) STRICT;

CREATE TABLE schedule_runtime_policies (
    schedule_id INTEGER PRIMARY KEY REFERENCES schedules(id) ON DELETE CASCADE,
    retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count BETWEEN 0 AND 10),
    retry_delay_seconds INTEGER NOT NULL DEFAULT 30 CHECK (retry_delay_seconds BETWEEN 1 AND 86400),
    missed_run_policy TEXT NOT NULL DEFAULT 'SKIP' CHECK (missed_run_policy IN ('SKIP', 'RUN_ONCE')),
    concurrency_policy TEXT NOT NULL DEFAULT 'SKIP' CHECK (concurrency_policy IN ('SKIP', 'QUEUE', 'ALLOW')),
    lease_seconds INTEGER NOT NULL DEFAULT 3600 CHECK (lease_seconds BETWEEN 30 AND 86400),
    notify_on_success INTEGER NOT NULL DEFAULT 0 CHECK (notify_on_success IN (0, 1)),
    notify_on_failure INTEGER NOT NULL DEFAULT 1 CHECK (notify_on_failure IN (0, 1)),
    last_scheduled_at INTEGER,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE TABLE schedule_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    schedule_id INTEGER NOT NULL REFERENCES schedules(id) ON DELETE CASCADE,
    organization_id INTEGER,
    trigger_kind TEXT NOT NULL CHECK (trigger_kind IN ('CRON', 'MANUAL', 'MISSED', 'RETRY')),
    status TEXT NOT NULL CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED', 'SKIPPED')),
    attempt INTEGER NOT NULL DEFAULT 1,
    scheduled_at INTEGER NOT NULL,
    started_at INTEGER NOT NULL DEFAULT (unixepoch()),
    finished_at INTEGER,
    message TEXT,
    stdout TEXT,
    stderr TEXT
) STRICT;
CREATE INDEX idx_schedule_executions_schedule_started ON schedule_executions(schedule_id, started_at DESC);
CREATE INDEX idx_schedule_executions_org_started ON schedule_executions(organization_id, started_at DESC);

CREATE TABLE schedule_execution_locks (
    schedule_id INTEGER PRIMARY KEY REFERENCES schedules(id) ON DELETE CASCADE,
    owner_id TEXT NOT NULL,
    acquired_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_schedules_application_id ON schedules(application_id);
CREATE INDEX idx_schedules_compose_id ON schedules(compose_id);
CREATE INDEX idx_schedules_server_id ON schedules(server_id);
CREATE INDEX idx_schedules_organization_id ON schedules(organization_id);

-- Trigger Function
CREATE TRIGGER schedules_updated_at
AFTER UPDATE ON schedules
FOR EACH ROW
BEGIN
	UPDATE schedules
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

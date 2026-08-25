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
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL, schedule_action TEXT NOT NULL DEFAULT 'EXEC'
CHECK (schedule_action IN ('EXEC', 'DEPLOY', 'REDEPLOY', 'REBUILD', 'RELOAD', 'START', 'STOP')),
	CONSTRAINT schedule_shell_type_check CHECK (shell_type IN ('BASH', 'SH')),
	CONSTRAINT schedule_type_check CHECK (schedule_type IN ('APPLICATION', 'COMPOSE', 'SERVER', 'DOKPANEL-SERVER'))
) STRICT;
CREATE INDEX idx_schedules_application_id ON schedules(application_id);
CREATE INDEX idx_schedules_compose_id ON schedules(compose_id);
CREATE INDEX idx_schedules_server_id ON schedules(server_id);
CREATE INDEX idx_schedules_organization_id ON schedules(organization_id);
CREATE TRIGGER schedules_updated_at
AFTER UPDATE ON schedules
FOR EACH ROW
BEGIN
	UPDATE schedules
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

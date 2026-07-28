CREATE TABLE compose_projects (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- Unique slug used as Docker stack name
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	env_var TEXT,
	compose_file TEXT NOT NULL DEFAULT '',
	refresh_token TEXT,
	-- source_type: GIT | GITHUB | GITLAB | BITBUCKET | GITEA | RAW
	source_type TEXT NOT NULL DEFAULT 'GITHUB',
	-- compose_type: DOCKER-COMPOSE | STACK
	compose_type TEXT NOT NULL DEFAULT 'DOCKER-COMPOSE',
	-- compose_status: IDLE | RUNNING | DONE | ERROR
	compose_status TEXT NOT NULL DEFAULT 'IDLE',
	-- trigger_type: PUSH | TAG
	trigger_type TEXT NOT NULL DEFAULT 'PUSH',
	-- Github source
	repository TEXT,
	owner TEXT,
	branch TEXT,
	auto_deploy INTEGER NOT NULL DEFAULT 1,
	-- GitLab source
	gitlab_project_id INTEGER,
	gitlab_repository TEXT,
	gitlab_owner TEXT,
	gitlab_branch TEXT,
	gitlab_path_namespace TEXT,
	-- Bitbucket source
	bitbucket_repository TEXT,
	bitbucket_repository_slug TEXT,
	bitbucket_owner TEXT,
	bitbucket_branch TEXT,
	-- Gitea source
	gitea_repository TEXT,
	gitea_owner TEXT,
	gitea_branch TEXT,
	-- Custom Git source
	custom_git_url TEXT,
	custom_git_branch TEXT,
	custom_git_ssh_key_id INTEGER REFERENCES ssh_keys(id) ON DELETE SET NULL,
	-- Build & run config
	command TEXT NOT NULL DEFAULT '',
	enable_submodules INTEGER NOT NULL DEFAULT 0,
	compose_path TEXT NOT NULL DEFAULT './docker-compose.yml',
	suffix TEXT NOT NULL DEFAULT '',
	randomize INTEGER NOT NULL DEFAULT 0,
	isolated_deployment INTEGER NOT NULL DEFAULT 0,
	isolated_deployments_volume INTEGER NOT NULL DEFAULT 0,
	watch_paths TEXT, -- JSON array of strings
	service_networks TEXT NOT NULL DEFAULT '[]', -- JSON array of per-service network configs
	-- Foreign keys
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	github_provider_id INTEGER REFERENCES github_providers(id) ON DELETE SET NULL,
	gitlab_provider_id INTEGER REFERENCES gitlab_providers(id) ON DELETE SET NULL,
	gitea_provider_id INTEGER REFERENCES gitea_providers(id) ON DELETE SET NULL,
	bitbucket_provider_id INTEGER REFERENCES bitbucket_providers(id) ON DELETE SET NULL,
	-- Timestamps
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT compose_source_type_check CHECK (source_type IN ('GIT', 'GITHUB', 'GITLAB', 'BITBUCKET', 'GITEA', 'RAW')),
	CONSTRAINT compose_type_check CHECK (compose_type IN ('DOCKER-COMPOSE', 'STACK')),
	CONSTRAINT compose_status_check CHECK (compose_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED')),
	CONSTRAINT compose_trigger_type_check CHECK (trigger_type IN ('PUSH', 'TAG'))
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_compose_projects_environment_id ON compose_projects(environment_id);
CREATE INDEX idx_compose_projects_server_id ON compose_projects(server_id);

-- Trigger Function
CREATE TRIGGER compose_projects_updated_at
AFTER UPDATE ON compose_projects
FOR EACH ROW
BEGIN
	UPDATE compose_projects
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TABLE applications (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- Unique slug used as Docker service name (e.g. 'my-app-x7k2')
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	-- source_type: DOCKER | GIT | GITHUB | GITLAB | BITBUCKET | GITEA | DROP
	source_type TEXT NOT NULL DEFAULT 'GITHUB',
	-- build_type: DOCKERFILE | HEROKU_BUILDPACKS | PAKETO_BUILDPACKS | NIXPACKS | STATIC | RAILPACK
	build_type TEXT NOT NULL DEFAULT 'NIXPACKS',
	-- app_status: IDLE | RUNNING | DONE | ERROR
	app_status TEXT NOT NULL DEFAULT 'IDLE',
	-- trigger_type: PUSH | TAG
	trigger_type TEXT NOT NULL DEFAULT 'PUSH',
	-- Build config
	build_args TEXT,
	build_secrets TEXT,
	dockerfile TEXT DEFAULT 'Dockerfile',
	docker_context_path TEXT,
	docker_build_stage TEXT,
	publish_directory TEXT,
	is_static_spa INTEGER DEFAULT 0,
	create_env_file INTEGER NOT NULL DEFAULT 1,
	railpack_version TEXT DEFAULT '0.15.4',
	heroku_version TEXT DEFAULT '24',
	command TEXT,
	args TEXT, -- JSON array of args e.g. '["--port","3000"]'
	env_var TEXT,
	build_path TEXT DEFAULT '/',
	clean_cache INTEGER NOT NULL DEFAULT 0,
	drop_build_path TEXT,
	enable_submodules INTEGER NOT NULL DEFAULT 0,
	watch_paths TEXT, -- JSON array of paths to watch for auto-deploy
	refresh_token TEXT,
	icon TEXT,
	-- Resource limits
	memory_reservation TEXT,
	memory_limit TEXT,
	cpu_reservation TEXT,
	cpu_limit TEXT,
	replicas INTEGER NOT NULL DEFAULT 1,
	-- Docker Swarm JSON configs (stored as JSON text)
	health_check_swarm TEXT,
	restart_policy_swarm TEXT,
	placement_swarm TEXT,
	update_config_swarm TEXT,
	rollback_config_swarm TEXT,
	mode_swarm TEXT,
	labels_swarm TEXT,
	network_swarm TEXT,
	endpoint_spec_swarm TEXT,
	ulimits_swarm TEXT,
	stop_grace_period_swarm INTEGER,
	-- GitHub source
	repository TEXT,
	owner TEXT,
	branch TEXT,
	auto_deploy INTEGER DEFAULT 1,
	-- GitLab source
	gitlab_project_id INTEGER,
	gitlab_repository TEXT,
	gitlab_owner TEXT,
	gitlab_branch TEXT,
	gitlab_build_path TEXT DEFAULT '/',
	gitlab_path_namespace TEXT,
	-- Gitea source
	gitea_repository TEXT,
	gitea_owner TEXT,
	gitea_branch TEXT,
	gitea_build_path TEXT DEFAULT '/',
	-- Bitbucket source
	bitbucket_repository TEXT,
	bitbucket_repository_slug TEXT,
	bitbucket_owner TEXT,
	bitbucket_branch TEXT,
	bitbucket_build_path TEXT DEFAULT '/',
	-- Docker image source
	docker_image TEXT,
	docker_username TEXT,
	docker_password TEXT,
	registry_url TEXT,
	-- Custom Git (SSH) source
	custom_git_url TEXT,
	custom_git_branch TEXT,
	custom_git_build_path TEXT,
	custom_git_ssh_key_id INTEGER REFERENCES ssh_keys(id) ON DELETE SET NULL,
	-- Preview deployments
	preview_env TEXT,
	preview_build_args TEXT,
	preview_build_secrets TEXT,
	preview_labels TEXT, -- JSON array of preview labels
	preview_wildcard TEXT,
	preview_port INTEGER DEFAULT 3000,
	preview_https INTEGER NOT NULL DEFAULT 0,
	preview_path TEXT DEFAULT '/',
	-- preview_certificate_type: LETSENCRYPT | NONE | CUSTOM
	preview_certificate_type TEXT NOT NULL DEFAULT 'NONE',
	preview_custom_cert_resolver TEXT,
	preview_limit INTEGER DEFAULT 3,
	is_preview_deployments_active INTEGER NOT NULL DEFAULT 0,
	preview_require_collaborator_permissions INTEGER NOT NULL DEFAULT 1,
	rollback_active INTEGER NOT NULL DEFAULT 0,
	-- Foreign keys (Inline References)
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	build_server_id INTEGER REFERENCES servers(id) ON DELETE SET NULL,
	registry_id INTEGER REFERENCES registries(id) ON DELETE SET NULL,
	rollback_registry_id INTEGER REFERENCES registries(id) ON DELETE SET NULL,
	build_registry_id INTEGER REFERENCES registries(id) ON DELETE SET NULL,
	github_provider_id INTEGER REFERENCES github_providers(id) ON DELETE SET NULL,
	gitlab_provider_id INTEGER REFERENCES gitlab_providers(id) ON DELETE SET NULL,
	gitea_provider_id INTEGER REFERENCES gitea_providers(id) ON DELETE SET NULL,
	bitbucket_provider_id INTEGER REFERENCES bitbucket_providers(id) ON DELETE SET NULL,
	-- Timestamps
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT app_source_type_check CHECK (source_type IN ('DOCKER', 'GIT', 'GITHUB', 'GITLAB', 'BITBUCKET', 'GITEA', 'DROP')),
	CONSTRAINT app_build_type_check CHECK (build_type IN ('DOCKERFILE', 'HEROKU_BUILDPACKS', 'PAKETO_BUILDPACKS', 'NIXPACKS', 'STATIC', 'RAILPACK')),
	CONSTRAINT app_status_check CHECK (app_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED')),
	CONSTRAINT app_trigger_type_check CHECK (trigger_type IN ('PUSH', 'TAG')),
	CONSTRAINT app_preview_cert_check CHECK (preview_certificate_type IN ('LETSENCRYPT', 'NONE', 'CUSTOM'))
) STRICT;

CREATE INDEX idx_applications_environment_id ON applications(environment_id);
CREATE INDEX idx_applications_server_id ON applications(server_id);
CREATE INDEX idx_applications_app_status ON applications(app_status);

-- Trigger Function
CREATE TRIGGER applications_updated_at
AFTER UPDATE ON applications
FOR EACH ROW
BEGIN
	UPDATE applications
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
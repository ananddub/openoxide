-- Git/VCS source config, shared by APPLICATION and COMPOSE services
CREATE TABLE service_git (
    service_id BIGINT NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    source_type TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    auto_deploy BIGINT,
    watch_paths TEXT,
    enable_submodules BIGINT NOT NULL,
    repository TEXT,
    owner TEXT,
    branch TEXT,
    gitlab_project_id BIGINT,
    gitlab_repository TEXT,
    gitlab_owner TEXT,
    gitlab_branch TEXT,
    gitlab_build_path TEXT,
    gitlab_path_namespace TEXT,
    gitea_repository TEXT,
    gitea_owner TEXT,
    gitea_branch TEXT,
    gitea_build_path TEXT,
    bitbucket_repository TEXT,
    bitbucket_repository_slug TEXT,
    bitbucket_owner TEXT,
    bitbucket_branch TEXT,
    bitbucket_build_path TEXT,
    custom_git_url TEXT,
    custom_git_branch TEXT,
    custom_git_build_path TEXT,
    custom_git_ssh_key_id BIGINT REFERENCES ssh_keys(id) ON DELETE SET NULL,
    github_provider_id BIGINT REFERENCES git_providers(id) ON DELETE SET NULL,
    gitlab_provider_id BIGINT REFERENCES git_providers(id) ON DELETE SET NULL,
    gitea_provider_id BIGINT REFERENCES git_providers(id) ON DELETE SET NULL,
    bitbucket_provider_id BIGINT REFERENCES git_providers(id) ON DELETE SET NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_service_git_ssh_key ON service_git(custom_git_ssh_key_id);

CREATE TRIGGER service_git_updated_at
AFTER UPDATE ON service_git
FOR EACH ROW
BEGIN
    UPDATE service_git SET updated_at = strftime('%s', 'now') WHERE service_id = OLD.service_id;
END;

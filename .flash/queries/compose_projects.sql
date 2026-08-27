-- Extracted from src/db/repository/compose_projects.rs. Do not edit repository SQL directly.

-- name: ComposeProjectsGetAll :many
SELECT id AS id, name AS name, app_name AS app_name, description AS description, env_var AS env_var, compose_file AS compose_file, refresh_token AS refresh_token, source_type AS source_type, compose_type AS compose_type, compose_status AS compose_status, last_error AS last_error, trigger_type AS trigger_type, repository AS repository, owner AS owner, branch AS branch, auto_deploy AS auto_deploy, gitlab_project_id AS gitlab_project_id, gitlab_repository AS gitlab_repository, gitlab_owner AS gitlab_owner, gitlab_branch AS gitlab_branch, gitlab_path_namespace AS gitlab_path_namespace, bitbucket_repository AS bitbucket_repository, bitbucket_repository_slug AS bitbucket_repository_slug, bitbucket_owner AS bitbucket_owner, bitbucket_branch AS bitbucket_branch, gitea_repository AS gitea_repository, gitea_owner AS gitea_owner, gitea_branch AS gitea_branch, custom_git_url AS custom_git_url, custom_git_branch AS custom_git_branch, custom_git_ssh_key_id AS custom_git_ssh_key_id, command AS command, enable_submodules AS enable_submodules, compose_path AS compose_path, suffix AS suffix, randomize AS randomize, isolated_deployment AS isolated_deployment, isolated_deployments_volume AS isolated_deployments_volume, watch_paths AS watch_paths, service_networks AS service_networks, environment_id AS environment_id, server_id AS server_id, github_provider_id AS github_provider_id, gitlab_provider_id AS gitlab_provider_id, gitea_provider_id AS gitea_provider_id, bitbucket_provider_id AS bitbucket_provider_id, created_at AS created_at, updated_at AS updated_at FROM compose_projects;

-- name: ComposeProjectsGetById :one
SELECT id AS id, name AS name, app_name AS app_name, description AS description, env_var AS env_var, compose_file AS compose_file, refresh_token AS refresh_token, source_type AS source_type, compose_type AS compose_type, compose_status AS compose_status, last_error AS last_error, trigger_type AS trigger_type, repository AS repository, owner AS owner, branch AS branch, auto_deploy AS auto_deploy, gitlab_project_id AS gitlab_project_id, gitlab_repository AS gitlab_repository, gitlab_owner AS gitlab_owner, gitlab_branch AS gitlab_branch, gitlab_path_namespace AS gitlab_path_namespace, bitbucket_repository AS bitbucket_repository, bitbucket_repository_slug AS bitbucket_repository_slug, bitbucket_owner AS bitbucket_owner, bitbucket_branch AS bitbucket_branch, gitea_repository AS gitea_repository, gitea_owner AS gitea_owner, gitea_branch AS gitea_branch, custom_git_url AS custom_git_url, custom_git_branch AS custom_git_branch, custom_git_ssh_key_id AS custom_git_ssh_key_id, command AS command, enable_submodules AS enable_submodules, compose_path AS compose_path, suffix AS suffix, randomize AS randomize, isolated_deployment AS isolated_deployment, isolated_deployments_volume AS isolated_deployments_volume, watch_paths AS watch_paths, service_networks AS service_networks, environment_id AS environment_id, server_id AS server_id, github_provider_id AS github_provider_id, gitlab_provider_id AS gitlab_provider_id, gitea_provider_id AS gitea_provider_id, bitbucket_provider_id AS bitbucket_provider_id, created_at AS created_at, updated_at AS updated_at FROM compose_projects WHERE id = ?;

-- name: ComposeProjectsCreate :exec
INSERT INTO compose_projects (name, app_name, description, env_var, compose_file, refresh_token, source_type, compose_type, compose_status, trigger_type, repository, owner, branch, auto_deploy, gitlab_project_id, gitlab_repository, gitlab_owner, gitlab_branch, gitlab_path_namespace, bitbucket_repository, bitbucket_repository_slug, bitbucket_owner, bitbucket_branch, gitea_repository, gitea_owner, gitea_branch, custom_git_url, custom_git_branch, custom_git_ssh_key_id, command, enable_submodules, compose_path, suffix, randomize, isolated_deployment, isolated_deployments_volume, watch_paths, service_networks, environment_id, server_id, github_provider_id, gitlab_provider_id, gitea_provider_id, bitbucket_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: ComposeProjectsUpdate :exec
UPDATE compose_projects SET name = ?, app_name = ?, description = ?, env_var = ?, compose_file = ?, refresh_token = ?, source_type = ?, compose_type = ?, compose_status = ?, trigger_type = ?, repository = ?, owner = ?, branch = ?, auto_deploy = ?, gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?, gitlab_path_namespace = ?, bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?, bitbucket_branch = ?, gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, custom_git_url = ?, custom_git_branch = ?, custom_git_ssh_key_id = ?, command = ?, enable_submodules = ?, compose_path = ?, suffix = ?, randomize = ?, isolated_deployment = ?, isolated_deployments_volume = ?, watch_paths = ?, service_networks = ?, environment_id = ?, server_id = ?, github_provider_id = ?, gitlab_provider_id = ?, gitea_provider_id = ?, bitbucket_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: ComposeProjectsDelete :exec
DELETE FROM compose_projects WHERE id = ?;

-- name: ComposeProjectsUpdateComposeFile :exec
UPDATE compose_projects SET compose_file = ?, source_type = 'RAW', updated_at = CURRENT_TIMESTAMP WHERE id = ?;

-- name: ComposeProjectsMoveToEnvironmentInSameOrganization :exec
UPDATE compose_projects
               SET environment_id = ?, updated_at = CURRENT_TIMESTAMP
               WHERE id = ? AND EXISTS (
                 SELECT 1 FROM environments source_environment
                 JOIN projects source_project ON source_project.id = source_environment.project_id
                 JOIN environments target_environment ON target_environment.id = ?
                 JOIN projects target_project ON target_project.id = target_environment.project_id
                 WHERE source_environment.id = compose_projects.environment_id
                   AND source_project.organization_id = target_project.organization_id
               );

-- name: ComposeProjectsRotateRefreshToken :exec
UPDATE compose_projects SET refresh_token = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?;

-- name: ComposeProjectsListByEnvironment :many
SELECT id AS id, name AS name, app_name AS app_name, description AS description, env_var AS env_var, compose_file AS compose_file, refresh_token AS refresh_token, source_type AS source_type, compose_type AS compose_type, compose_status AS compose_status, last_error AS last_error, trigger_type AS trigger_type, repository AS repository, owner AS owner, branch AS branch, auto_deploy AS auto_deploy, gitlab_project_id AS gitlab_project_id, gitlab_repository AS gitlab_repository, gitlab_owner AS gitlab_owner, gitlab_branch AS gitlab_branch, gitlab_path_namespace AS gitlab_path_namespace, bitbucket_repository AS bitbucket_repository, bitbucket_repository_slug AS bitbucket_repository_slug, bitbucket_owner AS bitbucket_owner, bitbucket_branch AS bitbucket_branch, gitea_repository AS gitea_repository, gitea_owner AS gitea_owner, gitea_branch AS gitea_branch, custom_git_url AS custom_git_url, custom_git_branch AS custom_git_branch, custom_git_ssh_key_id AS custom_git_ssh_key_id, command AS command, enable_submodules AS enable_submodules, compose_path AS compose_path, suffix AS suffix, randomize AS randomize, isolated_deployment AS isolated_deployment, isolated_deployments_volume AS isolated_deployments_volume, watch_paths AS watch_paths, service_networks AS service_networks, environment_id AS environment_id, server_id AS server_id, github_provider_id AS github_provider_id, gitlab_provider_id AS gitlab_provider_id, gitea_provider_id AS gitea_provider_id, bitbucket_provider_id AS bitbucket_provider_id, created_at AS created_at, updated_at AS updated_at FROM compose_projects WHERE environment_id = ? ORDER BY created_at DESC, id DESC;

-- name: ComposeProjectsCreateSimple :exec
INSERT INTO compose_projects (name, app_name, description, environment_id, server_id, source_type, compose_type, compose_file)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?);

-- name: ComposeProjectsPatch :exec
UPDATE compose_projects SET
               name = ?, description = ?, env_var = ?, compose_file = ?, compose_type = ?,
               trigger_type = ?, command = ?, enable_submodules = COALESCE(?, enable_submodules),
               compose_path = ?, suffix = COALESCE(?, suffix), randomize = COALESCE(?, randomize),
               isolated_deployment = COALESCE(?, isolated_deployment),
               isolated_deployments_volume = COALESCE(?, isolated_deployments_volume),
               watch_paths = COALESCE(?, watch_paths), service_networks = ?, server_id = ?
               WHERE id = ?;

-- name: ComposeProjectsUpdateStatus :exec
UPDATE compose_projects SET compose_status = ?, last_error = NULL WHERE id = ?;

-- name: ComposeProjectsGetDeploymentContext :one
SELECT c.environment_id, e.project_id, c.server_id
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               WHERE c.id = ?;

-- name: ComposeProjectsSetGithubSource :exec
UPDATE compose_projects SET source_type = 'GITHUB',
               repository = ?, owner = ?, branch = ?, github_provider_id = ?, auto_deploy = ?,
               gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
               gitlab_path_namespace = NULL, gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?;

-- name: ComposeProjectsSetGitlabSource :exec
UPDATE compose_projects SET source_type = 'GITLAB',
               gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?,
               gitlab_path_namespace = ?, gitlab_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitea_repository = NULL, gitea_owner = NULL,
               gitea_branch = NULL, bitbucket_repository = NULL, bitbucket_repository_slug = NULL,
               bitbucket_owner = NULL, bitbucket_branch = NULL, custom_git_url = NULL,
               custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?;

-- name: ComposeProjectsSetGiteaSource :exec
UPDATE compose_projects SET source_type = 'GITEA',
               gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?;

-- name: ComposeProjectsSetBitbucketSource :exec
UPDATE compose_projects SET source_type = 'BITBUCKET',
               bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?,
               bitbucket_branch = ?, bitbucket_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?;

-- name: ComposeProjectsSetCustomGitSource :exec
UPDATE compose_projects SET source_type = 'GIT',
               custom_git_url = ?, custom_git_branch = ?, custom_git_ssh_key_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL
               WHERE id = ?;

-- name: ComposeProjectsSetRawSource :exec
UPDATE compose_projects SET source_type = 'RAW', compose_file = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?;

-- name: ComposeProjectsSetDropSource :exec
UPDATE compose_projects SET source_type = 'DROP',
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?;

-- name: ComposeProjectsGetSpecRow :one
SELECT
               c.app_name, c.source_type, c.compose_type, c.compose_file, c.env_var,
               c.service_networks, c.randomize, c.suffix, c.isolated_deployment,
               c.isolated_deployments_volume,
               c.repository, c.owner, c.branch, c.gitlab_repository, c.gitlab_owner,
               c.gitlab_branch, c.gitea_repository, c.gitea_branch, c.bitbucket_repository,
               c.bitbucket_owner, c.bitbucket_branch, c.custom_git_url, c.custom_git_branch,
               c.custom_git_ssh_key_id, c.enable_submodules, c.compose_path, e.env_var AS environment_env,
               p.env_var AS project_env,
               ghp.github_private_key AS github_token,
               glp.access_token AS gitlab_token,
               bbp.api_token AS bitbucket_token,
               gtp.access_token AS gitea_token,
               sk.private_key AS ssh_private_key
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               JOIN projects p ON p.id = e.project_id
               LEFT JOIN github_providers ghp ON ghp.id = c.github_provider_id
               LEFT JOIN gitlab_providers glp ON glp.id = c.gitlab_provider_id
               LEFT JOIN bitbucket_providers bbp ON bbp.id = c.bitbucket_provider_id
               LEFT JOIN gitea_providers gtp ON gtp.id = c.gitea_provider_id
               LEFT JOIN ssh_keys sk ON sk.id = c.custom_git_ssh_key_id
               WHERE c.id = ?;

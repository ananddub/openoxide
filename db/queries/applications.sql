-- ============================================================
-- APPLICATIONS
-- ============================================================

-- name: GetApplicationByID :one
SELECT * FROM applications WHERE id = ?;

-- name: GetApplicationByAppName :one
SELECT * FROM applications WHERE app_name = ?;

-- name: ListApplicationsByEnvironment :many
SELECT * FROM applications WHERE environment_id = ? ORDER BY created_at DESC;

-- name: ListApplicationsByServer :many
SELECT * FROM applications WHERE server_id = ? ORDER BY created_at DESC;

-- name: ListApplicationsByStatus :many
SELECT * FROM applications WHERE app_status = ? ORDER BY created_at DESC;

-- name: CreateApplication :one
INSERT INTO applications (
    name, app_name, description,
    source_type, build_type, app_status, trigger_type,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateApplicationStatus :exec
UPDATE applications SET app_status = ? WHERE id = ?;

-- name: UpdateApplicationSource :one
UPDATE applications
SET
    source_type           = ?,
    repository            = ?,
    owner                 = ?,
    branch                = ?,
    auto_deploy           = ?,
    github_provider_id    = ?,
    gitlab_project_id     = ?,
    gitlab_repository     = ?,
    gitlab_owner          = ?,
    gitlab_branch         = ?,
    gitlab_path_namespace = ?,
    gitea_repository      = ?,
    gitea_owner           = ?,
    gitea_branch          = ?,
    bitbucket_repository  = ?,
    bitbucket_owner       = ?,
    bitbucket_branch      = ?,
    docker_image          = ?,
    docker_username       = ?,
    docker_password       = ?,
    registry_url          = ?,
    custom_git_url        = ?,
    custom_git_branch     = ?,
    custom_git_ssh_key_id = ?
WHERE id = ?
RETURNING *;

-- name: UpdateApplicationBuildConfig :one
UPDATE applications
SET
    build_type            = ?,
    build_args            = ?,
    build_secrets         = ?,
    dockerfile            = ?,
    docker_context_path   = ?,
    docker_build_stage    = ?,
    publish_directory     = ?,
    is_static_spa         = ?,
    create_env_file       = ?,
    railpack_version      = ?,
    heroku_version        = ?,
    command               = ?,
    args                  = ?,
    env_var               = ?,
    build_path            = ?,
    clean_cache           = ?,
    enable_submodules     = ?,
    watch_paths           = ?
WHERE id = ?
RETURNING *;

-- name: UpdateApplicationResources :one
UPDATE applications
SET
    memory_reservation = ?,
    memory_limit       = ?,
    cpu_reservation    = ?,
    cpu_limit          = ?,
    replicas           = ?
WHERE id = ?
RETURNING *;

-- name: UpdateApplicationServer :exec
UPDATE applications SET server_id = ? WHERE id = ?;

-- name: UpdateApplicationEnvVar :exec
UPDATE applications SET env_var = ? WHERE id = ?;

-- name: UpdateApplicationRefreshToken :exec
UPDATE applications SET refresh_token = ? WHERE id = ?;

-- name: UpdateApplicationRollback :exec
UPDATE applications SET rollback_active = ? WHERE id = ?;

-- name: DeleteApplication :exec
DELETE FROM applications WHERE id = ?;

-- ============================================================
-- ROLLBACKS
-- ============================================================

-- name: GetRollbackByID :one
SELECT * FROM rollbacks WHERE id = ?;

-- name: ListRollbacksByDeployment :many
SELECT * FROM rollbacks WHERE deployment_id = ? ORDER BY version DESC;

-- name: GetLatestRollback :one
SELECT * FROM rollbacks
WHERE deployment_id = ?
ORDER BY version DESC
LIMIT 1;

-- name: CreateRollback :one
INSERT INTO rollbacks (deployment_id, version, image, full_context)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: DeleteRollback :exec
DELETE FROM rollbacks WHERE id = ?;

-- name: DeleteRollbacksByDeployment :exec
DELETE FROM rollbacks WHERE deployment_id = ?;

-- BEGIN GENERATED REPOSITORY QUERIES
-- Extracted from src/db/repository/applications.rs. Do not edit this generated section directly.

-- name: ApplicationsUpdateNetworks :exec
UPDATE applications SET network_ids = ?, detach_rustploy_network = ? WHERE id = ?;

-- name: ApplicationsMoveToEnvironmentInSameOrganization :exec
UPDATE applications
               SET environment_id = ?, updated_at = strftime('%s', 'now')
               WHERE id = ?
                 AND EXISTS (
                     SELECT 1
                     FROM environments source_environment
                     INNER JOIN projects source_project ON source_project.id = source_environment.project_id
                     INNER JOIN environments target_environment ON target_environment.id = ?
                     INNER JOIN projects target_project ON target_project.id = target_environment.project_id
                     WHERE source_environment.id = applications.environment_id
                       AND source_project.organization_id = target_project.organization_id
                 );

-- name: ApplicationsRotateRefreshToken :exec
UPDATE applications SET refresh_token = ? WHERE id = ?;

-- name: ApplicationsUpdatePreviewConfig :exec
UPDATE applications SET preview_env = ?, preview_build_args = ?, preview_build_secrets = ?, preview_labels = ?, preview_wildcard = ?, preview_port = ?, preview_https = ?, preview_path = ?, preview_certificate_type = ?, preview_custom_cert_resolver = ?, preview_limit = ?, is_preview_deployments_active = ?, preview_require_collaborator_permissions = ? WHERE id = ?;

-- name: ApplicationsGetAll :many
SELECT id AS id, name AS name, app_name AS app_name, description AS description, source_type AS source_type, build_type AS build_type, app_status AS app_status, last_error AS last_error, trigger_type AS trigger_type, build_args AS build_args, build_secrets AS build_secrets, dockerfile AS dockerfile, docker_context_path AS docker_context_path, docker_build_stage AS docker_build_stage, publish_directory AS publish_directory, is_static_spa AS is_static_spa, create_env_file AS create_env_file, railpack_version AS railpack_version, heroku_version AS heroku_version, command AS command, args AS args, env_var AS env_var, build_path AS build_path, clean_cache AS clean_cache, drop_build_path AS drop_build_path, enable_submodules AS enable_submodules, watch_paths AS watch_paths, refresh_token AS refresh_token, icon AS icon, network_ids AS network_ids, detach_rustploy_network AS detach_rustploy_network, memory_reservation AS memory_reservation, memory_limit AS memory_limit, cpu_reservation AS cpu_reservation, cpu_limit AS cpu_limit, replicas AS replicas, health_check_swarm AS health_check_swarm, restart_policy_swarm AS restart_policy_swarm, placement_swarm AS placement_swarm, update_config_swarm AS update_config_swarm, rollback_config_swarm AS rollback_config_swarm, mode_swarm AS mode_swarm, labels_swarm AS labels_swarm, network_swarm AS network_swarm, endpoint_spec_swarm AS endpoint_spec_swarm, ulimits_swarm AS ulimits_swarm, stop_grace_period_swarm AS stop_grace_period_swarm, repository AS repository, owner AS owner, branch AS branch, auto_deploy AS auto_deploy, gitlab_project_id AS gitlab_project_id, gitlab_repository AS gitlab_repository, gitlab_owner AS gitlab_owner, gitlab_branch AS gitlab_branch, gitlab_build_path AS gitlab_build_path, gitlab_path_namespace AS gitlab_path_namespace, gitea_repository AS gitea_repository, gitea_owner AS gitea_owner, gitea_branch AS gitea_branch, gitea_build_path AS gitea_build_path, bitbucket_repository AS bitbucket_repository, bitbucket_repository_slug AS bitbucket_repository_slug, bitbucket_owner AS bitbucket_owner, bitbucket_branch AS bitbucket_branch, bitbucket_build_path AS bitbucket_build_path, docker_image AS docker_image, docker_username AS docker_username, docker_password AS docker_password, registry_url AS registry_url, custom_git_url AS custom_git_url, custom_git_branch AS custom_git_branch, custom_git_build_path AS custom_git_build_path, custom_git_ssh_key_id AS custom_git_ssh_key_id, preview_env AS preview_env, preview_build_args AS preview_build_args, preview_build_secrets AS preview_build_secrets, preview_labels AS preview_labels, preview_wildcard AS preview_wildcard, preview_port AS preview_port, preview_https AS preview_https, preview_path AS preview_path, preview_certificate_type AS preview_certificate_type, preview_custom_cert_resolver AS preview_custom_cert_resolver, preview_limit AS preview_limit, is_preview_deployments_active AS is_preview_deployments_active, preview_require_collaborator_permissions AS preview_require_collaborator_permissions, rollback_active AS rollback_active, environment_id AS environment_id, server_id AS server_id, build_server_id AS build_server_id, registry_id AS registry_id, rollback_registry_id AS rollback_registry_id, build_registry_id AS build_registry_id, github_provider_id AS github_provider_id, gitlab_provider_id AS gitlab_provider_id, gitea_provider_id AS gitea_provider_id, bitbucket_provider_id AS bitbucket_provider_id, created_at AS created_at, updated_at AS updated_at FROM applications;

-- name: ApplicationsGetById :one
SELECT id AS id, name AS name, app_name AS app_name, description AS description, source_type AS source_type, build_type AS build_type, app_status AS app_status, last_error AS last_error, trigger_type AS trigger_type, build_args AS build_args, build_secrets AS build_secrets, dockerfile AS dockerfile, docker_context_path AS docker_context_path, docker_build_stage AS docker_build_stage, publish_directory AS publish_directory, is_static_spa AS is_static_spa, create_env_file AS create_env_file, railpack_version AS railpack_version, heroku_version AS heroku_version, command AS command, args AS args, env_var AS env_var, build_path AS build_path, clean_cache AS clean_cache, drop_build_path AS drop_build_path, enable_submodules AS enable_submodules, watch_paths AS watch_paths, refresh_token AS refresh_token, icon AS icon, network_ids AS network_ids, detach_rustploy_network AS detach_rustploy_network, memory_reservation AS memory_reservation, memory_limit AS memory_limit, cpu_reservation AS cpu_reservation, cpu_limit AS cpu_limit, replicas AS replicas, health_check_swarm AS health_check_swarm, restart_policy_swarm AS restart_policy_swarm, placement_swarm AS placement_swarm, update_config_swarm AS update_config_swarm, rollback_config_swarm AS rollback_config_swarm, mode_swarm AS mode_swarm, labels_swarm AS labels_swarm, network_swarm AS network_swarm, endpoint_spec_swarm AS endpoint_spec_swarm, ulimits_swarm AS ulimits_swarm, stop_grace_period_swarm AS stop_grace_period_swarm, repository AS repository, owner AS owner, branch AS branch, auto_deploy AS auto_deploy, gitlab_project_id AS gitlab_project_id, gitlab_repository AS gitlab_repository, gitlab_owner AS gitlab_owner, gitlab_branch AS gitlab_branch, gitlab_build_path AS gitlab_build_path, gitlab_path_namespace AS gitlab_path_namespace, gitea_repository AS gitea_repository, gitea_owner AS gitea_owner, gitea_branch AS gitea_branch, gitea_build_path AS gitea_build_path, bitbucket_repository AS bitbucket_repository, bitbucket_repository_slug AS bitbucket_repository_slug, bitbucket_owner AS bitbucket_owner, bitbucket_branch AS bitbucket_branch, bitbucket_build_path AS bitbucket_build_path, docker_image AS docker_image, docker_username AS docker_username, docker_password AS docker_password, registry_url AS registry_url, custom_git_url AS custom_git_url, custom_git_branch AS custom_git_branch, custom_git_build_path AS custom_git_build_path, custom_git_ssh_key_id AS custom_git_ssh_key_id, preview_env AS preview_env, preview_build_args AS preview_build_args, preview_build_secrets AS preview_build_secrets, preview_labels AS preview_labels, preview_wildcard AS preview_wildcard, preview_port AS preview_port, preview_https AS preview_https, preview_path AS preview_path, preview_certificate_type AS preview_certificate_type, preview_custom_cert_resolver AS preview_custom_cert_resolver, preview_limit AS preview_limit, is_preview_deployments_active AS is_preview_deployments_active, preview_require_collaborator_permissions AS preview_require_collaborator_permissions, rollback_active AS rollback_active, environment_id AS environment_id, server_id AS server_id, build_server_id AS build_server_id, registry_id AS registry_id, rollback_registry_id AS rollback_registry_id, build_registry_id AS build_registry_id, github_provider_id AS github_provider_id, gitlab_provider_id AS gitlab_provider_id, gitea_provider_id AS gitea_provider_id, bitbucket_provider_id AS bitbucket_provider_id, created_at AS created_at, updated_at AS updated_at FROM applications WHERE id = ?;

-- name: ApplicationsCreate :exec
INSERT INTO applications (name, app_name, description, source_type, build_type, app_status, trigger_type, build_args, build_secrets, dockerfile, docker_context_path, docker_build_stage, publish_directory, is_static_spa, create_env_file, railpack_version, heroku_version, command, args, env_var, build_path, clean_cache, drop_build_path, enable_submodules, watch_paths, refresh_token, icon, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas, health_check_swarm, restart_policy_swarm, placement_swarm, update_config_swarm, rollback_config_swarm, mode_swarm, labels_swarm, network_swarm, endpoint_spec_swarm, ulimits_swarm, stop_grace_period_swarm, repository, owner, branch, auto_deploy, gitlab_project_id, gitlab_repository, gitlab_owner, gitlab_branch, gitlab_build_path, gitlab_path_namespace, gitea_repository, gitea_owner, gitea_branch, gitea_build_path, bitbucket_repository, bitbucket_repository_slug, bitbucket_owner, bitbucket_branch, bitbucket_build_path, docker_image, docker_username, docker_password, registry_url, custom_git_url, custom_git_branch, custom_git_build_path, custom_git_ssh_key_id, preview_env, preview_build_args, preview_build_secrets, preview_labels, preview_wildcard, preview_port, preview_https, preview_path, preview_certificate_type, preview_custom_cert_resolver, preview_limit, is_preview_deployments_active, preview_require_collaborator_permissions, rollback_active, environment_id, server_id, build_server_id, registry_id, rollback_registry_id, build_registry_id, github_provider_id, gitlab_provider_id, gitea_provider_id, bitbucket_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: ApplicationsUpdate :exec
UPDATE applications SET name = ?, app_name = ?, description = ?, source_type = ?, build_type = ?, app_status = ?, trigger_type = ?, build_args = ?, build_secrets = ?, dockerfile = ?, docker_context_path = ?, docker_build_stage = ?, publish_directory = ?, is_static_spa = ?, create_env_file = ?, railpack_version = ?, heroku_version = ?, command = ?, args = ?, env_var = ?, build_path = ?, clean_cache = ?, drop_build_path = ?, enable_submodules = ?, watch_paths = ?, refresh_token = ?, icon = ?, memory_reservation = ?, memory_limit = ?, cpu_reservation = ?, cpu_limit = ?, replicas = ?, health_check_swarm = ?, restart_policy_swarm = ?, placement_swarm = ?, update_config_swarm = ?, rollback_config_swarm = ?, mode_swarm = ?, labels_swarm = ?, network_swarm = ?, endpoint_spec_swarm = ?, ulimits_swarm = ?, stop_grace_period_swarm = ?, repository = ?, owner = ?, branch = ?, auto_deploy = ?, gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?, gitlab_build_path = ?, gitlab_path_namespace = ?, gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_build_path = ?, bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?, bitbucket_branch = ?, bitbucket_build_path = ?, docker_image = ?, docker_username = ?, docker_password = ?, registry_url = ?, custom_git_url = ?, custom_git_branch = ?, custom_git_build_path = ?, custom_git_ssh_key_id = ?, preview_env = ?, preview_build_args = ?, preview_build_secrets = ?, preview_labels = ?, preview_wildcard = ?, preview_port = ?, preview_https = ?, preview_path = ?, preview_certificate_type = ?, preview_custom_cert_resolver = ?, preview_limit = ?, is_preview_deployments_active = ?, preview_require_collaborator_permissions = ?, rollback_active = ?, environment_id = ?, server_id = ?, build_server_id = ?, registry_id = ?, rollback_registry_id = ?, build_registry_id = ?, github_provider_id = ?, gitlab_provider_id = ?, gitea_provider_id = ?, bitbucket_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: ApplicationsDelete :exec
DELETE FROM applications WHERE id = ?;

-- name: ApplicationsListByEnvironment :many
SELECT id AS id, name AS name, app_name AS app_name, description AS description, source_type AS source_type, build_type AS build_type, app_status AS app_status, last_error AS last_error, trigger_type AS trigger_type, build_args AS build_args, build_secrets AS build_secrets, dockerfile AS dockerfile, docker_context_path AS docker_context_path, docker_build_stage AS docker_build_stage, publish_directory AS publish_directory, is_static_spa AS is_static_spa, create_env_file AS create_env_file, railpack_version AS railpack_version, heroku_version AS heroku_version, command AS command, args AS args, env_var AS env_var, build_path AS build_path, clean_cache AS clean_cache, drop_build_path AS drop_build_path, enable_submodules AS enable_submodules, watch_paths AS watch_paths, refresh_token AS refresh_token, icon AS icon, network_ids AS network_ids, detach_rustploy_network AS detach_rustploy_network, memory_reservation AS memory_reservation, memory_limit AS memory_limit, cpu_reservation AS cpu_reservation, cpu_limit AS cpu_limit, replicas AS replicas, health_check_swarm AS health_check_swarm, restart_policy_swarm AS restart_policy_swarm, placement_swarm AS placement_swarm, update_config_swarm AS update_config_swarm, rollback_config_swarm AS rollback_config_swarm, mode_swarm AS mode_swarm, labels_swarm AS labels_swarm, network_swarm AS network_swarm, endpoint_spec_swarm AS endpoint_spec_swarm, ulimits_swarm AS ulimits_swarm, stop_grace_period_swarm AS stop_grace_period_swarm, repository AS repository, owner AS owner, branch AS branch, auto_deploy AS auto_deploy, gitlab_project_id AS gitlab_project_id, gitlab_repository AS gitlab_repository, gitlab_owner AS gitlab_owner, gitlab_branch AS gitlab_branch, gitlab_build_path AS gitlab_build_path, gitlab_path_namespace AS gitlab_path_namespace, gitea_repository AS gitea_repository, gitea_owner AS gitea_owner, gitea_branch AS gitea_branch, gitea_build_path AS gitea_build_path, bitbucket_repository AS bitbucket_repository, bitbucket_repository_slug AS bitbucket_repository_slug, bitbucket_owner AS bitbucket_owner, bitbucket_branch AS bitbucket_branch, bitbucket_build_path AS bitbucket_build_path, docker_image AS docker_image, docker_username AS docker_username, docker_password AS docker_password, registry_url AS registry_url, custom_git_url AS custom_git_url, custom_git_branch AS custom_git_branch, custom_git_build_path AS custom_git_build_path, custom_git_ssh_key_id AS custom_git_ssh_key_id, preview_env AS preview_env, preview_build_args AS preview_build_args, preview_build_secrets AS preview_build_secrets, preview_labels AS preview_labels, preview_wildcard AS preview_wildcard, preview_port AS preview_port, preview_https AS preview_https, preview_path AS preview_path, preview_certificate_type AS preview_certificate_type, preview_custom_cert_resolver AS preview_custom_cert_resolver, preview_limit AS preview_limit, is_preview_deployments_active AS is_preview_deployments_active, preview_require_collaborator_permissions AS preview_require_collaborator_permissions, rollback_active AS rollback_active, environment_id AS environment_id, server_id AS server_id, build_server_id AS build_server_id, registry_id AS registry_id, rollback_registry_id AS rollback_registry_id, build_registry_id AS build_registry_id, github_provider_id AS github_provider_id, gitlab_provider_id AS gitlab_provider_id, gitea_provider_id AS gitea_provider_id, bitbucket_provider_id AS bitbucket_provider_id, created_at AS created_at, updated_at AS updated_at FROM applications WHERE environment_id = ? ORDER BY created_at DESC, id DESC;

-- name: ApplicationsCreateSimple :exec
INSERT INTO applications (name, app_name, description, source_type, build_type, environment_id, server_id) VALUES (?, ?, ?, ?, ?, ?, ?);

-- name: ApplicationsPatch :exec
UPDATE applications SET name = ?, description = ?, build_type = ?, trigger_type = ?, env_var = ?, icon = ?, server_id = ?, build_server_id = ?, registry_id = ?, network_ids = ?, detach_rustploy_network = ? WHERE id = ?;

-- name: ApplicationsPatchBuildConfig :exec
UPDATE applications SET
               build_args = COALESCE(?, build_args), build_secrets = COALESCE(?, build_secrets),
               dockerfile = COALESCE(?, dockerfile), docker_context_path = COALESCE(?, docker_context_path),
               docker_build_stage = COALESCE(?, docker_build_stage), publish_directory = COALESCE(?, publish_directory),
               is_static_spa = COALESCE(?, is_static_spa), create_env_file = COALESCE(?, create_env_file),
               railpack_version = COALESCE(?, railpack_version), heroku_version = COALESCE(?, heroku_version),
               command = COALESCE(?, command), args = COALESCE(?, args), build_path = COALESCE(?, build_path),
               clean_cache = COALESCE(?, clean_cache), enable_submodules = COALESCE(?, enable_submodules),
               watch_paths = COALESCE(?, watch_paths)
               WHERE id = ?;

-- name: ApplicationsPatchResourceConfig :exec
UPDATE applications SET
               memory_reservation = ?, memory_limit = ?,
               cpu_reservation = ?, cpu_limit = ?,
               replicas = COALESCE(?, replicas)
               WHERE id = ?;

-- name: ApplicationsSetGithubSource :exec
UPDATE applications SET
               source_type = 'GITHUB',
               repository = ?, owner = ?, branch = ?, build_path = ?, github_provider_id = ?, auto_deploy = ?,
               gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
               gitlab_path_namespace = NULL, gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?;

-- name: ApplicationsSetGitlabSource :exec
UPDATE applications SET
               source_type = 'GITLAB',
               gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?,
               gitlab_build_path = ?, gitlab_path_namespace = ?, gitlab_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitea_repository = NULL, gitea_owner = NULL,
               gitea_branch = NULL, bitbucket_repository = NULL, bitbucket_repository_slug = NULL,
               bitbucket_owner = NULL, bitbucket_branch = NULL, docker_image = NULL, docker_username = NULL,
               docker_password = NULL, registry_url = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?;

-- name: ApplicationsSetGiteaSource :exec
UPDATE applications SET
               source_type = 'GITEA',
               gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_build_path = ?, gitea_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?;

-- name: ApplicationsSetBitbucketSource :exec
UPDATE applications SET
               source_type = 'BITBUCKET',
               bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?,
               bitbucket_branch = ?, bitbucket_build_path = ?, bitbucket_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?;

-- name: ApplicationsSetDockerSource :exec
UPDATE applications SET
               source_type = 'DOCKER',
               docker_image = ?, docker_username = ?, docker_password = ?, registry_url = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?;

-- name: ApplicationsSetCustomGitSource :exec
UPDATE applications SET
               source_type = 'GIT',
               custom_git_url = ?, custom_git_branch = ?, custom_git_build_path = ?, custom_git_ssh_key_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL, drop_build_path = NULL
               WHERE id = ?;

-- name: ApplicationsSetDropSource :exec
UPDATE applications SET
               source_type = 'DROP',
               drop_build_path = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?;

-- name: ApplicationsUpdateStatus :exec
UPDATE applications SET app_status = ?, last_error = NULL WHERE id = ?;

-- name: ApplicationsGetDeploymentContext :one
SELECT a.environment_id, e.project_id, a.server_id
                FROM applications a
                JOIN environments e ON e.id = a.environment_id
                WHERE a.id = ?;

-- name: ApplicationsBelongsToOrganization :one
SELECT EXISTS(SELECT 1 FROM applications a JOIN environments e ON e.id=a.environment_id JOIN projects p ON p.id=e.project_id WHERE a.id=? AND p.organization_id=?);

-- name: ApplicationsGetSpecRow :one
SELECT
                a.app_name, a.source_type, a.build_type, a.build_args, a.build_secrets,
                a.dockerfile, a.docker_context_path, a.docker_build_stage,
                a.publish_directory, a.is_static_spa, a.build_path, a.command, a.args, a.env_var,
                a.clean_cache, a.network_ids, a.detach_rustploy_network,
                a.memory_reservation, a.memory_limit,
                a.cpu_reservation, a.cpu_limit, a.replicas, a.health_check_swarm,
                a.placement_swarm, a.stop_grace_period_swarm, a.repository, a.owner,
                a.branch, a.gitlab_repository, a.gitlab_owner, a.gitlab_branch,
                a.gitea_repository, a.gitea_branch, a.bitbucket_repository,
                a.bitbucket_owner, a.bitbucket_branch, a.docker_image, a.docker_username,
                a.docker_password, a.registry_url, a.custom_git_url, a.custom_git_branch,
                a.custom_git_ssh_key_id,
                e.env_var AS environment_env, p.env_var AS project_env,
                r.image_prefix AS registry_image_prefix, r.username AS registry_username,
                r.password AS registry_password, r.registry_url AS joined_registry_url,
                ghp.github_private_key AS github_token,
                glp.access_token AS gitlab_token,
                bbp.api_token AS bitbucket_token,
                gtp.access_token AS gitea_token,
                sk.private_key AS ssh_private_key
                FROM applications a
                JOIN environments e ON e.id = a.environment_id
                JOIN projects p ON p.id = e.project_id
                LEFT JOIN registries r ON r.id = a.registry_id
                LEFT JOIN github_providers ghp ON ghp.id = a.github_provider_id
                LEFT JOIN gitlab_providers glp ON glp.id = a.gitlab_provider_id
                LEFT JOIN bitbucket_providers bbp ON bbp.id = a.bitbucket_provider_id
                LEFT JOIN gitea_providers gtp ON gtp.id = a.gitea_provider_id
                LEFT JOIN ssh_keys sk ON sk.id = a.custom_git_ssh_key_id
                WHERE a.id = ?;

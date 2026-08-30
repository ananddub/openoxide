-- Preview-environment extras owned by the preview service migration.
-- Query names are globally unique and prefixed with PrevEnv.
--
-- The old repository cloned a flat `applications` row for each preview. In the
-- unified schema an application is (services, service_git, service_application),
-- so a preview clone copies all three rows from the base service and then
-- overrides the preview-specific columns (name/app_name/app_status/env_var/
-- labels/auto_deploy/preview flags) plus the fork source fields from the event.
--
-- CAST(? AS BIGINT) is required for integer binds inside INSERT..SELECT: the
-- generator otherwise infers String for positionally untyped parameters.

-- name: PrevEnvCloneService :one
INSERT INTO services (
    service_type, name, app_name, description, icon, app_status, last_error, env_var,
    command, args, refresh_token, docker_image, memory_reservation, memory_limit,
    cpu_reservation, cpu_limit, replicas, health_check_swarm, restart_policy_swarm,
    placement_swarm, update_config_swarm, rollback_config_swarm, mode_swarm, labels_swarm,
    network_swarm, endpoint_spec_swarm, ulimits_swarm, stop_grace_period_swarm, network_ids,
    detach_rustploy_network, environment_id, server_id, created_at, updated_at
)
SELECT
    base.service_type, ?, ?, base.description, base.icon, 'IDLE', base.last_error, ?,
    base.command, base.args, base.refresh_token, base.docker_image, base.memory_reservation, base.memory_limit,
    base.cpu_reservation, base.cpu_limit, base.replicas, base.health_check_swarm, base.restart_policy_swarm,
    base.placement_swarm, base.update_config_swarm, base.rollback_config_swarm, base.mode_swarm,
    COALESCE(sa.preview_labels, base.labels_swarm),
    base.network_swarm, base.endpoint_spec_swarm, base.ulimits_swarm, base.stop_grace_period_swarm, base.network_ids,
    base.detach_rustploy_network, base.environment_id, base.server_id, CAST(? AS BIGINT), CAST(? AS BIGINT)
FROM services base
JOIN service_application sa ON sa.service_id = base.id
WHERE base.id = ?
RETURNING id AS id;

-- name: PrevEnvCloneServiceGit :exec
INSERT INTO service_git (
    service_id, source_type, trigger_type, auto_deploy, watch_paths, enable_submodules,
    repository, owner, branch, gitlab_project_id, gitlab_repository, gitlab_owner, gitlab_branch,
    gitlab_build_path, gitlab_path_namespace, gitea_repository, gitea_owner, gitea_branch,
    gitea_build_path, bitbucket_repository, bitbucket_repository_slug, bitbucket_owner,
    bitbucket_branch, bitbucket_build_path, custom_git_url, custom_git_branch, custom_git_build_path,
    custom_git_ssh_key_id, github_provider_id, gitlab_provider_id, gitea_provider_id,
    bitbucket_provider_id, created_at, updated_at
)
SELECT
    CAST(? AS BIGINT), source_type, trigger_type, 0, watch_paths, enable_submodules,
    repository, owner, branch, gitlab_project_id, gitlab_repository, gitlab_owner, gitlab_branch,
    gitlab_build_path, gitlab_path_namespace, gitea_repository, gitea_owner, gitea_branch,
    gitea_build_path, bitbucket_repository, bitbucket_repository_slug, bitbucket_owner,
    bitbucket_branch, bitbucket_build_path, custom_git_url, custom_git_branch, custom_git_build_path,
    custom_git_ssh_key_id, github_provider_id, gitlab_provider_id, gitea_provider_id,
    bitbucket_provider_id, CAST(? AS BIGINT), CAST(? AS BIGINT)
FROM service_git
WHERE service_id = ?;

-- name: PrevEnvCloneServiceApplication :exec
INSERT INTO service_application (
    service_id, build_type, build_args, build_secrets, dockerfile, docker_context_path,
    docker_build_stage, publish_directory, is_static_spa, create_env_file, railpack_version,
    heroku_version, build_path, clean_cache, drop_build_path, docker_username, docker_password,
    registry_url, build_server_id, registry_id, rollback_registry_id, build_registry_id,
    rollback_active, preview_env, preview_build_args, preview_build_secrets, preview_labels,
    preview_wildcard, preview_port, preview_https, preview_path, preview_certificate_type,
    preview_custom_cert_resolver, preview_limit, is_preview_deployments_active,
    preview_require_collaborator_permissions
)
SELECT
    CAST(? AS BIGINT), build_type, COALESCE(preview_build_args, build_args), COALESCE(preview_build_secrets, build_secrets),
    dockerfile, docker_context_path, docker_build_stage, publish_directory, is_static_spa, create_env_file,
    railpack_version, heroku_version, build_path, clean_cache, drop_build_path, docker_username,
    docker_password, registry_url, build_server_id, registry_id, rollback_registry_id, build_registry_id,
    rollback_active, preview_env, preview_build_args, preview_build_secrets, preview_labels,
    preview_wildcard, preview_port, preview_https, preview_path, preview_certificate_type,
    preview_custom_cert_resolver, preview_limit, 0, 0
FROM service_application
WHERE service_id = ?;

-- name: PrevEnvSyncService :exec
UPDATE services AS preview
SET
    service_type = base.service_type,
    name = ?,
    app_name = ?,
    description = base.description,
    icon = base.icon,
    app_status = 'IDLE',
    last_error = base.last_error,
    env_var = ?,
    command = base.command,
    args = base.args,
    refresh_token = base.refresh_token,
    docker_image = base.docker_image,
    memory_reservation = base.memory_reservation,
    memory_limit = base.memory_limit,
    cpu_reservation = base.cpu_reservation,
    cpu_limit = base.cpu_limit,
    replicas = base.replicas,
    health_check_swarm = base.health_check_swarm,
    restart_policy_swarm = base.restart_policy_swarm,
    placement_swarm = base.placement_swarm,
    update_config_swarm = base.update_config_swarm,
    rollback_config_swarm = base.rollback_config_swarm,
    mode_swarm = base.mode_swarm,
    labels_swarm = COALESCE(base_app.preview_labels, base.labels_swarm),
    network_swarm = base.network_swarm,
    endpoint_spec_swarm = base.endpoint_spec_swarm,
    ulimits_swarm = base.ulimits_swarm,
    stop_grace_period_swarm = base.stop_grace_period_swarm,
    network_ids = base.network_ids,
    detach_rustploy_network = base.detach_rustploy_network,
    environment_id = base.environment_id,
    server_id = base.server_id,
    created_at = ?,
    updated_at = ?
FROM services AS base
JOIN service_application AS base_app ON base_app.service_id = base.id
WHERE preview.id = ? AND base.id = ?;

-- name: PrevEnvSyncServiceGit :exec
UPDATE service_git AS preview
SET
    source_type = base.source_type,
    trigger_type = base.trigger_type,
    auto_deploy = base.auto_deploy,
    watch_paths = base.watch_paths,
    enable_submodules = base.enable_submodules,
    repository = base.repository,
    owner = base.owner,
    branch = base.branch,
    gitlab_project_id = base.gitlab_project_id,
    gitlab_repository = base.gitlab_repository,
    gitlab_owner = base.gitlab_owner,
    gitlab_branch = base.gitlab_branch,
    gitlab_build_path = base.gitlab_build_path,
    gitlab_path_namespace = base.gitlab_path_namespace,
    gitea_repository = base.gitea_repository,
    gitea_owner = base.gitea_owner,
    gitea_branch = base.gitea_branch,
    gitea_build_path = base.gitea_build_path,
    bitbucket_repository = base.bitbucket_repository,
    bitbucket_repository_slug = base.bitbucket_repository_slug,
    bitbucket_owner = base.bitbucket_owner,
    bitbucket_branch = base.bitbucket_branch,
    bitbucket_build_path = base.bitbucket_build_path,
    custom_git_url = base.custom_git_url,
    custom_git_branch = base.custom_git_branch,
    custom_git_build_path = base.custom_git_build_path,
    custom_git_ssh_key_id = base.custom_git_ssh_key_id,
    github_provider_id = base.github_provider_id,
    gitlab_provider_id = base.gitlab_provider_id,
    gitea_provider_id = base.gitea_provider_id,
    bitbucket_provider_id = base.bitbucket_provider_id
FROM service_git AS base
WHERE preview.service_id = ? AND base.service_id = ?;

-- name: PrevEnvSyncServiceApplication :exec
UPDATE service_application AS preview
SET
    build_type = base.build_type,
    build_args = COALESCE(base.preview_build_args, base.build_args),
    build_secrets = COALESCE(base.preview_build_secrets, base.build_secrets),
    dockerfile = base.dockerfile,
    docker_context_path = base.docker_context_path,
    docker_build_stage = base.docker_build_stage,
    publish_directory = base.publish_directory,
    is_static_spa = base.is_static_spa,
    create_env_file = base.create_env_file,
    railpack_version = base.railpack_version,
    heroku_version = base.heroku_version,
    build_path = base.build_path,
    clean_cache = base.clean_cache,
    drop_build_path = base.drop_build_path,
    docker_username = base.docker_username,
    docker_password = base.docker_password,
    registry_url = base.registry_url,
    build_server_id = base.build_server_id,
    registry_id = base.registry_id,
    rollback_registry_id = base.rollback_registry_id,
    build_registry_id = base.build_registry_id,
    rollback_active = base.rollback_active,
    preview_env = base.preview_env,
    preview_build_args = base.preview_build_args,
    preview_build_secrets = base.preview_build_secrets,
    preview_labels = base.preview_labels,
    preview_wildcard = base.preview_wildcard,
    preview_port = base.preview_port,
    preview_https = base.preview_https,
    preview_path = base.preview_path,
    preview_certificate_type = base.preview_certificate_type,
    preview_custom_cert_resolver = base.preview_custom_cert_resolver,
    preview_limit = base.preview_limit,
    is_preview_deployments_active = 0,
    preview_require_collaborator_permissions = 0
FROM service_application AS base
WHERE preview.service_id = ? AND base.service_id = ?;

-- name: PrevEnvSetGithubSource :exec
UPDATE service_git SET repository = ?, owner = ?, branch = ?, auto_deploy = 0 WHERE service_id = ?;

-- name: PrevEnvSetGitlabSource :exec
UPDATE service_git SET gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?, auto_deploy = 0 WHERE service_id = ?;

-- name: PrevEnvSetGiteaSource :exec
UPDATE service_git SET gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, auto_deploy = 0 WHERE service_id = ?;

-- name: PrevEnvSetBitbucketSource :exec
UPDATE service_git SET bitbucket_repository = ?, bitbucket_owner = ?, bitbucket_branch = ?, auto_deploy = 0 WHERE service_id = ?;

-- name: PrevEnvCreateDomain :one
INSERT INTO domains
               (host, https, port, path, internal_path, custom_entrypoint, service_name,
                custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type,
                service_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS id, host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, service_id,
               created_at, updated_at;

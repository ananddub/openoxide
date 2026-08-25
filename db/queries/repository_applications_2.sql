-- Extracted from src/db/repository/applications.rs. Do not edit repository SQL directly.

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

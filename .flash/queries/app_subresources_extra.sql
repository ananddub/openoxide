-- Application sub-resource services (mounts, ports, redirects, security,
-- patches, middlewares, networks, import/export): extra queries on tables
-- owned by other Flash query files. All names are prefixed AppSub* and are
-- globally unique.

-- Application existence check scoped to APPLICATION services (the old
-- applications table only ever contained applications).

-- name: AppSubGetApplicationByID :one
SELECT * FROM services WHERE id = ? AND service_type = 'APPLICATION';

-- Joined APPLICATION record (services + service_application + service_git)
-- used by the application transfer (import/export) service to rebuild an
-- ApplicationRecord after import. Mirrors DatabaseGetById for the database
-- domain. Git-less sources (DOCKER/DROP) resolve via docker_image /
-- drop_build_path like ApplicationGetSpecRow does.

-- name: AppSubGetApplicationRecord :one
SELECT
    s.id AS id,
    s.name AS name,
    s.app_name AS app_name,
    s.description AS description,
    COALESCE(g.source_type,
        CASE WHEN s.docker_image IS NOT NULL THEN 'DOCKER'
             WHEN a.drop_build_path IS NOT NULL THEN 'DROP'
             ELSE 'NONE' END) AS source_type,
    a.build_type AS build_type,
    s.app_status AS app_status,
    s.last_error AS last_error,
    COALESCE(g.trigger_type, 'PUSH') AS trigger_type,
    s.environment_id AS environment_id,
    s.server_id AS server_id,
    a.build_server_id AS build_server_id,
    a.registry_id AS registry_id,
    s.network_ids AS network_ids,
    s.detach_rustploy_network AS detach_rustploy_network,
    s.env_var AS env_var,
    s.icon AS icon,
    g.repository AS repository,
    g.owner AS owner,
    g.branch AS branch,
    g.gitlab_repository AS gitlab_repository,
    g.gitlab_owner AS gitlab_owner,
    g.gitlab_branch AS gitlab_branch,
    g.gitea_repository AS gitea_repository,
    g.gitea_owner AS gitea_owner,
    g.gitea_branch AS gitea_branch,
    g.bitbucket_repository AS bitbucket_repository,
    g.bitbucket_owner AS bitbucket_owner,
    g.bitbucket_branch AS bitbucket_branch,
    s.docker_image AS docker_image,
    a.registry_url AS registry_url,
    g.custom_git_url AS custom_git_url,
    g.custom_git_branch AS custom_git_branch,
    a.preview_env AS preview_env,
    a.preview_build_args AS preview_build_args,
    a.preview_build_secrets AS preview_build_secrets,
    a.preview_labels AS preview_labels,
    a.preview_wildcard AS preview_wildcard,
    a.preview_port AS preview_port,
    a.preview_https AS preview_https,
    a.preview_path AS preview_path,
    a.preview_certificate_type AS preview_certificate_type,
    a.preview_custom_cert_resolver AS preview_custom_cert_resolver,
    a.preview_limit AS preview_limit,
    a.is_preview_deployments_active AS is_preview_deployments_active,
    a.preview_require_collaborator_permissions AS preview_require_collaborator_permissions,
    s.memory_reservation AS memory_reservation,
    s.memory_limit AS memory_limit,
    s.cpu_reservation AS cpu_reservation,
    s.cpu_limit AS cpu_limit,
    s.replicas AS replicas,
    s.created_at AS created_at,
    s.updated_at AS updated_at
FROM services s
JOIN service_application a ON a.service_id = s.id
LEFT JOIN service_git g ON g.service_id = s.id
WHERE s.id = ? AND s.service_type = 'APPLICATION';

-- Compose-specific config for COMPOSE services.
-- Base fields live in services; git source config in service_git.

-- name: GetServiceCompose :one
SELECT * FROM service_compose WHERE service_id = ?;

-- name: CreateServiceCompose :one
INSERT INTO service_compose (
    service_id, compose_type, compose_file, compose_path, suffix, randomize,
    isolated_deployment, isolated_deployments_volume, service_networks
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: CreateServiceComposeSimple :exec
INSERT INTO service_compose (service_id, compose_type, compose_file, compose_path, suffix, randomize, isolated_deployment, isolated_deployments_volume, service_networks)
VALUES (?, ?, ?, '', '', 0, 0, 0, '[]');

-- name: DeleteServiceCompose :exec
DELETE FROM service_compose WHERE service_id = ?;

-- name: UpdateServiceCompose :one
UPDATE service_compose
SET compose_type = ?, compose_file = ?, compose_path = ?, suffix = ?, randomize = ?,
    isolated_deployment = ?, isolated_deployments_volume = ?, service_networks = ?
WHERE service_id = ?
RETURNING *;

-- name: PatchServiceCompose :exec
UPDATE service_compose
SET compose_file = ?, compose_type = ?, compose_path = ?,
    suffix = COALESCE(?, suffix), randomize = COALESCE(?, randomize),
    isolated_deployment = COALESCE(?, isolated_deployment),
    isolated_deployments_volume = COALESCE(?, isolated_deployments_volume),
    service_networks = ?
WHERE service_id = ?;

-- name: UpdateComposeFile :exec
UPDATE service_compose SET compose_file = ? WHERE service_id = ?;

-- name: ComposeGetDeploymentContext :one
SELECT s.environment_id, e.project_id, s.server_id
FROM services s
JOIN environments e ON e.id = s.environment_id
WHERE s.id = ?;

-- Switch a compose service to an inline (RAW) source: git source row is
-- removed, the file content lives in service_compose.compose_file.

-- name: ComposeSetRawSource :exec
DELETE FROM service_git WHERE service_id = ?;

-- name: ComposeSetRawSourceContent :exec
UPDATE service_compose SET compose_file = ? WHERE service_id = ?;

-- Full deploy spec for a COMPOSE service (base + git + provider tokens).
-- source_type: service_git for git sources, else RAW by stored file or DROP.

-- name: ComposeGetSpecRow :one
SELECT
    s.app_name AS app_name,
    COALESCE(g.source_type,
        CASE WHEN c.compose_file != '' THEN 'RAW' ELSE 'DROP' END) AS source_type,
    c.compose_type AS compose_type,
    c.compose_file AS compose_file,
    s.env_var AS env_var,
    c.service_networks AS service_networks,
    c.randomize AS randomize,
    c.suffix AS suffix,
    c.isolated_deployment AS isolated_deployment,
    c.isolated_deployments_volume AS isolated_deployments_volume,
    g.repository AS repository,
    g.owner AS owner,
    g.branch AS branch,
    g.gitlab_repository AS gitlab_repository,
    g.gitlab_owner AS gitlab_owner,
    g.gitlab_branch AS gitlab_branch,
    g.gitea_repository AS gitea_repository,
    g.gitea_branch AS gitea_branch,
    g.bitbucket_repository AS bitbucket_repository,
    g.bitbucket_owner AS bitbucket_owner,
    g.bitbucket_branch AS bitbucket_branch,
    g.custom_git_url AS custom_git_url,
    g.custom_git_branch AS custom_git_branch,
    g.custom_git_ssh_key_id AS custom_git_ssh_key_id,
    g.enable_submodules AS enable_submodules,
    c.compose_path AS compose_path,
    e.env_var AS environment_env,
    p.env_var AS project_env,
    gpgh.config_json AS github_config,
    gpgl.config_json AS gitlab_config,
    gpbb.config_json AS bitbucket_config,
    gpgt.config_json AS gitea_config,
    sk.private_key AS ssh_private_key
FROM services s
JOIN service_compose c ON c.service_id = s.id
JOIN environments e ON e.id = s.environment_id
JOIN projects p ON p.id = e.project_id
LEFT JOIN service_git g ON g.service_id = s.id
LEFT JOIN git_providers gpgh ON gpgh.id = g.github_provider_id
LEFT JOIN git_providers gpgl ON gpgl.id = g.gitlab_provider_id
LEFT JOIN git_providers gpbb ON gpbb.id = g.bitbucket_provider_id
LEFT JOIN git_providers gpgt ON gpgt.id = g.gitea_provider_id
LEFT JOIN ssh_keys sk ON sk.id = g.custom_git_ssh_key_id
WHERE s.id = ?;

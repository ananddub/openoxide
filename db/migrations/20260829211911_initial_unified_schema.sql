-- Migration: initial unified schema
-- Created: 2026-08-29T21:19:11Z

-- +migrate Up
-- ==== ai_settings.sql ====
CREATE TABLE ai_settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    api_url TEXT NOT NULL,
    api_key TEXT NOT NULL,
    model TEXT NOT NULL,
    is_enabled INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== alert_events.sql ====
CREATE TABLE alert_events (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    alert_rule_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    target_key TEXT NOT NULL,
    state TEXT NOT NULL,
    value REAL,
    threshold REAL,
    message TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    acknowledged_at INTEGER,
    acknowledged_by INTEGER,
    silenced_until INTEGER,
    resolved_at INTEGER,
    notification_correlation_id TEXT
) STRICT;

-- ==== alert_rules.sql ====
CREATE TABLE alert_rules (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id INTEGER NOT NULL,
    metric_name TEXT NOT NULL,
    operator TEXT NOT NULL,
    threshold REAL NOT NULL,
    duration_seconds INTEGER NOT NULL,
    notification_channel TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== audit_logs.sql ====
CREATE TABLE audit_logs (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_email TEXT NOT NULL,
    user_role TEXT NOT NULL,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    resource_name TEXT,
    metadata TEXT,
    organization_id INTEGER,
    user_id INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== background_policies.sql ====
CREATE TABLE background_policies (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    panel_backup_cron TEXT NOT NULL,
    log_cleanup_cron TEXT NOT NULL,
    log_retention_days INTEGER NOT NULL,
    panel_backup_enabled INTEGER NOT NULL,
    log_cleanup_enabled INTEGER NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== backup_executions.sql ====
CREATE TABLE backup_executions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    backup_kind TEXT NOT NULL,
    operation TEXT NOT NULL,
    backup_id INTEGER,
    status TEXT NOT NULL,
    object_key TEXT,
    sha256_checksum TEXT,
    size_bytes INTEGER,
    attempt INTEGER NOT NULL,
    error TEXT,
    started_at INTEGER NOT NULL,
    finished_at INTEGER
) STRICT;

-- ==== certificate_renewals.sql ====
CREATE TABLE certificate_renewals (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    certificate_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    previous_expires_at INTEGER,
    new_expires_at INTEGER,
    error TEXT,
    started_at INTEGER NOT NULL,
    finished_at INTEGER
) STRICT;

-- ==== certificates.sql ====
CREATE TABLE certificates (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    certificate_data TEXT NOT NULL,
    private_key TEXT NOT NULL,
    certificate_path TEXT NOT NULL,
    auto_renew INTEGER NOT NULL,
    server_id INTEGER,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== database_networks.sql ====
CREATE TABLE database_networks (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    docker_network_name TEXT NOT NULL,
    description TEXT,
    external INTEGER NOT NULL,
    server_id INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== destinations.sql ====
CREATE TABLE destinations (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider TEXT NOT NULL,
    access_key TEXT NOT NULL,
    secret_access_key TEXT NOT NULL,
    bucket TEXT NOT NULL,
    region TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== dns_providers.sql ====
CREATE TABLE dns_providers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    credentials_json TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== email_verification_tokens.sql ====
CREATE TABLE email_verification_tokens (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token_hash TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    used_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== environments.sql ====
CREATE TABLE environments (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    env_var TEXT NOT NULL,
    is_default INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== git_providers.sql ====
-- Unified git provider credentials (vault-style: provider_type + config_json).
-- GITHUB: {"app_name":..., "app_id":..., "client_id":..., "client_secret":..., "installation_id":..., "private_key":..., "webhook_secret":...}
-- GITLAB: {"url":..., "internal_url":..., "application_id":..., "redirect_uri":..., "secret":..., "access_token":..., "refresh_token":..., "group_name":..., "expires_at":...}
-- GITEA:  {"url":..., "internal_url":..., "redirect_uri":..., "client_id":..., "client_secret":..., "access_token":..., "refresh_token":..., "expires_at":..., "scopes":..., "last_authenticated_at":...}
-- BITBUCKET: {"username":..., "email":..., "app_password":..., "api_token":..., "workspace_name":...}
CREATE TABLE git_providers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    config_json TEXT NOT NULL DEFAULT '{}',
    shared INTEGER NOT NULL,
    webhook_secret TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    CONSTRAINT git_provider_type_check CHECK (provider_type IN ('GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET')),
    CONSTRAINT git_provider_config_json_check CHECK (json_valid(config_json))
) STRICT;

-- ==== group_policy.sql ====
CREATE TABLE group_policy (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    group_id INTEGER NOT NULL,
    policy_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== groups.sql ====
CREATE TABLE groups (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    is_system INTEGER NOT NULL,
    organization_id INTEGER
) STRICT;

-- ==== health_reports.sql ====
CREATE TABLE health_reports (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    target_id INTEGER NOT NULL,
    target_type TEXT NOT NULL,
    status TEXT NOT NULL,
    response_time_ms INTEGER NOT NULL,
    error_message TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== jwt_tokens.sql ====
CREATE TABLE jwt_tokens (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    jti TEXT NOT NULL,
    role TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    is_blacklist INTEGER,
    blacklist_at INTEGER,
    expired_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    session_id TEXT,
    token_kind TEXT
) STRICT;

-- ==== monitoring_agents.sql ====
CREATE TABLE monitoring_agents (
    server_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    token_hash TEXT NOT NULL,
    last_seen_at INTEGER,
    agent_version TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    query_token TEXT
) STRICT;

-- ==== monitoring_maintenance_windows.sql ====
CREATE TABLE monitoring_maintenance_windows (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    server_id INTEGER,
    starts_at INTEGER NOT NULL,
    ends_at INTEGER NOT NULL,
    reason TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== monitoring_policy.sql ====
CREATE TABLE monitoring_policy (
    organization_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    desired_agent_version TEXT,
    retention_days INTEGER NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== notification_delivery_attempts.sql ====
CREATE TABLE notification_delivery_attempts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    trigger_name TEXT NOT NULL,
    correlation_id TEXT NOT NULL,
    status TEXT NOT NULL,
    attempt INTEGER NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    error TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    finished_at INTEGER
) STRICT;

-- ==== notification_resource_bindings.sql ====
CREATE TABLE notification_resource_bindings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== notifications.sql ====
CREATE TABLE notifications (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    config_json TEXT NOT NULL DEFAULT '{}',
    on_app_deploy INTEGER NOT NULL,
    on_app_build_error INTEGER NOT NULL,
    on_database_backup INTEGER NOT NULL,
    on_volume_backup INTEGER NOT NULL,
    on_panel_backup INTEGER NOT NULL,
    on_panel_restart INTEGER NOT NULL,
    on_docker_cleanup INTEGER NOT NULL,
    on_server_threshold INTEGER NOT NULL,
    on_server_migration INTEGER NOT NULL,
    on_security_alert INTEGER NOT NULL,
    on_schedule_success INTEGER NOT NULL,
    on_schedule_failure INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    CONSTRAINT notification_config_json_check CHECK (json_valid(config_json))
) STRICT;

CREATE INDEX idx_notifications_organization_id ON notifications(organization_id);

-- ==== organization_invites.sql ====
CREATE TABLE organization_invites (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL,
    role TEXT,
    status TEXT,
    token TEXT NOT NULL,
    group_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    invited_by INTEGER NOT NULL,
    expired_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== organization_members.sql ====
CREATE TABLE organization_members (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    role TEXT,
    user_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    group_id INTEGER
) STRICT;

-- ==== organization.sql ====
CREATE TABLE organization (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    logo TEXT,
    slug TEXT NOT NULL,
    owner_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== password_reset_tokens.sql ====
CREATE TABLE password_reset_tokens (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token_hash TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    used_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== personal_access_tokens.sql ====
CREATE TABLE personal_access_tokens (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    token_prefix TEXT NOT NULL,
    token_hash TEXT NOT NULL,
    expires_at INTEGER,
    last_used_at INTEGER,
    revoked_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== policy.sql ====
CREATE TABLE policy (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    action TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== projects.sql ====
CREATE TABLE projects (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    env_var TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== project_tags.sql ====
CREATE TABLE project_tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL
) STRICT;

-- ==== registries.sql ====
CREATE TABLE registries (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    registry_name TEXT NOT NULL,
    image_prefix TEXT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    registry_url TEXT NOT NULL,
    registry_type TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== resource_access.sql ====
CREATE TABLE resource_access (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    org_id INTEGER,
    resource_type TEXT,
    resource_id INTEGER,
    created_at INTEGER
) STRICT;

-- ==== rollbacks.sql ====
CREATE TABLE rollbacks (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    deployment_id INTEGER NOT NULL,
    version INTEGER NOT NULL,
    image TEXT,
    full_context TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== schedule_execution_locks.sql ====
CREATE TABLE schedule_execution_locks (
    schedule_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    owner_id TEXT NOT NULL,
    acquired_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
) STRICT;

-- ==== schedule_executions.sql ====
CREATE TABLE schedule_executions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    schedule_id INTEGER NOT NULL,
    organization_id INTEGER,
    trigger_kind TEXT NOT NULL,
    status TEXT NOT NULL,
    attempt INTEGER NOT NULL,
    scheduled_at INTEGER NOT NULL,
    started_at INTEGER NOT NULL,
    finished_at INTEGER,
    message TEXT,
    stdout TEXT,
    stderr TEXT
) STRICT;

-- ==== schedule_runtime_policies.sql ====
CREATE TABLE schedule_runtime_policies (
    schedule_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    retry_count INTEGER NOT NULL,
    retry_delay_seconds INTEGER NOT NULL,
    missed_run_policy TEXT NOT NULL,
    concurrency_policy TEXT NOT NULL,
    lease_seconds INTEGER NOT NULL,
    notify_on_success INTEGER NOT NULL,
    notify_on_failure INTEGER NOT NULL,
    last_scheduled_at INTEGER,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== server_cleanup_executions.sql ====
CREATE TABLE server_cleanup_executions (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    server_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    policy TEXT NOT NULL,
    stdout TEXT,
    stderr TEXT,
    started_at INTEGER NOT NULL,
    finished_at INTEGER
) STRICT;

-- ==== server_management.sql ====
CREATE TABLE server_management (
    server_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    maintenance_mode INTEGER NOT NULL,
    maintenance_message TEXT,
    labels TEXT NOT NULL,
    cleanup_policy TEXT NOT NULL,
    gpu_enabled INTEGER NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== server_migrations.sql ====
CREATE TABLE server_migrations (
    id TEXT NOT NULL PRIMARY KEY,
    source_server_id INTEGER NOT NULL,
    target_server_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    service_ids TEXT NOT NULL,
    build_service_ids TEXT NOT NULL,
    certificate_ids TEXT NOT NULL,
    schedule_ids TEXT NOT NULL,
    queued_services INTEGER NOT NULL,
    queued_total INTEGER NOT NULL,
    error TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== server_private_networks.sql ====
CREATE TABLE server_private_networks (
    server_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    connection_mode TEXT NOT NULL,
    provider TEXT,
    private_host TEXT,
    tunnel_address TEXT,
    public_key TEXT,
    endpoint TEXT,
    listen_port INTEGER,
    persistent_keepalive INTEGER,
    status TEXT NOT NULL,
    last_handshake_at INTEGER,
    config_version INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    dns_name TEXT,
    routes TEXT NOT NULL,
    health_status TEXT NOT NULL,
    health_error TEXT,
    last_health_check_at INTEGER,
    consecutive_failures INTEGER NOT NULL,
    operation TEXT,
    operation_lease_until INTEGER,
    config_hash TEXT,
    rotation_state TEXT NOT NULL
) STRICT;

-- ==== servers.sql ====
CREATE TABLE servers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    ip_address TEXT NOT NULL,
    port INTEGER NOT NULL,
    username TEXT NOT NULL,
    app_name TEXT NOT NULL,
    server_status TEXT NOT NULL,
    server_type TEXT NOT NULL,
    enable_docker_cleanup INTEGER NOT NULL,
    log_cleanup_cron TEXT,
    command TEXT NOT NULL,
    metrics_config TEXT NOT NULL,
    ssh_key_id INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    build_memory_limit TEXT,
    build_cpu_limit TEXT
) STRICT;

-- ==== settings.sql ====
CREATE TABLE settings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    server_ip TEXT,
    certificate_type TEXT NOT NULL,
    custom_cert_resolver TEXT,
    https INTEGER NOT NULL,
    host TEXT,
    lets_encrypt_email TEXT,
    enable_docker_cleanup INTEGER NOT NULL,
    log_cleanup_cron TEXT,
    metrics_config TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== ssh_keys.sql ====
CREATE TABLE ssh_keys (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    private_key TEXT NOT NULL,
    public_key TEXT NOT NULL,
    last_used_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== tags.sql ====
CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== two_factor.sql ====
CREATE TABLE two_factor (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    secret TEXT NOT NULL,
    backup_codes TEXT NOT NULL,
    user_id INTEGER NOT NULL
) STRICT;

-- ==== user_policy.sql ====
CREATE TABLE user_policy (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    org_id INTEGER NOT NULL,
    policy_id INTEGER NOT NULL,
    effect TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== users.sql ====
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    email TEXT,
    last_name TEXT,
    first_name TEXT,
    avatar TEXT NOT NULL,
    role TEXT,
    about_me TEXT,
    password TEXT NOT NULL,
    is_email_verify INTEGER,
    email_verify_at INTEGER,
    two_factor_enable INTEGER,
    is_registered INTEGER NOT NULL,
    added_by INTEGER,
    group_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    is_owner INTEGER
) STRICT;

-- ==== vault_providers.sql ====
CREATE TABLE vault_providers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    api_url TEXT NOT NULL,
    auth_token TEXT NOT NULL,
    namespace TEXT,
    config_json TEXT,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- ==== services.sql ====
-- Unified base for every deployable: APPLICATION | COMPOSE | DATABASE
CREATE TABLE services (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    service_type TEXT NOT NULL,
    name TEXT NOT NULL,
    app_name TEXT NOT NULL UNIQUE,
    description TEXT,
    icon TEXT,
    app_status TEXT NOT NULL,
    last_error TEXT,
    env_var TEXT,
    command TEXT,
    args TEXT,
    refresh_token TEXT,
    docker_image TEXT,
    memory_reservation TEXT,
    memory_limit TEXT,
    cpu_reservation TEXT,
    cpu_limit TEXT,
    replicas INTEGER NOT NULL,
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
    network_ids TEXT NOT NULL,
    detach_rustploy_network INTEGER NOT NULL,
    environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
    server_id INTEGER REFERENCES servers(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    CONSTRAINT service_type_check CHECK (service_type IN ('APPLICATION', 'COMPOSE', 'DATABASE'))
) STRICT;

CREATE INDEX idx_services_environment_id ON services(environment_id);
CREATE INDEX idx_services_server_id ON services(server_id);
CREATE INDEX idx_services_service_type ON services(service_type);
CREATE INDEX idx_services_app_status ON services(app_status);

CREATE TRIGGER services_updated_at
AFTER UPDATE ON services
FOR EACH ROW
BEGIN
    UPDATE services SET updated_at = strftime('%s', 'now') WHERE id = OLD.id;
END;

-- ==== service_git.sql ====
-- Git/VCS source config, shared by APPLICATION and COMPOSE services
CREATE TABLE service_git (
    service_id INTEGER NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    source_type TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    auto_deploy INTEGER,
    watch_paths TEXT,
    enable_submodules INTEGER NOT NULL,
    repository TEXT,
    owner TEXT,
    branch TEXT,
    gitlab_project_id INTEGER,
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
    custom_git_ssh_key_id INTEGER REFERENCES ssh_keys(id) ON DELETE SET NULL,
    github_provider_id INTEGER REFERENCES git_providers(id) ON DELETE SET NULL,
    gitlab_provider_id INTEGER REFERENCES git_providers(id) ON DELETE SET NULL,
    gitea_provider_id INTEGER REFERENCES git_providers(id) ON DELETE SET NULL,
    bitbucket_provider_id INTEGER REFERENCES git_providers(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_service_git_ssh_key ON service_git(custom_git_ssh_key_id);

CREATE TRIGGER service_git_updated_at
AFTER UPDATE ON service_git
FOR EACH ROW
BEGIN
    UPDATE service_git SET updated_at = strftime('%s', 'now') WHERE service_id = OLD.service_id;
END;

-- ==== service_application.sql ====
-- Build config and preview deployments for APPLICATION services
CREATE TABLE service_application (
    service_id INTEGER NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    build_type TEXT NOT NULL,
    build_args TEXT,
    build_secrets TEXT,
    dockerfile TEXT,
    docker_context_path TEXT,
    docker_build_stage TEXT,
    publish_directory TEXT,
    is_static_spa INTEGER,
    create_env_file INTEGER NOT NULL,
    railpack_version TEXT,
    heroku_version TEXT,
    build_path TEXT,
    clean_cache INTEGER NOT NULL,
    drop_build_path TEXT,
    docker_username TEXT,
    docker_password TEXT,
    registry_url TEXT,
    build_server_id INTEGER REFERENCES servers(id) ON DELETE SET NULL,
    registry_id INTEGER REFERENCES registries(id) ON DELETE SET NULL,
    rollback_registry_id INTEGER REFERENCES registries(id) ON DELETE SET NULL,
    build_registry_id INTEGER REFERENCES registries(id) ON DELETE SET NULL,
    rollback_active INTEGER NOT NULL,
    preview_env TEXT,
    preview_build_args TEXT,
    preview_build_secrets TEXT,
    preview_labels TEXT,
    preview_wildcard TEXT,
    preview_port INTEGER,
    preview_https INTEGER NOT NULL,
    preview_path TEXT,
    preview_certificate_type TEXT NOT NULL,
    preview_custom_cert_resolver TEXT,
    preview_limit INTEGER,
    is_preview_deployments_active INTEGER NOT NULL,
    preview_require_collaborator_permissions INTEGER NOT NULL,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_service_application_build_server ON service_application(build_server_id);

CREATE TRIGGER service_application_updated_at
AFTER UPDATE ON service_application
FOR EACH ROW
BEGIN
    UPDATE service_application SET updated_at = strftime('%s', 'now') WHERE service_id = OLD.service_id;
END;

-- ==== service_compose.sql ====
-- Compose/stack config; the rest lives in services + service_git
CREATE TABLE service_compose (
    service_id INTEGER NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    compose_type TEXT NOT NULL,
    compose_file TEXT NOT NULL,
    compose_path TEXT NOT NULL,
    suffix TEXT NOT NULL,
    randomize INTEGER NOT NULL,
    isolated_deployment INTEGER NOT NULL,
    isolated_deployments_volume INTEGER NOT NULL,
    service_networks TEXT NOT NULL
) STRICT;

-- ==== service_database.sql ====
-- Managed databases: POSTGRES | MYSQL | MARIADB | MONGO | REDIS | LIBSQL
-- Engine-specific credentials/config live in engine_config (JSON).
CREATE TABLE service_database (
    service_id INTEGER NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    db_type TEXT NOT NULL,
    external_port INTEGER,
    -- common credentials
    db_name TEXT,
    db_user TEXT,
    db_password TEXT,
    -- engine config JSON, validated:
    --   POSTGRES: {"postgres_config":..., "replication_role":..., "primary_host":..., "primary_port":..., "replication_user":..., "replication_password":...}
    --   MYSQL/MARIADB: {"database_root_password":...}
    --   MONGO: {"database_root_password":..., "replica_sets":...}
    --   LIBSQL: {"sqld_node":..., "sqld_primary_url":..., "enable_namespaces":..., "external_grpc_port":..., "external_admin_port":...}
    engine_config TEXT NOT NULL DEFAULT '{}',
    CONSTRAINT db_type_check CHECK (db_type IN ('POSTGRES', 'MYSQL', 'MARIADB', 'MONGO', 'REDIS', 'LIBSQL')),
    CONSTRAINT engine_config_json_check CHECK (json_valid(engine_config))
) STRICT;

CREATE INDEX idx_service_database_db_type ON service_database(db_type);

-- ==== deployments.sql ====
CREATE TABLE deployments (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    state TEXT NOT NULL,
    log_path TEXT NOT NULL,
    pid TEXT,
    error_message TEXT,
    operation TEXT,
    is_preview_deployment INTEGER NOT NULL,
    started_at INTEGER,
    last_state_at INTEGER,
    finished_at INTEGER,
    service_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    server_id INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    database_kind TEXT
) STRICT;

-- ==== preview_deployments.sql ====
CREATE TABLE preview_deployments (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    base_service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    preview_service_id INTEGER REFERENCES services(id) ON DELETE SET NULL,
    provider_type TEXT NOT NULL,
    provider_id INTEGER NOT NULL,
    owner TEXT NOT NULL,
    repository TEXT NOT NULL,
    pull_request_number TEXT NOT NULL,
    source_branch TEXT NOT NULL,
    target_branch TEXT NOT NULL,
    commit_sha TEXT,
    author TEXT,
    status TEXT NOT NULL,
    domain TEXT NOT NULL,
    last_deployment_id INTEGER REFERENCES deployments(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    source_owner TEXT,
    source_repository TEXT
) STRICT;

CREATE INDEX idx_preview_deployments_base_service_id ON preview_deployments(base_service_id);
CREATE INDEX idx_preview_deployments_preview_service_id ON preview_deployments(preview_service_id);

-- ==== ai_generations.sql ====
CREATE TABLE ai_generations (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    ai_setting_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_by INTEGER NOT NULL,
    prompt TEXT NOT NULL,
    output_json TEXT NOT NULL,
    status TEXT NOT NULL,
    service_id INTEGER REFERENCES services(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_ai_generations_service_id ON ai_generations(service_id);

-- ==== application_middlewares.sql ====
CREATE TABLE application_middlewares (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    middleware_type TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    config TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_application_middlewares_service_id ON application_middlewares(service_id);

-- ==== backups.sql ====
CREATE TABLE backups (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    app_name TEXT NOT NULL,
    schedule TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    database_name TEXT NOT NULL,
    prefix TEXT NOT NULL,
    service_name TEXT,
    keep_latest_count INTEGER,
    backup_type TEXT NOT NULL,
    database_type TEXT,
    metadata TEXT,
    service_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    destination_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_backups_service_id ON backups(service_id);

-- ==== domains.sql ====
CREATE TABLE domains (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    host TEXT NOT NULL,
    https INTEGER NOT NULL,
    port INTEGER,
    path TEXT,
    internal_path TEXT,
    custom_entrypoint TEXT,
    service_name TEXT,
    custom_cert_resolver TEXT,
    strip_path INTEGER NOT NULL,
    middlewares TEXT NOT NULL,
    domain_type TEXT NOT NULL,
    certificate_type TEXT NOT NULL,
    service_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_domains_service_id ON domains(service_id);

-- ==== mounts.sql ====
CREATE TABLE mounts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    mount_type TEXT NOT NULL,
    service_type TEXT NOT NULL,
    host_path TEXT,
    volume_name TEXT,
    file_path TEXT,
    content TEXT,
    mount_path TEXT NOT NULL,
    service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_mounts_service_id ON mounts(service_id);

-- ==== patches.sql ====
CREATE TABLE patches (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    patch_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    content TEXT NOT NULL,
    service_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_patches_service_id ON patches(service_id);

-- ==== ports.sql ====
CREATE TABLE ports (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    published_port INTEGER NOT NULL,
    target_port INTEGER NOT NULL,
    protocol TEXT NOT NULL,
    publish_mode TEXT NOT NULL,
    service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_ports_service_id ON ports(service_id);

-- ==== redirects.sql ====
CREATE TABLE redirects (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    regex TEXT NOT NULL,
    replacement TEXT NOT NULL,
    permanent INTEGER NOT NULL,
    unique_config_key INTEGER,
    service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_redirects_service_id ON redirects(service_id);

-- ==== schedules.sql ====
CREATE TABLE schedules (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    cron_expression TEXT NOT NULL,
    app_name TEXT NOT NULL,
    service_name TEXT,
    shell_type TEXT NOT NULL,
    schedule_type TEXT NOT NULL,
    command TEXT NOT NULL,
    script TEXT,
    timezone TEXT,
    enabled INTEGER NOT NULL,
    service_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    server_id INTEGER,
    organization_id INTEGER,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    schedule_action TEXT NOT NULL
) STRICT;

CREATE INDEX idx_schedules_service_id ON schedules(service_id);

-- ==== security.sql ====
CREATE TABLE security (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_security_service_id ON security(service_id);

-- ==== volume_backups.sql ====
CREATE TABLE volume_backups (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    volume_name TEXT NOT NULL,
    prefix TEXT NOT NULL,
    service_type TEXT NOT NULL,
    app_name TEXT NOT NULL,
    service_name TEXT,
    turn_off INTEGER NOT NULL,
    cron_expression TEXT NOT NULL,
    keep_latest_count INTEGER,
    enabled INTEGER NOT NULL,
    service_id INTEGER REFERENCES services(id) ON DELETE CASCADE,
    destination_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_volume_backups_service_id ON volume_backups(service_id);

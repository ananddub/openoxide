-- Build config and preview deployments for APPLICATION services
CREATE TABLE service_application (
    service_id BIGINT NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    build_type TEXT NOT NULL,
    build_args TEXT,
    build_secrets TEXT,
    dockerfile TEXT,
    docker_context_path TEXT,
    docker_build_stage TEXT,
    publish_directory TEXT,
    is_static_spa BIGINT,
    create_env_file BIGINT NOT NULL,
    railpack_version TEXT,
    heroku_version TEXT,
    build_path TEXT,
    clean_cache BIGINT NOT NULL,
    drop_build_path TEXT,
    docker_username TEXT,
    docker_password TEXT,
    registry_url TEXT,
    build_server_id BIGINT REFERENCES servers(id) ON DELETE SET NULL,
    registry_id BIGINT REFERENCES registries(id) ON DELETE SET NULL,
    rollback_registry_id BIGINT REFERENCES registries(id) ON DELETE SET NULL,
    build_registry_id BIGINT REFERENCES registries(id) ON DELETE SET NULL,
    rollback_active BIGINT NOT NULL,
    preview_env TEXT,
    preview_build_args TEXT,
    preview_build_secrets TEXT,
    preview_labels TEXT,
    preview_wildcard TEXT,
    preview_port BIGINT,
    preview_https BIGINT NOT NULL,
    preview_path TEXT,
    preview_certificate_type TEXT NOT NULL,
    preview_custom_cert_resolver TEXT,
    preview_limit BIGINT,
    is_preview_deployments_active BIGINT NOT NULL,
    preview_require_collaborator_permissions BIGINT NOT NULL,
    updated_at BIGINT NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX idx_service_application_build_server ON service_application(build_server_id);

CREATE TRIGGER service_application_updated_at
AFTER UPDATE ON service_application
FOR EACH ROW
BEGIN
    UPDATE service_application SET updated_at = strftime('%s', 'now') WHERE service_id = OLD.service_id;
END;

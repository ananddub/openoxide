-- Unified base for every deployable: APPLICATION | COMPOSE | DATABASE
CREATE TABLE services (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
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
    replicas BIGINT NOT NULL,
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
    stop_grace_period_swarm BIGINT,
    network_ids TEXT NOT NULL,
    detach_rustploy_network BIGINT NOT NULL,
    environment_id BIGINT NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
    server_id BIGINT REFERENCES servers(id) ON DELETE SET NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    CONSTRAINT service_type_check CHECK (service_type IN ('APPLICATION', 'COMPOSE', 'DATABASE'))
);

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

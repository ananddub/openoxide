-- Compose/stack config; the rest lives in services + service_git
CREATE TABLE service_compose (
    service_id BIGINT NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    compose_type TEXT NOT NULL,
    compose_file TEXT NOT NULL,
    compose_path TEXT NOT NULL,
    suffix TEXT NOT NULL,
    randomize BIGINT NOT NULL,
    isolated_deployment BIGINT NOT NULL,
    isolated_deployments_volume BIGINT NOT NULL,
    service_networks TEXT NOT NULL
);

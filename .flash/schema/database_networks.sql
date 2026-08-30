CREATE TABLE database_networks (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    docker_network_name TEXT NOT NULL,
    description TEXT,
    external BIGINT NOT NULL,
    server_id BIGINT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

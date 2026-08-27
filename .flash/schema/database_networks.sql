CREATE TABLE database_networks (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    docker_network_name TEXT NOT NULL,
    description TEXT,
    external INTEGER NOT NULL,
    server_id INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

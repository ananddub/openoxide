CREATE TABLE rollbacks (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    deployment_id BIGINT NOT NULL,
    version BIGINT NOT NULL,
    image TEXT,
    full_context TEXT,
    created_at BIGINT NOT NULL
);

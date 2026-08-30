CREATE TABLE environments (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    env_var TEXT NOT NULL,
    is_default BIGINT NOT NULL,
    project_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

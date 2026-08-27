CREATE TABLE environments (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    env_var TEXT NOT NULL,
    is_default INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

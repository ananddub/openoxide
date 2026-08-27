CREATE TABLE ai_generations (
    id INTEGER NOT NULL PRIMARY KEY,
    ai_setting_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_by INTEGER NOT NULL,
    prompt TEXT NOT NULL,
    output_json TEXT NOT NULL,
    status TEXT NOT NULL,
    compose_id INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

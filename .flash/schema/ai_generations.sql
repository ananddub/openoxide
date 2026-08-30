CREATE TABLE ai_generations (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    ai_setting_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_by BIGINT NOT NULL,
    prompt TEXT NOT NULL,
    output_json TEXT NOT NULL,
    status TEXT NOT NULL,
    service_id BIGINT REFERENCES services(id) ON DELETE SET NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_ai_generations_service_id ON ai_generations(service_id);

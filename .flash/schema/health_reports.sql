CREATE TABLE health_reports (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    target_id BIGINT NOT NULL,
    target_type TEXT NOT NULL,
    status TEXT NOT NULL,
    response_time_ms BIGINT NOT NULL,
    error_message TEXT,
    created_at BIGINT NOT NULL
);

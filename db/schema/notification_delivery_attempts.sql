CREATE TABLE notification_delivery_attempts (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    notification_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    trigger_name TEXT NOT NULL,
    correlation_id TEXT NOT NULL,
    status TEXT NOT NULL,
    attempt BIGINT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    error TEXT,
    created_at BIGINT NOT NULL,
    finished_at BIGINT
);

CREATE TABLE notification_delivery_attempts (
    id INTEGER NOT NULL PRIMARY KEY,
    notification_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    trigger_name TEXT NOT NULL,
    correlation_id TEXT NOT NULL,
    status TEXT NOT NULL,
    attempt INTEGER NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    error TEXT,
    created_at INTEGER NOT NULL,
    finished_at INTEGER
);

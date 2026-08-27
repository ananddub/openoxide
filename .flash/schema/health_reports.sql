CREATE TABLE health_reports (
    id INTEGER NOT NULL PRIMARY KEY,
    target_id INTEGER NOT NULL,
    target_type TEXT NOT NULL,
    status TEXT NOT NULL,
    response_time_ms INTEGER NOT NULL,
    error_message TEXT,
    created_at INTEGER NOT NULL
);

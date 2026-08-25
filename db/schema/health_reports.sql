CREATE TABLE health_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    target_id INTEGER NOT NULL,
    target_type TEXT NOT NULL, -- 'APPLICATION', 'DATABASE', 'SERVER'
    status TEXT NOT NULL,      -- 'HEALTHY', 'UNHEALTHY', 'DEGRADED'
    response_time_ms INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);
CREATE INDEX idx_health_reports_target ON health_reports(target_type, target_id);
CREATE INDEX idx_health_reports_created ON health_reports(created_at);

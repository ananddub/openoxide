CREATE TABLE schedule_executions (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    schedule_id BIGINT NOT NULL,
    organization_id BIGINT,
    trigger_kind TEXT NOT NULL,
    status TEXT NOT NULL,
    attempt BIGINT NOT NULL,
    scheduled_at BIGINT NOT NULL,
    started_at BIGINT NOT NULL,
    finished_at BIGINT,
    message TEXT,
    stdout TEXT,
    stderr TEXT
);

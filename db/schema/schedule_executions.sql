CREATE TABLE schedule_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    schedule_id INTEGER NOT NULL REFERENCES schedules(id) ON DELETE CASCADE,
    organization_id INTEGER,
    trigger_kind TEXT NOT NULL CHECK (trigger_kind IN ('CRON', 'MANUAL', 'MISSED', 'RETRY')),
    status TEXT NOT NULL CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED', 'SKIPPED')),
    attempt INTEGER NOT NULL DEFAULT 1,
    scheduled_at INTEGER NOT NULL,
    started_at INTEGER NOT NULL DEFAULT (unixepoch()),
    finished_at INTEGER,
    message TEXT,
    stdout TEXT,
    stderr TEXT
) STRICT;
CREATE INDEX idx_schedule_executions_schedule_started ON schedule_executions(schedule_id, started_at DESC);
CREATE INDEX idx_schedule_executions_org_started ON schedule_executions(organization_id, started_at DESC);

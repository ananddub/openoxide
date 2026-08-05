CREATE TABLE schedule_runtime_policies (
    schedule_id INTEGER PRIMARY KEY REFERENCES schedules(id) ON DELETE CASCADE,
    retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count BETWEEN 0 AND 10),
    retry_delay_seconds INTEGER NOT NULL DEFAULT 30 CHECK (retry_delay_seconds BETWEEN 1 AND 86400),
    missed_run_policy TEXT NOT NULL DEFAULT 'SKIP' CHECK (missed_run_policy IN ('SKIP', 'RUN_ONCE')),
    concurrency_policy TEXT NOT NULL DEFAULT 'SKIP' CHECK (concurrency_policy IN ('SKIP', 'QUEUE', 'ALLOW')),
    lease_seconds INTEGER NOT NULL DEFAULT 3600 CHECK (lease_seconds BETWEEN 30 AND 86400),
    notify_on_success INTEGER NOT NULL DEFAULT 0 CHECK (notify_on_success IN (0, 1)),
    notify_on_failure INTEGER NOT NULL DEFAULT 1 CHECK (notify_on_failure IN (0, 1)),
    last_scheduled_at INTEGER,
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

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

CREATE TABLE schedule_execution_locks (
    schedule_id INTEGER PRIMARY KEY REFERENCES schedules(id) ON DELETE CASCADE,
    owner_id TEXT NOT NULL,
    acquired_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
) STRICT;

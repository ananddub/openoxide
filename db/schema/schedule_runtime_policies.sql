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

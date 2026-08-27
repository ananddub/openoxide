CREATE TABLE schedule_runtime_policies (
    schedule_id INTEGER NOT NULL PRIMARY KEY,
    retry_count INTEGER NOT NULL,
    retry_delay_seconds INTEGER NOT NULL,
    missed_run_policy TEXT NOT NULL,
    concurrency_policy TEXT NOT NULL,
    lease_seconds INTEGER NOT NULL,
    notify_on_success INTEGER NOT NULL,
    notify_on_failure INTEGER NOT NULL,
    last_scheduled_at INTEGER,
    updated_at INTEGER NOT NULL
);

CREATE TABLE schedule_runtime_policies (
    schedule_id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    retry_count BIGINT NOT NULL,
    retry_delay_seconds BIGINT NOT NULL,
    missed_run_policy TEXT NOT NULL,
    concurrency_policy TEXT NOT NULL,
    lease_seconds BIGINT NOT NULL,
    notify_on_success BIGINT NOT NULL,
    notify_on_failure BIGINT NOT NULL,
    last_scheduled_at BIGINT,
    updated_at BIGINT NOT NULL
);

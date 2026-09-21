CREATE TABLE traffic_stats (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    organization_id BIGINT NOT NULL,
    server_id BIGINT NOT NULL,
    service TEXT NOT NULL,
    bucket_at BIGINT NOT NULL,
    requests BIGINT NOT NULL DEFAULT 0,
    errors BIGINT NOT NULL DEFAULT 0,
    duration_ms REAL NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL
);

CREATE UNIQUE INDEX idx_traffic_stats_point
    ON traffic_stats(server_id, service, bucket_at);
CREATE INDEX idx_traffic_stats_bucket ON traffic_stats(bucket_at);
-- Per-service edge traffic, sampled from Traefik's Prometheus counters.
--
-- One row per (server, traefik service, minute). `requests` and `errors` are
-- deltas accumulated inside the minute, so the rows are the time series behind
-- the Traffic view — no per-request log is stored anywhere.
CREATE TABLE traffic_stats (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
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
-- ============================================================
-- TRAFFIC STATS
-- ============================================================

-- name: TrafficAccumulate :exec
INSERT INTO traffic_stats (
    organization_id, server_id, service, bucket_at, requests, errors, duration_ms, created_at
) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(server_id, service, bucket_at) DO UPDATE SET
    requests = requests + excluded.requests,
    errors = errors + excluded.errors,
    duration_ms = duration_ms + excluded.duration_ms;

-- name: TrafficSeries :many
SELECT service, bucket_at, requests, errors, duration_ms
FROM traffic_stats
WHERE server_id = ? AND bucket_at >= ?
ORDER BY bucket_at ASC;

-- name: TrafficSeriesForService :many
SELECT service, bucket_at, requests, errors, duration_ms
FROM traffic_stats
WHERE server_id = ? AND service = ? AND bucket_at >= ?
ORDER BY bucket_at ASC;

-- name: TrafficTotals :many
SELECT service,
       SUM(requests) AS requests,
       SUM(errors) AS errors,
       SUM(duration_ms) AS duration_ms
FROM traffic_stats
WHERE server_id = ? AND bucket_at >= ?
GROUP BY service
ORDER BY requests DESC;

-- name: TrafficTotalsForPrefix :many
SELECT service,
       SUM(requests) AS requests,
       SUM(errors) AS errors,
       SUM(duration_ms) AS duration_ms
FROM traffic_stats
WHERE server_id = ? AND (service = ? OR service LIKE ?) AND bucket_at >= ?
GROUP BY service
ORDER BY requests DESC;

-- name: TrafficSeriesForPrefix :many
SELECT service, bucket_at, requests, errors, duration_ms
FROM traffic_stats
WHERE server_id = ? AND (service = ? OR service LIKE ?) AND bucket_at >= ?
ORDER BY bucket_at ASC;

-- name: TrafficPurge :exec
DELETE FROM traffic_stats WHERE bucket_at < ?;
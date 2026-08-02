# rustploy_monitor

Standalone monitoring agent. Runs one instance per host, independently of the
panel, and does three things:

- samples host CPU / memory / disk / network and stores them locally
- samples per-container docker stats, stores them, and pushes them to the panel
- serves the panel's metric queries over gRPC

It keeps its own SQLite database, so metric history survives a panel restart and
the panel can be down without losing readings.

## Layout

```
src/
├── main.rs          task wiring and graceful shutdown
├── config.rs        every env var, validated at startup
├── collector/
│   ├── system.rs    host metrics via sysinfo
│   └── docker.rs    container metrics via `docker stats`
├── logs.rs          `docker logs` tail
├── store.rs         SQLite persistence
└── grpc.rs          query API served to the panel
```

## Configuration

All configuration is environment-based and validated on boot — an invalid value
exits with a message rather than starting half-configured.

| Variable | Default | Purpose |
|---|---|---|
| `SERVER_ID` | `1` | Panel's id for this host. Set this when running more than one agent. |
| `MONITOR_DATABASE_URL` | `sqlite://monitor.db` | Local metric store. |
| `GRPC_PORT` | `50051` | Port the query API listens on. |
| `REFRESH_RATE` | `60` | Seconds between collection cycles. |
| `RETENTION_DAYS` | `7` | Days of metrics kept before pruning. |
| `RUSTPLOY_SERVER_URL` | `http://127.0.0.1:4000` | Panel base URL for container metric pushes. |
| `METRICS_TOKEN` | *(empty)* | Shared secret sent as `X-Metrics-Token`. Warns if unset. |
| `RUST_LOG` | `rustploy_monitor=info` | Log filter. |

## Running

```bash
just run-monitor          # locally
just test-monitor         # test suite
just up-monitor           # as a container
just logs-monitor
```

The container needs the docker socket mounted read-only. It runs as a non-root
user, so it also needs the host's `docker` group id — check yours with
`getent group docker` and adjust `group_add` in `docker-compose.monitor.yml` if
it isn't `999`.

## gRPC API

The agent serves three RPCs (`proto/monitoring.proto`):

- `GetServerMetrics` — recent host metrics
- `GetContainerMetrics` — recent container metrics, filtered by app name
- `StreamLogs` — tail of a container's recent logs

Metric *ingestion* is not part of this API. Container metrics are pushed to the
panel over HTTP (`POST /api/monitoring/containers`) and host metrics stay in the
local store until queried.

## Notes

Container metrics are collected by shelling out to `docker stats --no-stream`
rather than reading the API socket. This avoids reimplementing docker's
two-sample CPU delta calculation, at the cost of one process spawn per cycle —
acceptable at a 60s cadence.

`net_rx_kbps` / `net_tx_kbps` in the panel push are cumulative totals since
container start, converted to KB, not per-second rates. The field names are
inherited from the panel's existing ingest schema.

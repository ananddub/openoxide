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
├── filter.rs        container include/exclude matching
├── rollup.rs        sample aggregation for high-density hosts
├── collector/
│   ├── system.rs    host metrics via sysinfo
│   └── cgroup.rs    container metrics via cgroup v2 (the dense path)
├── docker/
│   ├── api.rs       unix-socket client for the Docker Engine API
│   ├── cgroup.rs    cgroup v2 file reader
│   ├── json_lines.rs  framing for the daemon's streaming responses
│   ├── stats.rs     stats payloads and CPU/memory derivation
│   └── stream.rs    per-container stats streams (the live path)
├── logs.rs          container log tail (demuxes docker's stream framing)
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
| `DOCKER_SOCKET` | `/var/run/docker.sock` | Docker daemon socket to read stats and logs from. |
| `COLLECTION_MODE` | `auto` | `cgroup`, `stream`, or `auto` (cgroup when available). See below. |
| `INCLUDE_CONTAINERS` | *(empty)* | Comma-separated name patterns to monitor. Empty means all. `*` is the wildcard. |
| `EXCLUDE_CONTAINERS` | *(empty)* | Comma-separated name patterns to skip. Wins over include. |
| `ROLLUP_SAMPLES` | `1` | Samples per rollup window. `1` stores every sample; higher values store only an average + peak per window. |
| `RUST_LOG` | `rustploy_monitor=info` | Log filter. |

## Collection modes

**cgroup** (default) reads each container's metrics straight from
`/sys/fs/cgroup`. Measured at ~35 µs per container and scales linearly, so a
host with thousands of containers costs milliseconds per cycle and puts no load
on dockerd. It has no network counters — those live in the container's network
namespace, not its cgroup — so `net_in_mb` / `net_out_mb` are zero.

**stream** opens a persistent `GET /containers/{id}/stats?stream=true` per
container. The daemon pushes a frame per second, giving sub-second freshness and
network I/O, but it needs a connection and daemon work per container. Measured
at 14 containers this is free (0% CPU); it is the wrong choice in the thousands.

Measured on this machine, for reference:

| containers | cgroup read |
|---|---|
| 13 | 0.6 ms |
| 500 | 18 ms |
| 2,000 | 69 ms |
| 20,000 | 704 ms |

## Filtering

Patterns are matched with `*` as the wildcard and are **exact otherwise** — a
bare `api` matches a container named `api`, not `my-api-gateway`. To match a
substring, say `*api*`. This is deliberate: substring-by-default makes filters
silently over-match, which is very hard to notice.

```bash
EXCLUDE_CONTAINERS="*buildkit*,*_test"
INCLUDE_CONTAINERS="prod-*"
```

## Rollup

At high density, storing every sample is the constraint rather than collecting
it. Measured at 284 bytes per row, 20,000 containers on a 60 s cadence produce
~200M rows and ~57 GB a week — past what SQLite handles well.

`ROLLUP_SAMPLES=300` keeps sampling at the same rate but writes only two rows
per container per window: the average and the peak. The peak matters — a
container that pegs a core for ten seconds inside a five-minute window averages
away to almost nothing. Peak rows are suffixed ` (peak)` in the `name` column.

## Running

```bash
just run-monitor          # locally
just test-monitor         # test suite
just up-monitor           # as a container
just logs-monitor
```

The container needs the docker socket mounted read-only — that is its only host
dependency. The image is built `FROM scratch` with a static binary, so it has no
shell and runs as root (uid 0); there is no user database to map a non-root user
against.

## gRPC API

The agent serves three RPCs (`proto/monitoring.proto`):

- `GetServerMetrics` — recent host metrics
- `GetContainerMetrics` — recent container metrics, filtered by app name
- `StreamLogs` — tail of a container's recent logs

Metric *ingestion* is not part of this API. Container metrics are pushed to the
panel over HTTP (`POST /api/monitoring/containers`) and host metrics stay in the
local store until queried.

## Notes

Container metrics come from the Docker Engine API over the unix socket — the
agent never shells out to the docker CLI. Each non-streaming stats response
carries the previous CPU sample (`precpu_stats`), so a real usage percentage is
computed from the delta in one request; memory excludes page cache, matching
what `docker stats` displays. The tradeoff of this approach is the image stays
tiny (~7 MB binary, ~3 MB compressed) because no CLI or OpenSSL is bundled.

`net_rx_kbps` / `net_tx_kbps` in the panel push are cumulative totals since
container start, converted to KB, not per-second rates. The field names are
inherited from the panel's existing ingest schema.

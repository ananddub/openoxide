# OpenOxide Monitor

`openoxide_monitor` is the host-side monitoring agent for OpenOxide. One agent
runs on every Docker server managed by the OpenOxide panel. It collects host and
container telemetry, keeps the only durable metric history on that node, and
exposes an authenticated gRPC API for panel metric and log queries.

This document explains the complete data flow, configuration, server identity,
authentication, storage, deployment, and troubleshooting model.

## System overview

```text
Docker host
┌──────────────────────────────────────────────────────────────┐
│ openoxide_monitor                                             │
│                                                              │
│  Host collector ───────────────┐                             │
│  Container collector ──────────┼── Local SQLite history      │
│                                │                             │
│  Authenticated gRPC API ◄──────┘                        │    │
│       metrics + container logs                          │    │
└─────────────────────────────────────────────────────────┼────┘
                                                          │
                                                          ▼
                                            OpenOxide panel
                                            ┌───────────────┐
                                            │ authentication│
                                            │ query proxy   │
                                            │ alert engine  │
                                            │ notifications │
                                            └───────────────┘
```

The agent performs four main jobs:

1. Collect host CPU, memory, disk, uptime, OS, and network metrics.
2. Collect CPU, memory, network, and block-I/O metrics for Docker containers.
3. Persist readings in its own SQLite database and periodically remove expired
   history.
4. Serve authenticated gRPC metric/log requests from the OpenOxide panel.

The agent is the single metric store. The panel keeps configuration, agent
identity and alert history, and pulls raw metrics only when needed.

## Server identity: `SERVER_ID`

`SERVER_ID` is the most important setting. It is not an arbitrary machine
number, IP address, Docker ID, hostname, or Swarm node ID. It must equal the
primary key of this machine in the panel's `servers` table.

For example, if the panel database contains:

```text
id  name       ip_address
5   local-lima lima
10  raspberry  pi
12  production 203.0.113.10
```

the agent on `local-lima` must use:

```env
SERVER_ID=5
```

Every gRPC query contains the expected `server_id`. An agent rejects requests
for a different server, so one node cannot accidentally answer for another.

Common symptoms of an incorrect ID:

- Panel returns a gRPC `NOT_FOUND` response.
- Monitoring history appears unavailable for that server.

Never copy one compose file to several servers without changing `SERVER_ID`.
Each host must have its own panel server row and matching agent identity.

To inspect IDs in a local development database:

```bash
sqlite3 data/db.sqlite3 \
  'SELECT id, name, ip_address, server_status FROM servers ORDER BY id;'
```

In normal product usage, use the ID shown or returned by the OpenOxide server
API/UI rather than accessing SQLite directly.

## Authentication

All agent-to-panel HTTP pushes include:

```text
X-Metrics-Token: <secret>
X-Server-Id: <SERVER_ID>
```

The same token is also required as gRPC metadata under
`x-metrics-token`. gRPC does not accept `server_id=0` as a wildcard; requests
must identify the exact agent server.

The panel accepts either of these token models:

### Per-server token (recommended)

The panel generates a unique token for one server and stores only its SHA-256
hash in `monitoring_agents`. The plaintext token is returned once and should be
installed as the agent's `METRICS_TOKEN`.

```text
POST /api/monitoring/agents/{server_id}/token
```

The operation is permission- and organization-scoped. The panel verifies that
the server belongs to the authenticated organization through its deployed
applications or compose projects, or through an existing monitoring binding.

Advantages:

- Compromise of one server does not expose every monitoring agent.
- Tokens can be rotated independently.
- Successful authentication updates the agent's `last_seen_at` heartbeat.

### Global migration token

For local development or migration, the panel and agent may use the same global
`METRICS_TOKEN` environment value:

```env
# Panel
METRICS_TOKEN=a-long-random-secret

# Agent on the Docker host
METRICS_TOKEN=a-long-random-secret
```

An empty panel token does not authorize anonymous ingestion. `change_me` is
only a local example and must not be used in production.

Generate a suitable value with:

```bash
openssl rand -hex 32
```

## Complete metric flow

### Host metrics

The system collector samples:

- CPU percentage, model, speed, logical and physical core counts
- memory usage and total memory
- disk usage and total disk capacity
- network input/output counters
- uptime
- OS, distribution, kernel, and architecture

Each sample is saved only to the agent SQLite database. The panel reads it on
demand through the authenticated `GetServerMetrics` gRPC method.

### Container metrics

The agent discovers running containers through the Docker Engine API and
collects:

- CPU percentage
- memory used, limit, and percentage
- network received/transmitted totals when stream mode is used
- block read/write totals
- container ID and display name
- optional OpenOxide application or compose resource identity

Samples are stored only on the remote agent. The panel reads them on demand
through the authenticated `GetContainerMetrics` gRPC method.

### Live UI and history

The panel provides authorized pull-through history:

```text
GET /api/monitoring/containers/{server_id}
    ?organization_id=<organization_id>
    &appName=<optional-container-name>
    &limit=<1..1000>
```

These routes require monitoring permission and validate the server's
organization binding before returning data.

### Heartbeat and status

Successful per-server authentication updates `last_seen_at`. The panel status
endpoint reports:

```text
GET /api/monitoring/agents/{server_id}/status
```

States are:

- `ONLINE`: heartbeat received within 180 seconds
- `STALE`: an older heartbeat exists
- `NEVER_SEEN`: registered but no successful heartbeat has arrived

## Application and compose attribution

Container metrics can include `application_id` or `compose_id`. In stream mode,
the agent reads these Docker labels when present:

```text
com.openoxide.application-id=<application database id>
com.openoxide.compose-id=<compose project database id>
```

An unattributed container sends zero for both IDs. It is still stored and shown
as a server/container metric, but resource-specific alert rules ignore it.

Important current behavior: cgroup collection identifies containers through
cgroup data and does not currently enrich every row with Docker labels. Use
stream mode when label-based resource attribution and network metrics are
required. Deployment definitions must also apply the corresponding OpenOxide
label for attribution to work.

## Collection modes

### `auto`

Recommended default. Uses cgroup v2 when available and falls back to Docker
stats streaming otherwise.

### `cgroup`

Reads container CPU, memory, and block I/O directly from `/sys/fs/cgroup`.

Pros:

- Very low overhead.
- Scales to thousands of containers.
- Avoids one long-lived Docker stats connection per container.

Limitations:

- No container network counters.
- Resource-label attribution is limited because collection is cgroup-driven.

Measured reference performance:

| Containers | Collection time |
|---:|---:|
| 13 | 0.6 ms |
| 500 | 18 ms |
| 2,000 | 69 ms |
| 20,000 | 704 ms |

### `stream`

Opens Docker Engine `GET /containers/{id}/stats?stream=true` streams.

Pros:

- Faster, near-live updates.
- Network counters are available.
- Docker labels can be attached to samples for application/compose attribution.

Limitations:

- One stream and additional Docker daemon work per monitored container.
- Not recommended for hosts with thousands of containers.

## Local and panel storage

### Agent SQLite

`MONITOR_DATABASE_URL` controls the agent store. In Docker it should point to a
persistent volume:

```env
MONITOR_DATABASE_URL=sqlite:///app/data/monitor.db
```

The retention task periodically removes samples older than `RETENTION_DAYS`.
The default is seven days.

### Panel SQLite

The panel stores no raw host or container metrics. It stores monitoring agent
identity/token metadata and alert configuration/history only.

## Rollups for dense hosts

`ROLLUP_SAMPLES=1` stores every sample. A higher value buffers samples and emits
an average plus a peak row for each window.

For example, with a 60-second refresh interval:

```env
REFRESH_RATE=60
ROLLUP_SAMPLES=5
```

produces a five-minute window. The average represents sustained load and the
peak preserves short spikes that an average would hide.

At very high container counts this prevents SQLite growth from becoming the
system bottleneck.

## Filtering containers

An empty include list monitors all containers. Exclusions always win.

```env
INCLUDE_CONTAINERS=prod-*,database
EXCLUDE_CONTAINERS=*buildkit*,*_test
```

Patterns are exact unless they contain `*`:

- `api` matches only `api`
- `api-*` matches names beginning with `api-`
- `*api*` matches names containing `api`

## Alert processing

The panel alert service evaluates recent readings every ten seconds.

- Host samples older than approximately 30 seconds are excluded.
- Container samples are cached by `(server_id, container_id)` and expire after
  approximately 30 seconds.
- Server, application, and compose rules match only their own target type.
- A target ID of `0` means every target of that type.
- Sustained breaches use a sample window derived from `duration_seconds`.
- Notifications are delivered only through channels belonging to the rule's
  organization.
- Fired transitions are persisted in `alert_events` and exposed through the
  organization-scoped alert-events API.

Container disk-percentage alerts are not meaningful because Docker cgroup data
does not provide a writable-layer percentage against a defined capacity.

## gRPC metric and log API

The agent listens on `GRPC_PORT`, default `50051`, and serves definitions from
`proto/monitoring.proto`:

- `GetServerMetrics`
- `GetContainerMetrics`
- `StreamLogs`

Every request must include `x-metrics-token` metadata and the exact
`SERVER_ID`. Limits are bounded to prevent unbounded reads. `StreamLogs` tails
Docker logs directly and handles both TTY output and Docker's multiplexed
stdout/stderr framing.

Do not expose port `50051` publicly without network controls and transport
security. Token authentication prevents anonymous access, but TLS or a private
network/VPN is still recommended for remote hosts.

## Environment variables

| Variable | Default | Description |
|---|---|---|
| `SERVER_ID` | `1` | Exact existing `servers.id` represented by this agent. Explicitly set it in every real deployment. |
| `MONITOR_DATABASE_URL` | `sqlite://monitor.db` | Agent-local SQLite URL. Use a persistent volume in Docker. |
| `GRPC_PORT` | `50051` | Authenticated metric/log gRPC port. |
| `REFRESH_RATE` | `60` | Seconds between polling and push cycles. Must be greater than zero. |
| `RETENTION_DAYS` | `7` | Agent-local raw history retention. |
| `OPENOXIDE_SERVER_URL` | `http://127.0.0.1:4000` | Reachable base URL of the OpenOxide panel. |
| `METRICS_TOKEN` | none | Required agent secret. Must match a per-server or panel migration token. |
| `DOCKER_SOCKET` | `/var/run/docker.sock` | Docker Engine Unix socket. |
| `COLLECTION_MODE` | `auto` | `auto`, `cgroup`, or `stream`. |
| `INCLUDE_CONTAINERS` | empty | Comma-separated include patterns. Empty includes all. |
| `EXCLUDE_CONTAINERS` | empty | Comma-separated exclusion patterns. |
| `ROLLUP_SAMPLES` | `1` | Samples per average/peak rollup window. |
| `RUST_LOG` | `openoxide_monitor=info` | Rust tracing filter. |

Invalid critical configuration causes startup to fail with a clear message.
The agent requires a non-empty metrics token, a positive server ID, a valid
HTTP/HTTPS panel URL, a positive refresh/retention value, and a SQLite database
URL.

## Docker deployment

The published image is:

```text
dubeyanand/openoxide-monitor:latest
```

It is built as a static musl binary in a `scratch` image. It contains no shell
or package manager. This keeps the image small and reduces its attack surface.

Example compose service:

```yaml
services:
  openoxide-monitor:
    image: dubeyanand/openoxide-monitor:latest
    container_name: openoxide_monitor
    restart: unless-stopped
    network_mode: host
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - /sys/fs/cgroup:/sys/fs/cgroup:ro
      - monitor_data:/app/data
    environment:
      SERVER_ID: "5"
      OPENOXIDE_SERVER_URL: "http://127.0.0.1:4000"
      METRICS_TOKEN: "replace-with-a-random-secret"
      MONITOR_DATABASE_URL: "sqlite:///app/data/monitor.db"
      GRPC_PORT: "50051"
      REFRESH_RATE: "60"
      RETENTION_DAYS: "7"
      COLLECTION_MODE: "auto"
      ROLLUP_SAMPLES: "1"

volumes:
  monitor_data:
```

`network_mode: host` means `127.0.0.1` refers to the host network, allowing the
agent to reach a panel process listening on the same machine. Without host
networking, `127.0.0.1` inside the container refers to the monitoring container
itself; use the panel container/service name or `host.docker.internal` where
supported.

The Docker socket is mounted read-only at the filesystem level. Access to the
socket is still security-sensitive because Docker APIs can expose container
configuration and logs. Run the agent only on trusted hosts.

Start and inspect:

```bash
docker compose -f docker-compose.monitor.yml up -d --force-recreate
docker logs --tail=100 openoxide_monitor
```

## Running from source

```bash
just run-monitor
just test-monitor
just docker-build-monitor
just up-monitor
just logs-monitor
```

Or directly:

```bash
SERVER_ID=5 \
OPENOXIDE_SERVER_URL=http://127.0.0.1:4000 \
METRICS_TOKEN='replace-with-a-random-secret' \
cargo run -p openoxide_monitor
```

## Expected logs

Healthy startup resembles:

```text
starting openoxide monitor agent server_id=5 refresh_rate=60 grpc_port=50051
metric store initialized url="sqlite:///app/data/monitor.db"
gRPC query server listening addr=0.0.0.0:50051
collecting container metrics mode="cgroup" filter=all containers
```

Clean shutdown messages indicate normal Docker stop/restart behavior:

```text
shutdown signal received, stopping tasks
retention sweeper stopped
container collector stopped
host metric collector stopped
all tasks stopped cleanly
```

## Troubleshooting

### `could not reach panel`

The TCP connection could not be established.

Check:

```bash
curl http://127.0.0.1:4000/api/monitoring
```

Confirm the panel is running, the port is correct, and the URL is reachable
from the agent's network namespace.

### HTTP `401`

The panel is reachable but rejected authentication.

Verify:

- Agent `METRICS_TOKEN` matches its per-server token or panel global token.
- The panel process was restarted after changing `.env`.
- Docker compose recreated the container after changing its environment.
- No whitespace or quotes became part of the secret.

Inspect container configuration without printing the secret itself:

```bash
docker inspect openoxide_monitor \
  --format '{{range .Config.Env}}{{println .}}{{end}}' |
  grep -E '^(SERVER_ID|OPENOXIDE_SERVER_URL)='
```

### HTTP `500` immediately after authentication

The common cause is an invalid `SERVER_ID`. Confirm that it exists in the
panel's `servers` table. A foreign-key failure occurs if, for example, the agent
sends `SERVER_ID=1` but the panel only has IDs `5`, `10`, and `12`.

After changing compose environment values, recreate the container:

```bash
docker compose -f docker-compose.monitor.yml \
  up -d --force-recreate --no-build
```

### gRPC port already in use

Another agent/process is listening on `50051`. Stop the duplicate or configure
a different `GRPC_PORT`. When host networking is enabled there is no Docker port
mapping layer; the process binds directly on the host.

### Cgroup mode unavailable

Mount `/sys/fs/cgroup` read-only and verify the host uses cgroup v2. With
`COLLECTION_MODE=auto`, the agent automatically falls back to stream mode.

### No application/compose IDs

Ensure:

- `COLLECTION_MODE=stream` is used when Docker-label attribution is needed.
- Containers/services carry `com.openoxide.application-id` or
  `com.openoxide.compose-id`.
- The label value is a valid positive database ID.

### Database migration duplicate-column error in the panel

This means schema changes were manually applied but the SQLx migration ledger
was not updated. Do not repeatedly edit or rerun the migration blindly. Back up
the panel database, compare the complete schema created by that migration, and
reconcile `_sqlx_migrations` only when every migration object already exists.

## Source layout

```text
openoxide_monitor/src/
├── main.rs             startup, task lifecycle, graceful shutdown
├── config.rs           environment parsing and validation
├── context.rs          shared task dependencies
├── filter.rs           include/exclude pattern matching
├── rollup.rs           average and peak aggregation
├── collector/
│   ├── system.rs       host telemetry
│   └── cgroup.rs       scalable cgroup-v2 container collector
├── docker/
│   ├── api.rs          Docker Engine Unix-socket HTTP client
│   ├── cgroup.rs       cgroup file parsing
│   ├── json_lines.rs   streaming JSON framing
│   ├── stats.rs        CPU/memory/network/block calculations
│   ├── stream.rs       container discovery and live stats streams
│   └── types.rs        typed Docker payloads and identifiers
├── grpc.rs             authenticated metric/log gRPC service
├── logs.rs             Docker stdout/stderr demultiplexing
├── store.rs            agent SQLite schema and queries
└── tasks/
    ├── host.rs         host collection and local persistence
    ├── container.rs    container collection and local persistence
    ├── retention.rs    expired local history cleanup
    └── grpc.rs         authenticated gRPC server startup
```

## Production checklist

- Create/select the correct server in the panel.
- Set the exact corresponding `SERVER_ID` on that host.
- Generate a unique per-server monitoring token.
- Store secrets outside source-controlled compose files.
- Use HTTPS for panel ingestion across untrusted networks.
- Keep gRPC private or protect it with TLS/VPN/firewall rules.
- Persist `/app/data` on a Docker volume.
- Mount the Docker socket and cgroup filesystem read-only.
- Select `cgroup` for density or `stream` for richer live data.
- Configure include/exclude filters on busy hosts.
- Choose a rollup window appropriate for container count and retention.
- Confirm successful host and container pushes in logs.
- Verify the panel agent status becomes `ONLINE`.
- Test alert and notification delivery before relying on it operationally.

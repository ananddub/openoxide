# OpenOxide Monitor

`agent` is the host-side Agent for OpenOxide. One Agent
runs on every Docker server managed by the OpenOxide panel. It collects host and
container telemetry, keeps the only durable metric history on that node, and
exposes an authenticated gRPC API for panel metric and log queries.

This document explains the complete data flow, configuration, server identity,
authentication, storage, deployment, and troubleshooting model.

## System overview

```text
Docker host
┌──────────────────────────────────────────────────────────────┐
│ openoxide-agent                                               │
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

Container metrics can include `application_id` or `compose_id`. The panel
derives these from Docker labels when it reads agent container data:

```text
com.openoxide.application-id=<application database id>
com.openoxide.compose-id=<compose project database id>
```

An unattributed container sends zero for both IDs. It is still stored and shown
as a server/container metric, but resource-specific alert rules ignore it.

## Data model: nothing stored

The agent is a stateless query responder. Every metric the panel receives is
sampled live at request time from sysinfo and the Docker Engine API, over
reused keep-alive connections. The agent keeps no local database, no history,
and no buffers between requests; its only persistent state is open shell and
watch sessions.

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
`proto/agent.proto`:

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
| `SERVER_ID` | `1` | Exact existing `servers.id` represented by this agent (`-1` for the local agent). Explicitly set it in every real deployment. |
| `GRPC_PORT` | `50051` | Authenticated metric/log gRPC port. |
| `METRICS_TOKEN` | none | Required agent secret. Must match a per-server or panel migration token. |
| `DOCKER_SOCKET` | `/var/run/docker.sock` | Docker Engine Unix socket. |
| `MAX_UPLOAD_BYTES` | `1073741824` | Largest accepted `UploadFile` payload. |
| `UPLOAD_TIMEOUT_SECONDS` | `300` | Per-chunk idle timeout for uploads. |
| `UPLOAD_ROOTS` | `/tmp,/etc/openoxide,/app/workspace` | Absolute comma-separated directories `UploadFile` may write to. |
| `AGENT_TLS_CERT` | none | PEM server certificate path. Must be configured together with `AGENT_TLS_KEY`. |
| `AGENT_TLS_KEY` | none | PEM private-key path for the gRPC server certificate. |
| `RUST_LOG` | `agent=info` | Rust tracing filter. |

Invalid critical configuration causes startup to fail with a clear message.
The agent requires a non-empty metrics token, a valid `SERVER_ID` (positive or
`-1`), positive upload limits, and absolute upload roots.

## Docker deployment

The published image is:

```text
dubeyanand/openoxide-agent:latest
```

It is built as a static musl binary in a `scratch` image. It contains no shell
or package manager. This keeps the image small and reduces its attack surface.

Example compose service:

```yaml
services:
  openoxide-agent:
    image: dubeyanand/openoxide-agent:latest
    container_name: openoxide-agent
    restart: unless-stopped
    network_mode: host
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
    environment:
      SERVER_ID: "5"
      METRICS_TOKEN: "replace-with-a-random-secret"
      GRPC_PORT: "50051"
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
docker compose -f docker-compose.agent.yml up -d --force-recreate
docker logs --tail=100 openoxide-agent
```

## Running from source

```bash
just docker-build-agent
just up-agent
just logs-agent
just logs-agent
```

Or directly:

```bash
SERVER_ID=5 \
OPENOXIDE_SERVER_URL=http://127.0.0.1:4000 \
METRICS_TOKEN='replace-with-a-random-secret' \
cargo run -p agent
```

## Expected logs

Healthy startup resembles:

```text
starting openoxide agent server_id=5 grpc_port=50051
gRPC query server listening addr=0.0.0.0:50051
```

Clean shutdown messages indicate normal Docker stop/restart behavior:

```text
shutdown signal received, stopping tasks
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
docker inspect openoxide-agent \
  --format '{{range .Config.Env}}{{println .}}{{end}}' |
  grep -E '^(SERVER_ID|OPENOXIDE_SERVER_URL)='
```

### HTTP `500` immediately after authentication

The common cause is an invalid `SERVER_ID`. Confirm that it exists in the
panel's `servers` table. A foreign-key failure occurs if, for example, the agent
sends `SERVER_ID=1` but the panel only has IDs `5`, `10`, and `12`.

After changing compose environment values, recreate the container:

```bash
docker compose -f docker-compose.agent.yml \
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
agent/src/
├── main.rs                 startup and graceful shutdown
├── lib.rs                  crate surface
├── core/
│   ├── config.rs           environment parsing and validation
│   ├── context.rs          shared task dependencies
│   ├── error.rs            agent error types
│   └── middleware.rs       x-metrics-token authentication
├── api/                    generated tonic stubs + AgentService
├── app/grpc.rs             gRPC server startup (optional TLS)
├── services/
│   ├── monitoring/         metrics, stats streams, lifecycle watch,
│   │                       and shell/exec handlers (all live-sampled)
│   └── file_transfer.rs    streaming UploadFile with SHA-256 verify
└── utils/
    ├── docker/
    │   ├── transport/api.rs          keep-alive Docker Engine client
    │   ├── transport/json_lines.rs   streaming JSON framing
    │   └── query/types.rs            typed Docker payloads
    └── telemetry/collector/system    sysinfo host sampler
```

## Production checklist

- Create/select the correct server in the panel.
- Set the exact corresponding `SERVER_ID` on that host.
- Generate a unique per-server monitoring token.
- Store secrets outside source-controlled compose files.
- Use HTTPS for panel ingestion across untrusted networks.
- Keep gRPC private or protect it with TLS/VPN/firewall rules.
- Mount the Docker socket.
- Confirm successful queries in logs.
- Verify the panel agent status becomes `ONLINE`.
- Test alert and notification delivery before relying on it operationally.

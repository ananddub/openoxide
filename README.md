# Rustploy

Rustploy is a self-hosted deployment and infrastructure control panel written in Rust, with a React dashboard and a standalone Rust monitoring agent. It manages applications, Docker Compose projects, databases, domains, certificates, backups, remote servers, private networking, notifications, schedules, logs, and container operations from one panel.

The repository is organized as a Rust workspace. Reusable operating-system and command integrations live in dedicated crates, while the panel keeps API, business, persistence, and background-runtime responsibilities separate.

> Project status: active development. The backend and monitoring agent are functional, but APIs, migrations, and deployment behavior can still change between commits.

## Main capabilities

- Application deployment from Git repositories, Dockerfiles, Nixpacks, Railpack, Paketo/Cloud Native Buildpacks, Heroku buildpacks, and static sources.
- Docker Compose deployment, service management, mounts, patches, configs, secrets, imports, exports, and isolated deployments.
- PostgreSQL, MySQL, MariaDB, MongoDB, Redis, and LibSQL database lifecycle management.
- Remote server setup and auditing over SSH.
- Managed WireGuard private networking and support for external private-network providers.
- Docker Swarm and shared overlay-network management.
- Traefik routing, domains, redirects, middleware, TLS certificates, and ACME workflows.
- Deployment queues, live build streams, container logs, terminal sessions, rollback, and migration workflows.
- Database, volume, Compose configuration, and panel backup workflows.
- Standalone host/container monitoring agent with remote metric storage, retention, filtering, rollups, and gRPC queries.
- Alert rules and notification providers including email, webhook, Discord, Slack, Telegram, Teams, Gotify, ntfy, Pushover, Mattermost, Lark, and Resend.
- Authentication, sessions, 2FA, password reset, email verification, API tokens, organizations, permissions, and audit foundations.
- Scheduled jobs and centralized background runners.
- AI provider settings, generation workflows, structured validation, and log-analysis foundations.

## Architecture

```text
React dashboard
    │ HTTP / OpenAPI / SSE / Socket.IO
    ▼
Rustploy panel (Axum + Tokio)
    ├── API handlers and DTOs
    ├── business services
    ├── SQLx repositories and SQLite
    ├── deployment/build queue
    ├── background runners
    └── typed OS/Docker/SSH facade
            │
            ├── local Docker host
            └── remote servers over SSH/private networking

Monitoring agent (one per monitored host)
    ├── host and container collectors
    ├── local SQLite metric store
    ├── retention and rollups
    ├── panel event/summary pushes
    └── gRPC historical queries from the panel
```

### Request flow

```text
HTTP request
  → auto-registered controller
  → validated DTO and permission middleware
  → domain service
  → repository or typed OS/Docker operation
  → database/runtime result
  → JSON, SSE, WebSocket, or Socket.IO response
```

The intended responsibility boundaries are:

- `api/dto`: transport types, validation, and OpenAPI types.
- `api/handlers`: HTTP extraction, authorization, status mapping, and service calls.
- `services`: business rules and lifecycle orchestration.
- `db/repository`: database queries and persistence only.
- `utils/background`: long-running panel jobs and scheduling entry points.
- `crates/os`: reusable, typed operating-system, Docker, SSH, Git, WireGuard, and command builders.

## Workspace layout

```text
rustploy/
├── agent/                       # Standalone monitoring agent
├── crates/
│   ├── os/                      # Typed OS, Docker, Git, SSH and WireGuard facade
│   ├── sh_macros/               # Typed shell/script macros
│   └── auto/
│       ├── route/               # Controller registration/runtime
│       ├── route_macros/        # Route procedural macros
│       ├── socket/              # Socket handler registration/runtime
│       └── socket_macros/       # Socket procedural macros
├── db/
│   ├── migrations/              # Ordered runtime migrations
│   ├── schema/                  # Current schema representation
│   └── queries/                 # Sources for generated typed queries
├── proto/                       # Monitoring gRPC definitions
├── src/
│   ├── api/
│   │   ├── dto/                 # Request/response/OpenAPI types
│   │   ├── handlers/            # Auto-registered controllers
│   │   └── routes/              # Router, OpenAPI and Socket.IO wiring
│   ├── core/                    # Config, cache, errors, middleware and state
│   ├── db/
│   │   ├── connection/          # SQLite connection and migrations
│   │   ├── models/              # Generated/persisted models
│   │   └── repository/          # SQLx repositories
│   ├── services/                # Domain services and lifecycle orchestration
│   ├── utils/
│   │   ├── background/          # Background system startup and runners
│   │   ├── builder/             # Application, Compose, database and queue builders
│   │   ├── setup/               # Remote server setup/audit workflows
│   │   └── traefik/             # Traefik configuration builders
│   └── websocket/               # Terminal and streaming socket handlers
├── web/rustploy_react/          # React 19 dashboard
├── Dockerfile                   # Panel image definition
├── Dockerfile.monitor           # Static monitoring-agent image
├── docker-compose.monitor.yml   # Monitoring-agent deployment example
├── flake.nix                    # Reproducible Nix development shell
└── justfile                     # Common development commands
```

## Core backend domains

The `src/services` tree contains the product domains:

| Domain | Responsibility |
| --- | --- |
| `application` | Application CRUD, sources, deployment operations, ports, mounts, redirects, middleware, security, patches, networks, rollback and transfer |
| `compose` | Compose CRUD, source loading, deployment operations, service-level management and transfer |
| `database` | Database creation, configuration, credentials, import/export and runtime operations |
| `deployment` | Deployment records, Docker execution, logs, streams and lifecycle state |
| `server` | Remote server CRUD, SSH setup, maintenance, cleanup, migrations and private networking |
| `monitoring` | Agent authentication, lifecycle, health and panel-side monitoring access |
| `networking` | CDN operations, diagnostics and root-network repair |
| `traefik` | Traefik files, middleware, logs, health and version management |
| `certificate` | Certificate lifecycle, renewal and TLS integration |
| `backup` | Panel, database, volume, remote-file and Compose configuration backups |
| `notification` | Provider loading, message creation, delivery guards, scopes and triggers |
| `alert` | Metric readings, rule evaluation, state and incidents |
| `schedule` | Scheduled execution, retries, locking and notification integration |
| `git_provider` | GitHub/GitLab/Gitea/Bitbucket discovery, OAuth and webhooks |
| `preview` | Preview environment creation, queries, cleanup and lifecycle |
| `auth` | Login support services, sessions, 2FA, tokens, password reset and email verification |
| `organization` / `permission` | Membership, invitations, policies and resource access |
| `ai` | Provider settings, prompts, structured generation and analysis workflows |

## The `os` crate

`crates/os` is the reusable infrastructure layer. Product services should use its typed builders instead of assembling shell flags or Docker arguments directly.

Major modules include:

- `docker`: typed clients and handles for containers, Compose, images, networks, volumes, services, Swarm, system operations and queries.
- `exec`: local/remote command execution, cancellation, streaming, SSH authentication, host-key policy and script IR.
- `ssh`: typed SSH configuration and connection builders.
- `wireguard`: typed `wg`/`wg-quick` configuration, keys, interfaces, health and command builders.
- `git`: repositories and provider operations.
- `archive`: tar and zip creation, extraction, listing and sanitization.
- `file`, `dir`, `mount`, `symlink`: filesystem operations.
- `network`, `dns`, `firewall`, `port`: network and connectivity operations.
- `package`, `service`, `process`, `system`: host lifecycle operations.
- `gpu`, `cgroup`, `disk`, `resource`, `diagnostics`: host capability and resource management.
- `rclone`, `crypto`, `http`: remote storage, certificate/key utilities and HTTP operations.

The raw process invocation required to launch a binary belongs inside the relevant typed CLI implementation. Domain services should only select typed actions, values, and policies.

## Database and migrations

Rustploy uses SQLite through SQLx.

- Runtime migrations live in `db/migrations` and must remain append-only after release.
- `db/schema` represents the current expected schema.
- Compile-time SQLx macros use the database configured by `DATABASE_URL`.
- Generated models live in `src/db/models`.
- Generated and handwritten repositories live in `src/db/repository`.

When adding a database feature:

1. Add a new ordered migration; never rewrite a migration already used by installations.
2. Update the corresponding current-schema file.
3. Apply the migration to the local compile-time database.
4. Update or regenerate models/repositories if the persisted shape changed.
5. Use `sqlx::query!`, `query_as!`, or another compile-time checked macro where the query is static.
6. Run migration validation and the full Rust checks.

Useful commands:

```bash
atlas migrate hash --dir file://db/migrations
atlas migrate validate --dir file://db/migrations
just db-gen
```

`just db-gen` regenerates code and can modify many files. Review its diff before committing.

## Background systems

`BackgroundManager::start_all` starts the panel's long-running systems after dependency injection and before the HTTP listener begins serving:

- schedule runner;
- deployment/build queue;
- alert engine;
- deployment-log cleanup;
- scheduled panel backup;
- private-network reconciliation/health work;
- startup notification dispatch.

Background entry points live in `src/utils/background`; business behavior stays in the corresponding service.

## Remote servers and private networking

Remote servers are managed through SSH credentials stored by the panel. Server setup can audit and configure dependencies, Docker/Swarm, the shared network, Traefik, build tools, and monitoring.

Supported connection modes include:

- direct SSH;
- managed WireGuard;
- externally managed private networking such as Tailscale, NetBird, ZeroTier, or a custom provider.

Managed WireGuard lifecycle includes typed configuration generation, interface installation, health checks, route validation, repair, teardown, key rotation, rollback, and background reconciliation. Reusable `wg`/`wg-quick` behavior belongs in `crates/os/src/wireguard`; server-specific decisions belong in `src/services/server/private_network`.

## Monitoring architecture

The monitoring agent is the `agent` workspace package and is designed to run once per monitored host.

It performs four concurrent tasks:

1. Collect host metrics.
2. Collect Docker container metrics using cgroup or streaming mode.
3. Store metrics in the agent's local SQLite database.
4. Serve historical queries over gRPC while pruning expired metrics.

The remote agent database is the metric-history source. The panel authenticates agent traffic with `METRICS_TOKEN` and accesses summaries/history through monitoring services rather than duplicating the full metric history in the panel database.

### Agent collection modes

- `auto`: selects the best available collector.
- `cgroup`: reads cgroup data directly; suitable for hosts with many containers.
- `stream`: uses Docker streaming stats; provides fresher network I/O for smaller hosts.

Container filters support exact values and `*` wildcard patterns through `INCLUDE_CONTAINERS` and `EXCLUDE_CONTAINERS`.

## API and realtime interfaces

Controllers are registered through the `auto_route` macros. The router additionally exposes:

- OpenAPI document: `/openapi.json`
- Swagger UI: `/swagger-ui`
- Scalar API reference: `/scalar`
- Socket.IO layer for realtime events
- SSE endpoints for deployment and server-setup streams
- WebSocket terminal sessions

Request-duration middleware logs every request and promotes server errors or requests taking at least one second to warning-level logs.

## Configuration

Create a local `.env` file. Do not commit production secrets.

### Panel variables

| Variable | Default | Purpose |
| --- | --- | --- |
| `DATABASE_URL` | `sqlite:///db.sqlite3` | Panel SQLite database |
| `HOST` | `0.0.0.0` | HTTP bind address |
| `PORT` | `4000` | HTTP port |
| `SECRET_KEY` | development fallback | Legacy/general application secret; replace in production |
| `SOCKET_PATH` | `/var/run/docker.sock` | Docker socket |
| `BUILD_MEMORY_LIMIT` | `4G` | Default build memory limit |
| `BUILD_CPU_LIMIT` | `4` | Default build CPU limit |
| `METRICS_TOKEN` | empty | Shared monitoring-agent secret; empty disables authenticated ingestion |
| `RUSTPLOY_PUBLIC_URL` | unset | Public URL used in email/reset links |
| `RUSTPLOY_SERVER_URL` | `http://127.0.0.1:4000` in agent-related flows | URL agents and remote setup use to reach the panel |
| `TRAEFIK_BASE_PATH` | Rustploy data path + `/traefik` | Traefik configuration root |
| `DEPLOYMENT_PER_SERVER_CONCURRENCY` | code default | Parallel deployments allowed per server |
| `DEPLOYMENT_QUEUE_MAX_SIZE` | code default | Maximum queued deployment count |
| `JWT_ACCESS_SECRET` | application fallback/config | Access-token signing secret |
| `JWT_REFRESH_SECRET` | application fallback/config | Refresh-token signing secret |
| `JWT_ACCESS_EXPIRY_MINS` | code default | Access-token lifetime |
| `JWT_REFRESH_EXPIRY_DAYS` | code default | Refresh-token lifetime |

Production must use strong, different application/JWT secrets and a non-empty `METRICS_TOKEN`.

### Monitoring-agent variables

| Variable | Default | Purpose |
| --- | --- | --- |
| `SERVER_ID` | `1` | Existing panel server ID represented by this agent |
| `MONITOR_DATABASE_URL` | `sqlite://monitor.db` | Agent-local metric database |
| `GRPC_PORT` | `50051` | Historical-query gRPC port |
| `REFRESH_RATE` | `60` | Collection interval in seconds |
| `RETENTION_DAYS` | `7` | Local metric retention |
| `RUSTPLOY_SERVER_URL` | `http://127.0.0.1:4000` | Panel URL |
| `METRICS_TOKEN` | required | Must match the panel token |
| `DOCKER_SOCKET` | `/var/run/docker.sock` | Docker API socket |
| `COLLECTION_MODE` | `auto` | `auto`, `cgroup`, or `stream` |
| `INCLUDE_CONTAINERS` | empty | Optional comma-separated allow patterns |
| `EXCLUDE_CONTAINERS` | empty | Optional comma-separated deny patterns |
| `ROLLUP_SAMPLES` | `1` | Samples aggregated into one stored rollup |
| `RUST_LOG` | `agent=info` | Agent log filter |

The agent rejects invalid URLs, non-SQLite database URLs, non-positive retention/refresh values, invalid collection modes, and empty metric tokens at startup.

## Development setup

### Nix development shell

The recommended environment is the repository flake:

```bash
nix develop
```

It provides Rust, rustfmt, Clippy, rust-analyzer, SQLx CLI, Atlas, Docker CLI, WireGuard tools, DNS tools, Node.js, and other build dependencies. The shell also ensures a BuildKit container is available.

### Manual prerequisites

If not using Nix, install:

- current stable Rust toolchain with edition 2024 support;
- SQLite and SQLx CLI;
- Docker Engine and Docker CLI;
- Git;
- WireGuard/IPRoute2/DNS utilities for private-network development;
- Bun for the React dashboard;
- Atlas for migration validation;
- optional build tools used by the deployment drivers.

### Backend

```bash
export DATABASE_URL="sqlite://$(pwd)/data/db.sqlite3"
export METRICS_TOKEN="development-monitoring-token"
cargo run -p rustploy
```

The panel listens on `http://localhost:4000` unless `HOST` or `PORT` is changed.

With file watching:

```bash
just dev
```

### React dashboard

```bash
cd web/rustploy_react
bun install
bun run dev
```

The Vite development server listens on `http://localhost:3001`. Its current proxy target is configured in `web/rustploy_react/vite.config.ts`; update that target for your local panel when required.

Useful frontend commands:

```bash
bun run build
bun run test
bun run check:oxc
bun run check:tsc
bun run gen:api
```

### Monitoring agent

```bash
export SERVER_ID=1
export METRICS_TOKEN="development-monitoring-token"
export RUSTPLOY_SERVER_URL="http://127.0.0.1:4000"
cargo run -p agent
```

Or run the container example after editing its server ID and token:

```bash
docker compose -f docker-compose.monitor.yml up -d --build
docker compose -f docker-compose.monitor.yml logs -f
```

## Build and test

Run focused checks while developing and the workspace checks before merging:

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets
git diff --check
```

Common targeted commands:

```bash
cargo test -p os
cargo test -p agent
cargo test --lib server -- --nocapture
cargo test --lib wireguard -- --nocapture
```

Build release artifacts:

```bash
cargo build --release -p rustploy
cargo build --release -p agent
docker build -f Dockerfile.monitor -t rustploy-monitor:latest .
```

The root release profile optimizes for small binaries using size optimization, LTO, one codegen unit, abort-on-panic, and symbol stripping.

## Docker notes

The monitoring image is a static musl binary copied into a `scratch` image. It needs access to the Docker socket, cgroups for the scalable collector, persistent storage for its SQLite database, and network reachability to the panel.

`docker-compose.monitor.yml` uses host networking, so `RUSTPLOY_SERVER_URL=http://127.0.0.1:4000` reaches a panel running on the same host and `GRPC_PORT` binds directly to the host.

The panel image needs persistent database/data storage and Docker socket access. Docker socket access is equivalent to high privilege over the Docker host; only run Rustploy on trusted infrastructure and protect the panel with authentication and network controls.

> The root `Dockerfile` still references the previous frontend directory (`web/rustploy`). The active dashboard is `web/rustploy_react`; align the frontend build stage before using that Dockerfile for a production panel image.

## Development rules

These conventions keep the codebase maintainable:

- Keep `mod.rs` focused on declarations and re-exports.
- Split a domain into a folder when multiple responsibilities/files emerge.
- Keep handlers thin; database access belongs in repositories and lifecycle logic belongs in services.
- Prefer enums and typed builders over raw string states, actions, command names, and flags.
- Use `crates/os` for reusable operating-system and command behavior.
- Never assemble user-controlled shell command fragments directly.
- Prefer compile-time checked SQLx macros for static queries.
- Keep runtime migrations append-only and keep `db/schema` synchronized with the current schema.
- Keep background entry points together under `src/utils/background`.
- Preserve API serialization compatibility when replacing strings with enums.
- Add focused tests for validation, builders, state transitions, rollback, retry, and reconciliation logic.

## Security model

- The panel is self-hosted and stores operational state in SQLite.
- SSH connections support key authentication and host-key pinning; accepting unknown host keys should only be used during controlled setup.
- Monitoring ingestion requires a shared token.
- WireGuard private keys and temporary configuration content are handled through typed lifecycle code and zeroized where applicable.
- Core dumps are disabled at panel startup to reduce secret leakage from process memory.
- Paths used by archive, Traefik, backup, and file operations are validated before execution.
- Docker socket access is inherently privileged and must be treated as host-root-equivalent access.

## Troubleshooting

### SQLx reports `no such table`

Compile-time SQLx macros inspect the database referenced by `DATABASE_URL`. Apply the missing migration to the local development database, verify the table exists, and rerun the build. Do not rewrite an already released migration to solve a local database mismatch.

### Migration `VersionMismatch`

The local database migration history does not match the migration files. Determine whether the database or migration directory is stale before changing anything. Back up the database before repairing migration history.

### Monitoring agent cannot reach the panel

- Do not use `127.0.0.1` from a normal bridge-network container to reach a panel on the host.
- Use host networking, a host-gateway address, or a routable panel URL.
- Confirm `METRICS_TOKEN` matches on both sides.
- Confirm the configured `SERVER_ID` exists.
- Check panel ingestion logs and agent startup validation output.

### Monitoring endpoints return `500`

Verify migrations, the referenced server row, authentication token, and panel/agent version compatibility. The agent's historical metric database and the panel's product database have different responsibilities.

### WireGuard hostname does not resolve

WireGuard routes IP packets; it does not automatically provide DNS names. Configure the selected private-network provider's DNS, an explicit private DNS record, or use the tunnel IP directly.

## API discovery

With the panel running locally:

- OpenAPI JSON: `http://localhost:4000/openapi.json`
- Swagger UI: `http://localhost:4000/swagger-ui`
- Scalar: `http://localhost:4000/scalar`

The React client types can be regenerated from the OpenAPI document with `bun run gen:api` after pointing its script at the correct panel URL.

## Contributing

Before submitting a change:

1. Keep the change inside the correct architectural layer.
2. Add or update migrations and current schema together when persistence changes.
3. Add focused tests.
4. Run Rust and frontend checks relevant to the change.
5. Review generated or bulk-formatted diffs carefully.
6. Avoid committing local databases, secrets, build artifacts, or environment-specific proxy URLs.

## License

Add the project's chosen license file before public distribution. The previous README claimed MIT, but no repository license file is currently present, so this README does not make a license claim.

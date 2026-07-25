# Rustploy

A modern deployment and DevOps platform built with **Rust**, **Axum**, and **Tokio**, featuring automatic route discovery, dependency injection, and real-time WebSocket support via Socket.IO.

## What This Is

Rustploy is a comprehensive backend system for managing containerized deployments, server resources, and infrastructure orchestration. It provides REST APIs for project management, deployment tracking, server monitoring, and cron scheduling—all backed by SQLite and designed to integrate with Traefik v3 for dynamic routing configuration. The architecture leverages Spring-style automatic routing and dependency injection patterns adapted for Rust's type system, making it highly modular and testable.

### Stack

- **Language(s):** Rust (Edition 2024)
- **Framework / runtime:** Axum 0.8 + Tokio (async runtime)
- **Notable libraries:**
  - `auto_route` + `auto_di` – Spring-style automatic controller registration and dependency injection
  - `auto_socket` – Socket.IO event handler registration with DI support
  - `sqlx` – Compile-time checked SQL queries with SQLite
  - `poem-openapi` + `scalar_api_reference` – OpenAPI documentation and API reference UI
  - `socketioxide` – Socket.IO server integration for real-time updates

## How It's Organized

```
src/
  api/            REST API handlers (projects, deployments, servers, schedules, traefik)
  core/           Configuration, logging, and core utilities
  db/             SQLx models, migrations, and CRUD repositories
  services/       Business logic (deployment scheduling, build queue, status tracking)
  utils/          Helper modules (shell execution, build queue, config parsing)
  websocket/      Socket.IO event handlers and real-time updates

auto_route/            Proc-macro crate for Spring-style HTTP routing in Axum
auto_route_macros/     Procedural macro implementation
auto_socket/           Socket.IO event handler auto-registration
auto_socket_macros/    Socket.IO macro implementation
rustploy_sh_macros/    Shell command execution macros

db/                    SQL migration and schema files
web/                   Frontend project directory
justfile               Development task runner
Cargo.toml             Workspace configuration
flake.nix             Nix development environment
```

### How It Fits Together

The application is structured as an Axum web server with automatic route discovery via `auto_route` macros and dependency injection via `auto-di`. At startup (`src/main.rs`), the DI container resolves controllers annotated with `#[controller]`, combines their routes into a single `Router`, and serves them on a configurable host/port. Background services like `ScheduleRunner` (cron jobs) and `BuilderQueue` (async deployment builds) run concurrently. Socket.IO handlers registered via `#[auto_socket]` enable real-time metrics streaming (e.g., CPU, RAM, disk usage) to connected WebSocket clients. All database queries use compile-time verified SQLx, ensuring type safety at build time.

## How to Run It

### Prerequisites

- Rust 1.85+ (via rustup)
- SQLite (included in most systems)
- Tokio runtime (async)
- Optional: Nix (for development environment)

### Setup & Run

```bash
# Clone the repository
git clone https://github.com/raobadalyadav11/rustploy.git
cd rustploy

# Load environment variables
cp .env.example .env  # adjust DATABASE_URL and other vars as needed
export $(cat .env | xargs)

# Generate module structure and database models
just gen-mod
just db-gen
just query-gen

# Run the application
cargo run
# or via justfile
just run

# The server will start on the configured host:port (default: 127.0.0.1:3000)
```

### Key Environment Variables

- `DATABASE_URL` – SQLite connection string (e.g., `sqlite:///path/to/db.sqlite3`)
- `HOST` – Server bind address (default: `127.0.0.1`)
- `PORT` – Server port (default: `3000`)
- Additional config in `.env` (see `.env` for examples)

### Development Commands

```bash
# Watch for changes and rebuild
just run

# Generate OpenAPI documentation
cargo build --features openapi

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## API Overview

The REST API provides endpoints for:

- **Projects** (`GET /api/projects`, `POST /api/projects`) – Manage project containers
- **Deployments** (`GET /api/deployments`, `POST /api/deployments/trigger`) – Track and trigger builds
- **Servers** (`GET /api/servers`, `GET /api/servers/metrics/realtime`) – Monitor remote nodes and stream metrics
- **Schedules** (`GET /api/schedules`) – View configured cron tasks
- **Traefik Config** (`GET /api/traefik/configs`) – List dynamic routing configurations

Full API contract: see [`BACKEND_API_CONTRACT.md`](./BACKEND_API_CONTRACT.md)

### OpenAPI / Swagger

- **Swagger UI:** `GET /swagger-ui`
- **Scalar API Reference:** `GET /api/reference`
- **OpenAPI JSON:** `GET /openapi.json`

## Key Features

### 1. Automatic Route Discovery (`auto_route`)

Define controllers with `#[controller]` and methods with `#[get]`, `#[post]`, etc. Routes are discovered at compile time via the `inventory` crate and registered at runtime:

```rust
#[controller("/api/projects")]
impl ProjectController {
    fn new(service: Arc<ProjectService>) -> Self { /* ... */ }

    #[get]
    async fn list(&self) -> impl IntoResponse {
        // List projects
    }

    #[post]
    async fn create(&self, Json(req): Json<CreateProjectReq>) -> impl IntoResponse {
        // Create project
    }
}
```

### 2. Dependency Injection (`auto-di`)

Controllers and services are registered as singletons via the `#[singleton]` attribute. The DI container resolves dependencies at startup:

```rust
#[singleton]
impl ProjectService {
    fn new(db: Arc<Database>) -> Self { /* ... */ }
}
```

### 3. Real-Time WebSocket Support (`auto_socket`)

Socket.IO handlers auto-register for real-time metric streaming:

```rust
#[auto_socket("/metrics")]
impl MetricsSocket {
    fn new() -> Self { /* ... */ }

    #[on("subscribe")]
    async fn subscribe(&self, socket: SocketRef) {
        // Send real-time CPU/RAM/disk updates
    }
}
```

### 4. Type-Safe Database Queries (`sqlx`)

Queries are checked at compile time against the live database schema:

```rust
let project = sqlx::query_as::<_, Project>(
    "SELECT * FROM projects WHERE id = ?"
)
.bind(id)
.fetch_one(&self.db)
.await?;
```

### 5. OpenAPI Generation

Routes automatically generate OpenAPI specs via `poem-openapi`, with Swagger UI and Scalar API Reference frontends served at standard paths.

## Try Asking

- **How do I add a new API endpoint?** — Use `#[controller]` and `#[get]`/`#[post]`/etc. macros; see `auto_route` crate README.
- **Where are deployment build logs stored?** — Check `services/` and `utils/exec/` for build queue and script execution; logs may stream via WebSocket or be stored in the database.
- **How do I configure cron jobs?** — Define schedules in the `ScheduleRunner` service and persist them in the database; see `services/schedule.rs`.
- **What's the relationship between `auto_route` and `auto_route_macros`?** — `auto_route` is the public API; `auto_route_macros` contains the proc-macro implementation (required by Rust's macro crate system).
- **Can I use Socket.IO with my controller?** — Yes, define a struct with `#[auto_socket]` and register event handlers via `#[on]`; both auto_route and auto_socket coexist in the same app.

## License

MIT

---

**Repository**: [raobadalyadav11/rustploy](https://github.com/raobadalyadav11/rustploy)  
**Original**: [ananddub/rustploy](https://github.com/ananddub/rustploy)

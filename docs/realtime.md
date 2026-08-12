# Typed realtime endpoints

OpenOxide can expose an Axum controller method as a typed HTTP route and a React live data source at the same time. SQLite-backed endpoints can refresh automatically after committed writes, without calling `publish` from every mutation handler.

## Basic endpoint

```rust
use auto_route::controller;
use axum::Json;

#[controller("/api/todos")]
impl TodoController {
    #[get("")]
    #[live("todos", table = "todos")]
    async fn list(&self) -> Json<Vec<Todo>> {
        Json(
            sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
                .fetch_all(&self.pool)
                .await
                .unwrap_or_default(),
        )
    }
}
```

This declaration generates:

- the normal `GET /api/todos` route;
- the live endpoint identity `TodoController::list`;
- the server publisher `TodoController::todos()`;
- the typed subscription descriptor `TodoController::todos_subscription()`;
- the React hook name `useTodos` in the generated client manifest;
- an automatic refresh resolver associated with the `todos` SQLite table.

## Mutation handlers

Mutation handlers only write to SQLite:

```rust
#[post("")]
async fn create(&self, Json(input): Json<NewTodo>) -> StatusCode {
    sqlx::query("INSERT INTO todos(title) VALUES (?)")
        .bind(input.title)
        .execute(&self.pool)
        .await
        .expect("insert todo");

    StatusCode::CREATED
}
```

Do not query the list again or call `publish` after the write. The SQLite commit hook performs the refresh.

## Multiple tables

Use `tables` when the result depends on more than one table:

```rust
#[get("")]
#[live(
    "dashboard",
    tables = ["users", "projects", "deployments"]
)]
async fn dashboard(&self) -> Json<Dashboard> {
    Json(self.service.dashboard().await)
}
```

A committed change to any listed table schedules the endpoint resolver. If one transaction changes several listed tables, the endpoint still appears once in the refresh registry for each notification cycle and its in-flight guard prevents parallel duplicate refreshes.

The single-table shorthand remains supported:

```rust
#[live("todos", table = "todos")]
```

## Runtime flow

```text
SQLite INSERT / UPDATE / DELETE
              │
              ▼
       pre-update hook records
       table and row metadata
              │
              ▼
       successful commit hook
              │
              ▼
       DbEventBus publishes the
       committed table names
              │
              ▼
       matching live resolver
       executes the route once
              │
              ▼
       one typed payload is sent
       to the endpoint room
              │
              ▼
       every subscribed client
       receives the same update
```

Rollback clears pending changes and does not refresh endpoints. A short post-commit delay ensures the new rows are visible to SQLx readers.

## Query and connection behavior

Subscriber count does not control query count. Ten clients subscribed to the same zero-argument endpoint share one server refresh:

```text
1 committed database change
1 endpoint resolver execution
1 SELECT query
1 serialized live payload
10 socket deliveries
```

Each browser namespace also reuses a shared Socket.IO connection. Multiple generated React hooks create separate logical subscriptions, not separate connections.

Rapid commits are guarded per endpoint. If a resolver is already running, another notification marks it pending instead of starting a parallel query. After the active resolver completes, one more refresh reads the latest state.

## React usage

### Vite generation setup

Install and configure the plugin:

```ts
// vite.config.ts
import react from '@vitejs/plugin-react';
import {defineConfig} from 'vite';
import {openoxide} from '@openoxide/vite';

export default defineConfig({
  plugins: [
    react(),
    openoxide({
      manifestPath: '../Cargo.toml',
      manifestBin: 'openoxide-live-manifest',
      declarations: 'src/openoxide-live.generated.d.ts',
    }),
  ],
});
```

All options are optional. Without `manifestPath`, the plugin walks upward from the Vite root and uses the nearest `Cargo.toml`.

The paths mean:

| Option | Purpose | Default |
| --- | --- | --- |
| `manifestPath` | Cargo manifest containing the generator binary; relative to the Vite root | nearest parent `Cargo.toml` |
| `manifestBin` | Rust binary that prints the live JSON manifest to stdout | `openoxide-live-manifest` |
| `declarations` | Generated TypeScript declaration path, relative to the Vite root | `src/openoxide-live.generated.d.ts` |

At Vite startup the plugin executes:

```text
cargo run --quiet --manifest-path <manifestPath> --bin <manifestBin>
```

The binary must print a JSON manifest. The plugin then creates two client outputs:

```text
virtual:openoxide-live
    generated runtime module held in Vite memory

src/openoxide-live.generated.d.ts
    generated TypeScript types written to disk
```

There is intentionally no generated `live.ts` runtime file. Application code imports the virtual module:

```tsx
import {type Todo, useTodos} from 'virtual:openoxide-live';
```

The declaration file should be included by the application's `tsconfig.json` through its normal `src` include. Do not edit it manually.

Example generator binary:

```rust
// src/bin/openoxide-live-manifest.rs
use crate::{controller::TodoController, models::Todo};

fn main() {
    let subscription = TodoController::todos_subscription()
        .expect("generate todos subscription");

    println!("{}", serde_json::json!({
        "types": [{
            "name": "Todo",
            "definition": Todo::TYPESCRIPT
        }],
        "endpoints": [{
            "hook": "useTodos",
            "namespace": subscription.namespace(),
            "endpoint": subscription.endpoint(),
            "event": subscription.event(),
            "parameters": "",
            "result": "Todo[]"
        }]
    }));
}
```

The Vite plugin reads the generated live manifest and creates a typed hook:

```tsx
function TodoList() {
  const {data: todos, loading, connected} = useTodos();

  if (loading) return <p>Loading…</p>;

  return (
    <ul>
      {todos?.map(todo => <li key={todo.id}>{todo.title}</li>)}
    </ul>
  );
}
```

The hook subscribes with the compile-time endpoint identity and decodes updates as `Todo[]`. Endpoint strings and payload types do not need to be duplicated in application code.

## Explicit publishing

Explicit publishing remains available for data that is not driven by SQLite, such as filesystem notifications, metrics, jobs, or broadcast channels:

```rust
TodoController::todos()?
    .publish(todos)
    .await?;
```

## Delivery strategies

`#[live]` supports compile-time checked delivery strategies:

```rust
#[live("todos", strategy = sqlite, table = "todos")]
#[live("metrics", strategy = latest)]
#[live("logs", strategy = stream, capacity = 512, replay = 50)]
#[live("status", strategy = publish)]
```

| Strategy | Behavior | Intended data |
| --- | --- | --- |
| `sqlite` | A committed matching table change reruns the endpoint; parallel refreshes are coalesced | CRUD lists and database dashboards |
| `publish` | Every `.publish(data).await` directly emits the supplied payload | Normal application events and already-computed data |
| `latest` | Publishing replaces any queued value, retains the current value in memory, and sends it immediately to new subscribers | CPU, RAM, progress and current status |
| `stream` | Publishing enters a bounded ordered queue, applies backpressure, and can retain a bounded replay window | Logs, terminal output and ordered job events |

Defaults keep common declarations short:

```rust
#[live("todos", table = "todos")] // strategy = sqlite
#[live("status")]                 // strategy = publish
```

`stream` defaults to a capacity of 256 and `replay = 0`. A positive replay value retains that many recent events per endpoint-and-arguments room. A newly subscribing socket receives the retained events in order before continuing with live delivery; existing subscribers do not receive duplicates. Replay is in-memory, bounded to 10,000 events at compile time, and disappears when the process restarts.

`capacity` and `replay` are rejected for every non-stream strategy. `table` and `tables` are accepted only by `sqlite`, and explicitly choosing `sqlite` without a table is a compile error.

The strategy changes server delivery behavior only. React still consumes the same generated hook and `live:update` protocol.

Target one socket:

```rust
dashboard_live::metrics_event().emit(&socket, metrics)?;
```

Broadcast to the namespace:

```rust
dashboard_live::metrics_event()
    .broadcast(metrics)
    .await?;
```

## Current constraints

- Automatic `table`/`tables` refresh currently supports zero-argument controller methods returning `Json<T>`.
- Parameterized endpoints can still use generated typed rooms and explicit `publish` calls.
- Writes performed through the configured SQLite connections are observed. A separate process writing the database file does not pass through the in-process SQLx hooks.
- Table names are explicit metadata. OpenOxide does not attempt to parse arbitrary SQL or infer dependencies from service code.
- SQLite `SELECT` statements do not trigger the write hooks, so refresh queries do not create a reactive loop.

## Server setup

SQLx must enable SQLite pre-update hooks:

```toml
sqlx = {
  version = "0.9",
  default-features = false,
  features = ["runtime-tokio", "sqlite", "sqlite-preupdate-hook"]
}
```

Create Socket.IO and register generated handlers before serving the app:

```rust
let app = auto_route::routes().await?;
let (socket_layer, io) = socketioxide::SocketIo::new_layer();
auto_socket::register_global(&io).await?;
let app = app.layer(socket_layer);
```

The application SQLite pool installs `crate::db::reactive::install_hooks` in `after_connect`, ensuring every pooled connection participates in committed-change detection.

## Authentication and parameter-driven scope

The main OpenOxide backend configures an `AuthService` adapter for Socket.IO. The generated React runtime reads `openoxide-auth-session`, sends its access token in the Socket.IO handshake, and reconnects with the current token.

Controller parameters determine whether a live endpoint is private:

```rust
#[live("notifications", strategy = latest)]
async fn notifications(&self, claims: Claims) -> Json<Vec<Notification>> {
    Json(self.repository.for_user(claims.user.user_id).await)
}
```

`Claims` is injected by the server and is not part of the React hook arguments:

```tsx
const notifications = useNotifications();
```

Server publishing requires the verified claims value and targets only that user's room:

```rust
NotificationController::notifications(&claims)?
    .publish(notifications)
    .await?;
```

`RequirePermission<Resource, Operation>` is also treated as a server-only parameter. Before joining the room, the live runtime resolves the user's organization and calls the existing `PermissionService`. A denied or failed check does not join the room and receives no latest/replay payload.

Public endpoints without `Claims` or `RequirePermission` retain an unscoped room. Authenticated rooms use an internal `user:<id>:<endpoint>:<args>` key, so equal client arguments from different users cannot collide.

Authenticated `strategy = sqlite` endpoints are currently rejected at compile time because correct refresh requires executing the query once per subscribed user identity. Use an explicit authenticated `publish`/`latest`/`stream` endpoint until the per-user SQLite resolver registry is added.

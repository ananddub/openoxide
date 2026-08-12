# OpenOxide reactive Todo test

Run from the repository root:

```bash
nix develop -c cargo run -p openoxide-todo-test
```

The controller declares its HTTP and live behavior together:

```rust
#[get("")]
#[live("todos", table = "todos")]
async fn list(&self) -> Json<Vec<Todo>> { /* query */ }
```

Create, toggle, and delete handlers only execute SQL. SQLite pre-update/commit hooks detect committed `todos` writes, the generated resolver executes the list query once, and Socket.IO sends the same typed payload to every subscribed browser.

The test database is `crates/todo-test/todo-test.sqlite3` and is ignored by the
repository's `*.sqlite3` rule.
# React realtime Todo test

Terminal 1:

```bash
just todo-dev
```

Terminal 2:

```bash
cd crates/todo-test/react
bun install
bun run dev
```

The React Vite config points generation at the Todo Rust crate:

```ts
openoxide({
  manifestPath: '../Cargo.toml',
  declarations: 'src/openoxide-live.generated.d.ts',
})
```

The runtime hooks are imported from `virtual:openoxide-live`; only their TypeScript declarations are written to disk.

Open `http://127.0.0.1:3101` in two tabs. Creating, toggling, or deleting in one tab publishes the updated typed todo list to both tabs without refetching.

Expected server behavior for two or more tabs:

```text
one SQLite commit -> one list query -> one publish -> every tab updates
```

The generated React hook is:

```tsx
const {data: todos, loading, connected} = useTodos();
```

The page also exercises the non-SQLite strategies:

- **Burst 100 metrics** uses `strategy = latest` and should settle on sample `#100`.
- **Stream 20 events** uses `strategy = stream, capacity = 32` and visibly advances from event `#1` through `#20` in order.
- Creating, toggling, or deleting a Todo uses `strategy = sqlite` implicitly and updates every open tab without a manual publish call.

See [the complete realtime guide](../../docs/realtime.md).

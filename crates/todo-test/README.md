# Rustploy reactive Todo test

Run from the repository root:

```bash
nix develop -c cargo run -p rustploy-todo-test
```

Open `http://127.0.0.1:3100/todo` in two browser tabs. The app uses the real
`auto_route` controller macros. SQLite pre-update/commit hooks automatically
publish the `todos` table change, so both tabs re-run their reactive slot.

The test database is `crates/todo-test/todo-test.sqlite3` and is ignored by the
repository's `*.sqlite3` rule.

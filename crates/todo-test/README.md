# Rustploy reactive Todo test

Run from the repository root:

```bash
nix develop -c cargo run -p rustploy-todo-test
```

Open `http://127.0.0.1:3100` in two browser tabs. Adding, toggling, or deleting a
todo publishes a `todos` table change and both tabs re-run their reactive slot.

The test database is `crates/todo-test/todo-test.sqlite3` and is ignored by the
repository's `*.sqlite3` rule.

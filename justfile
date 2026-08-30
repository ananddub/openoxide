
gen-mod:
    #!/usr/bin/env bash
    set -euo pipefail

    # Add "pub mod <name>;" if not already present
    add_if_missing() {
        local mod_file="$1"
        local name="$2"
        local entry="pub mod ${name};"
        if grep -qxF "$entry" "$mod_file" 2>/dev/null; then
            echo "  [skip]   $mod_file  <-  $entry"
        else
            echo "$entry" >> "$mod_file"
            echo "  [added]  $mod_file  <-  $entry"
        fi
    }

    # Remove stale "pub mod <name>;" lines whose file/dir no longer exists
    remove_stale() {
        local mod_file="$1"
        local parent_dir="$2"
        [[ ! -f "$mod_file" ]] && return

        local tmpfile
        tmpfile="$(mktemp)"

        while IFS= read -r line; do
            # Only inspect "pub mod <name>;" lines
            if [[ "$line" =~ ^pub\ mod\ ([a-zA-Z0-9_]+)\;$ ]]; then
                local name="${BASH_REMATCH[1]}"
                # Valid if matching .rs file OR sub-directory exists
                if [[ -f "$parent_dir/${name}.rs" || -d "$parent_dir/${name}" ]]; then
                    echo "$line" >> "$tmpfile"
                else
                    echo "  [removed] $mod_file  <-  $line  (not found)"
                fi
            else
                echo "$line" >> "$tmpfile"
            fi
        done < "$mod_file"

        mv "$tmpfile" "$mod_file"
    }

    # ── Walk every sub-directory under src (depth-first via sort) ──────────────
    while IFS= read -r dir; do
        mod_file="$dir/mod.rs"
        touch "$mod_file"

        # 1. Remove stale entries first
        remove_stale "$mod_file" "$dir"

        # 2. Add .rs files (skip mod.rs itself)
        while IFS= read -r f; do
            name="$(basename "${f%.rs}")"
            add_if_missing "$mod_file" "$name"
        done < <(find "$dir" -maxdepth 1 -type f -name '*.rs' ! -name 'mod.rs' | sort)

        # 3. Add immediate sub-directories
        while IFS= read -r subdir; do
            name="$(basename "$subdir")"
            add_if_missing "$mod_file" "$name"
        done < <(find "$dir" -mindepth 1 -maxdepth 1 -type d | sort)

    done < <(find src -mindepth 1 -type d | sort)

    # ── Update src/lib.rs with top-level modules ───────────────────────────────
    touch src/lib.rs

    # Remove stale top-level entries
    remove_stale "src/lib.rs" "src"

    # Add missing top-level dirs
    while IFS= read -r dir; do
        name="$(basename "$dir")"
        add_if_missing "src/lib.rs" "$name"
    done < <(find src -mindepth 1 -maxdepth 1 -type d | sort)

# Generate typed sqlx query methods from db/queries/*.sql → src/db/queries/
query-gen:
    bash scripts/query-gen.sh


run:
    cargo run --bin openoxide

# Run the standalone reactive Todo integration test app
todo-dev:
    cargo run -p openoxide-todo-test

todo-react:
    cd crates/todo-test/react && bun run dev

# Run the React frontend dev server
web:
    cd web/openoxide_react && bun run dev

dev:
    sudo pkill openoxide || true
    cargo watch -x check -x 'run --bin openoxide'
# Build the standalone Agent image
docker-build-agent:
    docker build -f Dockerfile.agent -t openoxide-agent:latest .

# Start the Agent container
up-agent:
    docker compose -f docker-compose.agent.yml up -d --build

# Stop the Agent container
down-agent:
    docker compose -f docker-compose.agent.yml down

# Tail the Agent logs
logs-agent:
    docker compose -f docker-compose.agent.yml logs -f

# Start the main OpenOxide compose stack (builder/panel)
compose-up:
    docker compose up -d --build

# Stop the main OpenOxide compose stack
compose-down:
    docker compose down

# Tail the main OpenOxide compose logs
compose-logs:
    docker compose logs -f

# Short aliases for compose up / down / logs
up: compose-up
down: compose-down
logs: compose-logs

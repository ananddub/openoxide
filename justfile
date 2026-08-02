
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

# Generate Rust models + CRUD repositories from the SQLite database
db-gen:
    #!/usr/bin/env bash
    set -euo pipefail
    DB_URL="${DATABASE_URL:-sqlite:///run/media/das/SSD/Devloper/rustploy/data/db.sqlite3}"

    echo ">> Generating entities from $DB_URL ..."
    sqlx-gen generate entities \
        -u "$DB_URL" \
        -o src/db/models \
        -x _sqlx_migrations

    echo ">> Patching derives: removing Eq from files with f32/f64 fields ..."
    grep -rl "f64\|f32" src/db/models/ | xargs sed -i 's/, Eq,/, /g; s/, Eq)/)/g; s/(Eq, /(/g'

    echo ">> Suppressing warnings in generated files ..."
    for dir in src/db/models src/db/repository; do
        mod="$dir/mod.rs"
        grep -qx '#![allow(warnings)]' "$mod" || sed -i '1s/^/#![allow(warnings)]\n/' "$mod"
    done

    echo ">> Generating CRUD repositories ..."
    for f in src/db/models/*.rs; do
        name="$(basename "$f")"
        [[ "$name" == "mod.rs" || "$name" == "types.rs" ]] && continue
        sqlx-gen generate crud \
            -f "$f" \
            -d sqlite \
            -m '*' \
            -o src/db/repository
    done

    echo ">> Fixing import paths in generated repositories ..."
    sed -i 's/crate::models::/crate::db::models::/g' src/db/repository/*.rs
    sed -i 's/query_as::<\([^>]*\)>(\&sql)/query_as::<\1>(sqlx::AssertSqlSafe(\&*sql))/g' src/db/repository/*.rs

    echo ">> Done!"

run:
    cargo run
dev:
    sudo pkill rustploy || true
    cargo watch -x check -x run
# Build standalone rustploy_monitor binary
build-monitor:
    cargo build --release -p rustploy_monitor

# Run standalone rustploy_monitor binary locally
run-monitor:
    cargo run -p rustploy_monitor

# Run the monitoring agent's test suite
test-monitor:
    cargo test -p rustploy_monitor

# Build standalone rustploy_monitor Docker image
docker-build-monitor:
    docker build -f Dockerfile.monitor -t rustploy-monitor:latest .

# Start the monitoring agent as a container
up-monitor:
    docker compose -f docker-compose.monitor.yml up -d --build

# Stop the monitoring agent container
down-monitor:
    docker compose -f docker-compose.monitor.yml down

# Tail the monitoring agent's logs
logs-monitor:
    docker compose -f docker-compose.monitor.yml logs -f
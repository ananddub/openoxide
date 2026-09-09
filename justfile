# ── Local dev ────────────────────────────────────────────────────────────────

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

# Run the panel binary (openoxide) in the foreground. Builds the web bundle
# first — the SPA is RustEmbed'ed from web/openoxide_react/dist, so a stale
# dist means the panel serves an outdated dashboard.
run: web-build
    cargo run --bin openoxide

# Build the web bundle and mark the embed module dirty. Release builds embed
# dist at compile time, so dashboard.rs must be touched for cargo to pick up
# new assets; debug builds read dist from disk at runtime (touch is harmless).
web-build:
    cd web/openoxide_react && bun run build
    touch src/api/dashboard.rs

# Type-check the frontend only (no bundle emitted).
web-check:
    cd web/openoxide_react && bunx tsc --noEmit

# Build everything natively: panel (openoxide) + agent + web bundle.
build-native: web-build
    cargo build --bin openoxide --bin agent

# Build all dev images (panel + agent).
build-images:
    docker compose -f docker-compose.dev.yml build

# Run the standalone reactive Todo integration test app
todo-dev:
    cargo run -p openoxide-todo-test

todo-react:
    cd crates/todo-test/react && bun run dev

# Run the React frontend dev server
web:
    cd web/openoxide_react && bun run dev

# Run the agent binary in the foreground.
run-agent:
    cargo run --bin agent

# ── Full local stack (docker compose: panel + agent) ─────────────────────────

# Start the full stack (panel :4000, agent :50051 internal).
up:
    docker network create openoxide-network 2>/dev/null || true
    docker compose -f docker-compose.dev.yml up -d --build

# Stop the full stack.
down:
    docker compose -f docker-compose.dev.yml down

# Tail all services' logs.
logs:
    docker compose -f docker-compose.dev.yml logs -f

# Restart the dev stack (:4000). The panel image builds the SPA and the
# binary from source inside the Docker build, so host builds are unnecessary.
restart:
    docker compose -f docker-compose.dev.yml up -d --build --force-recreate

# Native (non-docker) restart: fresh bundle + fresh binary, foreground.
restart-native: web-build
    cargo run --bin openoxide

# ── Docker (production/standalone) ──────────────────────────────────────────

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

# Aliases for the production stacks (unchanged behavior)
up-prod: compose-up
down-prod: compose-down

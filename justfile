# ── Local dev ────────────────────────────────────────────────────────────────

# CPU cap for image builds, in µs per 100 ms period (800000 = 8 cores). The
# daemon sees every core, and an uncapped cargo build starves the desktop.
# Pair with CARGO_BUILD_JOBS in the Dockerfiles — cpu.max throttles throughput
# but does not shrink nproc, so cargo would still fork 32 rustc processes.
BUILD_CPU_QUOTA := "800000"

run: web-build
    cargo run --bin openoxide


web-build:
    cd web/openoxide_react && bun run build
    touch src/api/dashboard.rs

# Fast web-only deploy: build the SPA on the host and stream the bundle
# (tar over stdin) into the openoxide_web_dist volume the panel serves
# from (see docker-compose.dev.yml). The panel reads assets from disk per
# request, so the new bundle is live on refresh — no Rust recompile, no
# image build, no container restart. (Scratch panel has no shell, hence
# the alpine helper rather than docker exec.)
web-deploy:
    cd web/openoxide_react && bun run build
    tar -C web/openoxide_react/dist -cf - . | docker run --rm -i -v openoxide_openoxide_web_dist:/data alpine sh -c 'rm -rf /data/* && tar -C /data -xf -'
    @echo "✓ web deployed — refresh https://localhost:4000"

# Watch the React source and re-run `just web-deploy` automatically: nodemon
# debounces changes (1s after the last save), runs the deploy, and waits for
# the next change. Only src/public/index.html are watched — dist and
# node_modules are excluded so build output never re-triggers itself.
web-deploy-watch:
    cd web/openoxide_react && ./node_modules/.bin/nodemon \
        --watch src --watch public --watch index.html \
        --ext ts,tsx,js,jsx,css,html,svg,json \
        --delay 1s \
        --exec "just web-deploy"

web-check:
    cd web/openoxide_react && bunx tsc --noEmit


# Build both dev images with the CPU capped. Compose's own `build` has no
# resource flags, so the images are built directly and `up` runs without
# --build (compose still matches them by tag).
build-images:
    docker buildx build --resource cpu-quota={{BUILD_CPU_QUOTA}} -f Dockerfile -t dubeyanand/openoxide:dev .
    docker buildx build --resource cpu-quota={{BUILD_CPU_QUOTA}} -f Dockerfile.agent -t dubeyanand/openoxide-agent:dev .


web:
    cd web/openoxide_react && bun run dev

run-agent:
    cargo run --bin agent


# Prepare local environment: Swarm, overlay network, and SSL certificates.
prepare-dev:
	@./scripts/prepare-dev.sh

# Start the full stack (panel :4000, agent :50051, Traefik edge :80/443).
up: prepare-dev build-images
	docker compose -f docker-compose.dev.yml up -d

down:
	docker compose -f docker-compose.dev.yml down

# Tail all services' logs.
logs:
	docker compose -f docker-compose.dev.yml logs -f

# Restart the dev stack (:4000). The panel image builds the SPA and the
# binary from source inside the Docker build, so host builds are unnecessary.
restart: prepare-dev build-images
	docker compose -f docker-compose.dev.yml up -d --force-recreate

# ── Code generation (FlashORM) ──────────────────────────────────────────────
# Type-safe query code is generated into src/flash_gen (flash.toml ->
# [gen.rust].out). NEVER hand-edit files there — they are overwritten on every
# `flash gen`. Requires the FlashORM `flash` CLI (https://github.com/Lumos-Labs-HQ/flash),
# installed via: `go install github.com/Lumos-Labs-HQ/flash@latest` (verified
# against v2.8.23); it is not vendored in this repo.
# Workflow after editing db/schema/*.sql or db/queries/*.sql:
#   1. just gen          (or `just gen-force` to ignore the incremental cache)
#   2. commit the regenerated src/flash_gen/*.rs alongside the .sql sources
# Generated code uses runtime sqlx::query (macros=false), so `cargo build`
# does NOT need a live DATABASE_URL.
#
# Known toolchain caveats with flash v2.8.23 (re-apply after a full regen):
#   * src/flash_gen/migrations.rs is emitted as `sqlx::query(statement)`, which
#     does NOT compile against this repo's sqlx (it needs `sqlx::AssertSqlSafe`).
#     Keep the committed version: `git checkout src/flash_gen/migrations.rs`.
#   * db/queries/traffic_stats.sql uses `LIKE ? ESCAPE '\'`, which flash cannot
#     parse (and whose generated code needs an un-added `rust_decimal` dep). That
#     feature is unwired WIP; move the file aside before a full `flash gen`.

# Regenerate query code from db/schema + db/queries into src/flash_gen.
gen:
	flash gen
	git checkout -- src/flash_gen/migrations.rs

# Force a full regeneration, ignoring flash's incremental checksum cache.
gen-force:
	flash gen -f

# Fail if src/flash_gen is out of sync with the committed .sql sources.
gen-check: gen-force
	git diff --exit-code src/flash_gen

# Create a migration capturing pending schema changes, then apply it.
flash-migrate msg:
	flash migrate "{{msg}}"

flash-apply:
	flash apply

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
compose-up: prepare-dev
	docker compose up -d --build

# Stop the main OpenOxide compose stack
compose-down:
    docker compose down

# Tail the main OpenOxide compose logs
compose-logs:
    docker compose logs -f

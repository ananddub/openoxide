# syntax=docker/dockerfile:1

# Bun version for the SPA toolchain — global scope so `FROM` below can
# expand it (COPY --from=<image:tag> can't expand stage-scoped ARGs).
ARG BUN_VERSION=1.3.14

# Official bun toolchain image (Docker Hub) — pulled once, binary COPY'd
# into the builder (see the SPA build below).
FROM oven/bun:${BUN_VERSION}-debian AS bun

FROM rust:slim-bookworm AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang build-essential musl-tools libclang-dev pkg-config libsqlite3-dev sqlite3 ca-certificates protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# Pinned nightly (1.100.0-nightly) matching rust-toolchain.toml
RUN rustup toolchain install nightly-2026-08-30 --profile minimal \
        --target x86_64-unknown-linux-musl && \
    rustup default nightly-2026-08-30

WORKDIR /usr/src/openoxide

# Build profile: `release-fast` (no LTO, many codegen units) for quick
# iteration; `--build-arg BUILD_PROFILE=release` is the normal production
# build (thin LTO); `BUILD_PROFILE=release-size` gives the smallest binary
# (fat LTO, one codegen unit). The cargo `--profile` name and its target/
# output dir differ, so both are derived here.
ARG BUILD_PROFILE=release-fast

# ── 1. Dependency manifest pre-cache ──────────────────────────────────────────
# Real sources are copied before the pre-build on purpose: cargo's artifacts
# for unchanged workspace crates are then reused by the final build. A
# manifest-only layer with dummy roots was measured and is slower (203 s vs
# 148 s per source edit) because every workspace crate must recompile twice.
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY agent/   agent/
COPY proto/   proto/
COPY db/      db/
COPY data/db.sqlite3 data/db.sqlite3

# Create a dummy main.rs to pre-compile all dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

ENV DATABASE_URL="sqlite:///usr/src/openoxide/data/db.sqlite3" \
    CARGO_BUILD_JOBS=8 \
    CARGO_TARGET_DIR="/usr/src/openoxide/target"

# Pre-build dependencies layer (Cached by Docker unless Cargo.toml/Cargo.lock changes!)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/openoxide/target \
    cargo build --profile ${BUILD_PROFILE} --target x86_64-unknown-linux-musl -p openoxide || true

# ── 2. Dashboard SPA 
# Layer order matters: Docker invalidates every layer after a changed one, so
# the SPA builds before `COPY src/` — a Rust-only edit reuses these layers.─────────────────────────────────────────────────────
# Build the dashboard SPA into web/openoxide_react/dist so rust-embed
# bakes it into the panel binary (single-binary serving, prototype
# pattern). lefthook's prepare step needs git hooks tooling that doesn't
# exist in this image, hence --ignore-scripts.
# Bun comes from the official oven/bun image (Docker Hub) instead of a
# GitHub release ADD — release-assets.githubusercontent.com times out on
# some networks/CDNs, while registry-1.docker.io pulls reliably. The
# -debian variant matches this stage's bookworm glibc, so the binary
# drops in without extra deps.
COPY --from=bun /usr/local/bin/bun /usr/local/bin/bun
RUN bun --version
# Install deps first (only manifests — layer caches unless lockfile changes)
COPY web/openoxide_react/package.json web/openoxide_react/bun.lock* web/openoxide_react/
RUN --mount=type=cache,target=/root/.bun/install/cache \
    cd web/openoxide_react && bun install --frozen-lockfile --ignore-scripts

# Full SPA source (index.html, src/, vite config) — after deps so the
# dependency layer stays cached. node_modules/dist are dockerignored.
COPY web/openoxide_react/ web/openoxide_react/
RUN cd web/openoxide_react && bun run build

# ── 3. Actual Source Build ─
COPY src/ src/

# Rebuild only what changed. The SQLx migration macro reads SQL at compile
# time, so touch that module (and the entry points) to force just those to
# recompile; everything else reuses the incremental cache in the target mount.
# NOTE: no `cargo clean` here — a full clean threw away the incremental cache
# and forced a multi-minute from-scratch rebuild on every image build.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/openoxide/target \
    touch src/main.rs crates/db/src/db/connection/sqlite.rs crates/services/api/src/api/dashboard.rs && \
    cargo build --profile ${BUILD_PROFILE} --target x86_64-unknown-linux-musl -p openoxide && \
    _outdir=$([ "${BUILD_PROFILE}" = "release" ] && echo release || echo "${BUILD_PROFILE}") && \
    cp target/x86_64-unknown-linux-musl/${_outdir}/openoxide /usr/src/openoxide/openoxide-binary && \
    strip /usr/src/openoxide/openoxide-binary

# =============================================================================
# Stage 2: Minimal filesystem preparation
# =============================================================================
FROM alpine:3.21 AS runtime-files

ARG OPENOXIDE_BUILD_REV=backup-migration-0048

WORKDIR /app

RUN printf '%s' "$OPENOXIDE_BUILD_REV" > /tmp/openoxide-build-rev

RUN apk add --no-cache ca-certificates \
    && mkdir -p /app/data /tmp \
    && touch /app/data/.keep /tmp/.keep \
    && rm -rf /var/cache/apk/* /root/.cache

# Copy the compiled binary into the prepared filesystem.
COPY --from=backend-builder \
    /usr/src/openoxide/openoxide-binary \
    /usr/local/bin/openoxide

# =============================================================================
# Stage 3: Scratch runtime
# =============================================================================
FROM scratch

COPY --from=runtime-files /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=runtime-files /app /app
COPY --from=runtime-files /tmp /tmp
COPY --from=runtime-files /usr/local/bin/openoxide /usr/local/bin/openoxide

WORKDIR /app

ENV PORT=4000 \
    HOST=0.0.0.0 \
    DATABASE_URL="sqlite:///app/data/db.sqlite3" \
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

EXPOSE 4000

ENTRYPOINT ["/usr/local/bin/openoxide"]

CMD ["openoxide"]

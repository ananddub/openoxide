# syntax=docker/dockerfile:1

# =============================================================================
# Stage 1: Rust static musl build
# =============================================================================
FROM rust:slim-bookworm AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang build-essential musl-tools libclang-dev pkg-config libsqlite3-dev sqlite3 ca-certificates protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# Pinned nightly (1.100.0-nightly) matching rust-toolchain.toml
RUN rustup toolchain install nightly-2026-08-30 --profile minimal \
        --target x86_64-unknown-linux-musl && \
    rustup default nightly-2026-08-30

WORKDIR /usr/src/openoxide

# ── 1. Dependency manifest pre-cache ──────────────────────────────────────────
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY agent/   agent/
COPY proto/   proto/
COPY build.rs build.rs
COPY db/      db/
COPY data/db.sqlite3 data/db.sqlite3

# Create dummy main.rs and lib.rs to pre-compile all dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs

ENV DATABASE_URL="sqlite:///usr/src/openoxide/data/db.sqlite3" \
    CARGO_BUILD_JOBS=8 \
    CARGO_TARGET_DIR="/usr/src/openoxide/target"

# Pre-build dependencies layer (Cached by Docker unless Cargo.toml/Cargo.lock changes!)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/openoxide/target \
    cargo build --release --target x86_64-unknown-linux-musl -p openoxide || true

# ── 2. Actual Source Build ─────────────────────────────────────────────────────
COPY src/ src/

# Build the dashboard SPA into web/openoxide_react/dist so rust-embed
# bakes it into the panel binary (single-binary serving, prototype
# pattern). lefthook's prepare step needs git hooks tooling that doesn't
# exist in this image, hence --ignore-scripts.
ARG BUN_VERSION=1.3.14
ADD https://github.com/oven-sh/bun/releases/download/bun-v${BUN_VERSION}/bun-linux-x64.zip /tmp/bun.zip
RUN apt-get update -qq && apt-get install -y -qq unzip >/dev/null && \
    cd /tmp && unzip -q bun.zip && \
    mv bun-linux-x64/bun /usr/local/bin/bun && chmod +x /usr/local/bin/bun && bun --version
# Install deps first (only manifests — layer caches unless lockfile changes)
COPY web/openoxide_react/package.json web/openoxide_react/bun.lock* web/openoxide_react/
RUN --mount=type=cache,target=/root/.bun/install/cache \
    cd web/openoxide_react && bun install --ignore-scripts

# Full SPA source (index.html, src/, vite config) — after deps so the
# dependency layer stays cached. node_modules/dist are dockerignored.
COPY web/openoxide_react/ web/openoxide_react/
RUN cd web/openoxide_react && bun run build

# Invalidate the dummy binary and the SQLx migration embed point. The migration
# macro reads SQL files at compile time, so recompile this module whenever the
# copied db/migrations directory changes instead of reusing a stale Cargo cache.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/openoxide/target \
    cargo clean -p openoxide --target x86_64-unknown-linux-musl && \
    touch src/main.rs src/lib.rs src/db/connection/sqlite.rs && \
    cargo build --release --target x86_64-unknown-linux-musl -p openoxide && \
    cp target/x86_64-unknown-linux-musl/release/openoxide /usr/src/openoxide/openoxide-binary && \
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

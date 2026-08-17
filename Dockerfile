# syntax=docker/dockerfile:1

# =============================================================================
# Stage 1: Rust static musl build
# =============================================================================
FROM rust:slim-bookworm AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang build-essential musl-tools libclang-dev pkg-config libsqlite3-dev sqlite3 ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/openoxide

# ── dependency pre-cache ──────────────────────────────────────────────────────
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY agent/   agent/
COPY proto/   proto/
COPY build.rs build.rs
COPY db/      db/
COPY src/     src/
COPY data/db.sqlite3 data/db.sqlite3

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

RUN --mount=type=cache,target=/usr/src/openoxide/target \
    rm -rf /usr/src/openoxide/target/x86_64-unknown-linux-musl/release/deps/*html_rt* /usr/src/openoxide/target/release/deps/*html_rt*

ENV DATABASE_URL="sqlite:///usr/src/openoxide/data/db.sqlite3"
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/openoxide/target \
    cargo build --release --target x86_64-unknown-linux-musl -p openoxide && \
    cp target/x86_64-unknown-linux-musl/release/openoxide /usr/src/openoxide/openoxide-binary && \
    strip /usr/src/openoxide/openoxide-binary


# =============================================================================
# Stage 2: Alpine runtime — panel binary + all required CLI tools
# =============================================================================
FROM alpine:3.21

ARG NIXPACKS_VERSION=1.41.0

# ── System packages + rclone + nixpacks (Single Layer Optimization) ──────────
RUN apk add --no-cache \
    ca-certificates \
    docker-cli \
    docker-cli-compose \
    git \
    wireguard-tools \
    iproute2 \
    openssh-client \
    curl \
    tar \
    zip \
    unzip \
    && curl -fsSL https://rclone.org/install.sh | bash \
    && curl -sSL https://nixpacks.com/install.sh -o /tmp/nixpacks-install.sh \
    && chmod +x /tmp/nixpacks-install.sh \
    && VERSION=${NIXPACKS_VERSION} /tmp/nixpacks-install.sh \
    && rm -f /tmp/nixpacks-install.sh \
    && strip /usr/local/bin/nixpacks 2>/dev/null || true \
    && strip /usr/local/bin/rclone 2>/dev/null || true

# ── OpenOxide panel binary ────────────────────────────────────────────────────
COPY --from=backend-builder \
    /usr/src/openoxide/openoxide-binary \
    /usr/local/bin/openoxide

WORKDIR /app
RUN mkdir -p /app/data

ENV PORT=4000 \
    HOST=0.0.0.0 \
    DATABASE_URL="sqlite:///app/data/db.sqlite3"

EXPOSE 4000

HEALTHCHECK --interval=30s --timeout=5s --start-period=30s --retries=3 \
    CMD curl -fs http://localhost:4000/ || exit 0

CMD ["openoxide"]

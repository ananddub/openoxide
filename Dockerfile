# Stage 1: Build Frontend
FROM node:22-alpine AS frontend-builder
WORKDIR /app/web/openoxide
RUN corepack enable && corepack prepare pnpm@latest --activate
COPY web/openoxide/package.json ./
RUN pnpm install
COPY web/openoxide/ ./
RUN pnpm build

# Stage 2: Build Backend Binary
FROM rust:latest AS backend-builder
WORKDIR /usr/src/openoxide

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY agent ./agent
COPY db ./db
COPY data ./data
COPY src ./src

ENV DATABASE_URL="sqlite:///usr/src/openoxide/data/db.sqlite3"

RUN cargo build --release -p openoxide

# Stage 3: Production Runtime Image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    sqlite3 \
    git \
    docker.io \
    dnsutils \
    wireguard-tools \
    iproute2 \
    iputils-ping \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Rust backend binary
COPY --from=backend-builder /usr/src/openoxide/target/release/openoxide /usr/local/bin/openoxide

# Copy Frontend static assets from SvelteKit output
COPY --from=frontend-builder /app/web/openoxide/.svelte-kit/output/client ./web/static

# Environment defaults
ENV PORT=3000
ENV HOST=0.0.0.0
ENV DATABASE_URL="sqlite:///app/data/db.sqlite3"

EXPOSE 3000

CMD ["openoxide"]

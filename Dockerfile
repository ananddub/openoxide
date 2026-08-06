# Stage 1: Build Frontend
FROM node:22-alpine AS frontend-builder
WORKDIR /app/web/rustploy
RUN corepack enable && corepack prepare pnpm@latest --activate
COPY web/rustploy/package.json ./
RUN pnpm install
COPY web/rustploy/ ./
RUN pnpm build

# Stage 2: Build Backend Binary
FROM rust:latest AS backend-builder
WORKDIR /usr/src/rustploy

COPY Cargo.toml Cargo.lock ./
COPY auto_route ./auto_route
COPY auto_route_macros ./auto_route_macros
COPY auto_socket ./auto_socket
COPY auto_socket_macros ./auto_socket_macros
COPY rustploy_sh_macros ./rustploy_sh_macros
COPY rustploy_monitor ./rustploy_monitor
COPY db ./db
COPY data ./data
COPY src ./src

ENV DATABASE_URL="sqlite:///usr/src/rustploy/data/db.sqlite3"

RUN cargo build --release -p rustploy

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
COPY --from=backend-builder /usr/src/rustploy/target/release/rustploy /usr/local/bin/rustploy

# Copy Frontend static assets from SvelteKit output
COPY --from=frontend-builder /app/web/rustploy/.svelte-kit/output/client ./web/static

# Environment defaults
ENV PORT=3000
ENV HOST=0.0.0.0
ENV DATABASE_URL="sqlite:///app/data/db.sqlite3"

EXPOSE 3000

CMD ["rustploy"]

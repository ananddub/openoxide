# =============================================================================
# Dev Web Container for OpenOxide Dashboard (self-building SPA)
# =============================================================================
# Builds the React SPA with bun + vite inside the image, then serves it with
# Caddy (SPA + ConnectRPC proxy to the panel). Used by docker-compose.dev.yml
# so a single `docker compose up --build` brings up panel + agent + web
# without any pre-built dist/ on the host.

# ── Stage 1: build the SPA ───────────────────────────────────────────────────
FROM oven/bun:1-alpine AS build
WORKDIR /app

# lefthook's prepare script needs git hooks tooling that doesn't belong in
# an image; skip lifecycle scripts, then install deps normally.
COPY web/openoxide_react/package.json web/openoxide_react/bun.lock* ./
RUN bun install --ignore-scripts

COPY web/openoxide_react/ ./
RUN bun run build

# ── Stage 2: serve with Caddy ────────────────────────────────────────────────
FROM caddy:2-alpine

COPY --from=build /app/dist /srv
COPY web/openoxide_react/Caddyfile /etc/caddy/Caddyfile

EXPOSE 80

CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile", "--adapter", "caddyfile"]

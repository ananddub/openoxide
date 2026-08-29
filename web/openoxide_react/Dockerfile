# Stage 1: Build the React SPA
FROM oven/bun:alpine AS builder

WORKDIR /app

# Copy dependency files
COPY package.json bun.lock* ./

# Install dependencies
RUN bun install --frozen-lockfile || bun install

# Copy source files
COPY . .

# Build production bundle
RUN bun run build

# Stage 2: Serve with lightweight Nginx
FROM nginx:alpine

# Remove default nginx configs
RUN rm -rf /etc/nginx/conf.d/* /usr/share/nginx/html/*

# Copy built assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy custom nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

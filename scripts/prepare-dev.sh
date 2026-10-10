#!/usr/bin/env bash
set -euo pipefail

echo "==> [1/4] Ensuring Docker Swarm is active..."
SWARM_STATE="$(docker info --format '{{.Swarm.LocalNodeState}}')"
if [ "$SWARM_STATE" = "active" ]; then
    echo "Docker Swarm already active — skipping init"
else
    echo "Docker Swarm state is '$SWARM_STATE' — initializing..."
    docker swarm init
fi

echo "==> [2/4] Ensuring openoxide-network is an attachable overlay network..."
if docker network inspect openoxide-network 2>/dev/null | grep -q '"Scope": "local"'; then
    echo "openoxide-network is a local bridge — converting to swarm overlay..."
    docker network rm openoxide-network
fi
if ! docker network inspect openoxide-network >/dev/null 2>&1; then
    echo "Creating openoxide-network overlay network..."
    docker network create --driver overlay --attachable openoxide-network
else
    echo "openoxide-network already exists"
fi

echo "==> [3/4] Ensuring dev TLS certificates exist..."
TARGET_DIR="$HOME/.openoxide/certs"
mkdir -p "$TARGET_DIR"

if [ ! -f "$TARGET_DIR/cert.pem" ] || [ ! -f "$TARGET_DIR/key.pem" ]; then
    if [ -f "./certs/cert.pem" ] && [ -f "./certs/key.pem" ]; then
        echo "Staging certs from ./certs to $TARGET_DIR..."
        cp ./certs/cert.pem "$TARGET_DIR/cert.pem"
        cp ./certs/key.pem "$TARGET_DIR/key.pem"
    elif command -v mkcert >/dev/null 2>&1; then
        echo "Generating local TLS certificates with mkcert..."
        if ! mkcert -install; then
            echo "WARNING: mkcert -install failed (CA may not be trusted — browsers will show a warning)" >&2
        fi
        mkcert -key-file "$TARGET_DIR/key.pem" -cert-file "$TARGET_DIR/cert.pem" localhost 127.0.0.1 ::1
    else
        echo "Generating self-signed TLS certificates with openssl..."
        openssl req -x509 -newkey rsa:4096 -keyout "$TARGET_DIR/key.pem" -out "$TARGET_DIR/cert.pem" -days 365 -nodes -subj "/CN=localhost"
    fi
else
    echo "Certificates already present at $TARGET_DIR — skipping generation"
fi

echo "==> [4/4] Verifying cert files in Docker Desktop VM mount..."
if docker run --rm -v "$TARGET_DIR:/certs" alpine test -d /certs/cert.pem 2>/dev/null; then
    echo "Docker Desktop cached a stale directory in the certs mount — re-staging file contents..."
    docker run --rm -v "$TARGET_DIR:/certs" alpine rm -rf /certs/cert.pem /certs/key.pem
    docker run --rm -i -v "$TARGET_DIR:/certs" alpine sh -c "cat > /certs/cert.pem" < "$TARGET_DIR/cert.pem"
    docker run --rm -i -v "$TARGET_DIR:/certs" alpine sh -c "cat > /certs/key.pem" < "$TARGET_DIR/key.pem"
    echo "Cert files re-staged into the VM mount"
else
    echo "Cert files mount correctly into the Docker VM"
fi

echo "==> Preflight checks complete! Environment is ready."

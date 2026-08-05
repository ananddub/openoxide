CREATE TABLE server_private_networks (
    server_id INTEGER PRIMARY KEY NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    connection_mode TEXT NOT NULL DEFAULT 'DIRECT_SSH'
        CHECK (connection_mode IN ('DIRECT_SSH', 'MANAGED_WIREGUARD', 'EXTERNAL_PRIVATE_NETWORK')),
    provider TEXT
        CHECK (provider IS NULL OR provider IN ('WIREGUARD', 'TAILSCALE', 'ZEROTIER', 'NETBIRD', 'CUSTOM')),
    private_host TEXT,
    tunnel_address TEXT,
    public_key TEXT,
    endpoint TEXT,
    listen_port INTEGER,
    persistent_keepalive INTEGER,
    status TEXT NOT NULL DEFAULT 'DISABLED'
        CHECK (status IN ('DISABLED', 'CONFIGURING', 'ACTIVE', 'FAILED')),
    last_handshake_at INTEGER,
    config_version INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (connection_mode = 'DIRECT_SSH' OR private_host IS NOT NULL),
    CHECK (listen_port IS NULL OR listen_port BETWEEN 1 AND 65535),
    CHECK (persistent_keepalive IS NULL OR persistent_keepalive BETWEEN 0 AND 65535)
);

CREATE UNIQUE INDEX idx_server_private_network_tunnel_address
    ON server_private_networks(tunnel_address)
    WHERE tunnel_address IS NOT NULL;

-- Remote servers managed by dokpanel via SSH
CREATE TABLE servers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	description TEXT,
	ip_address TEXT NOT NULL,
	port INTEGER NOT NULL DEFAULT 22,
	username TEXT NOT NULL DEFAULT 'root',
	app_name TEXT NOT NULL UNIQUE,
	-- server_status: active | inactive
	server_status TEXT NOT NULL DEFAULT 'ACTIVE',
	-- server_type: deploy | build
	server_type TEXT NOT NULL DEFAULT 'DEPLOY',
	enable_docker_cleanup INTEGER NOT NULL DEFAULT 0,
	log_cleanup_cron TEXT DEFAULT '0 0 * * *',
	command TEXT NOT NULL DEFAULT '',
	-- JSON: metrics config object { server: {...}, containers: {...} }
	metrics_config TEXT NOT NULL DEFAULT '{}',
	ssh_key_id INTEGER REFERENCES ssh_keys(id) ON DELETE SET NULL,
	build_memory_limit TEXT,
	build_cpu_limit TEXT,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT server_status_check CHECK (server_status IN ('ACTIVE', 'INACTIVE')),
	CONSTRAINT server_type_check CHECK (server_type IN ('DEPLOY', 'BUILD'))
) STRICT;

CREATE TABLE server_management (
    server_id INTEGER PRIMARY KEY REFERENCES servers(id) ON DELETE CASCADE,
    maintenance_mode INTEGER NOT NULL DEFAULT 0 CHECK (maintenance_mode IN (0, 1)),
    maintenance_message TEXT,
    labels TEXT NOT NULL DEFAULT '{}',
    cleanup_policy TEXT NOT NULL DEFAULT '{"containers":true,"images":true,"networks":true,"volumes":false,"packages":false}',
    gpu_enabled INTEGER NOT NULL DEFAULT 0 CHECK (gpu_enabled IN (0, 1)),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE server_cleanup_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id INTEGER NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'RUNNING' CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED')),
    policy TEXT NOT NULL,
    stdout TEXT,
    stderr TEXT,
    started_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    finished_at INTEGER
);

CREATE INDEX idx_server_cleanup_executions_server ON server_cleanup_executions(server_id, started_at DESC);

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
    dns_name TEXT,
    routes TEXT NOT NULL DEFAULT '[]',
    health_status TEXT NOT NULL DEFAULT 'UNKNOWN'
        CHECK (health_status IN ('UNKNOWN', 'HEALTHY', 'DEGRADED', 'UNREACHABLE', 'CONFIG_DRIFT')),
    health_error TEXT,
    last_health_check_at INTEGER,
    consecutive_failures INTEGER NOT NULL DEFAULT 0,
    operation TEXT,
    operation_lease_until INTEGER,
    config_hash TEXT,
    rotation_state TEXT NOT NULL DEFAULT 'IDLE'
        CHECK (rotation_state IN ('IDLE', 'ROTATING', 'ROLLING_BACK', 'FAILED')),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (connection_mode = 'DIRECT_SSH' OR private_host IS NOT NULL),
    CHECK (listen_port IS NULL OR listen_port BETWEEN 1 AND 65535),
    CHECK (persistent_keepalive IS NULL OR persistent_keepalive BETWEEN 0 AND 65535)
);

CREATE UNIQUE INDEX idx_server_private_network_tunnel_address
    ON server_private_networks(tunnel_address)
    WHERE tunnel_address IS NOT NULL;

CREATE UNIQUE INDEX idx_server_private_network_dns_name
    ON server_private_networks(dns_name)
    WHERE dns_name IS NOT NULL;

-- Trigger Function
CREATE TRIGGER servers_updated_at
AFTER UPDATE ON servers
FOR EACH ROW
BEGIN
	UPDATE servers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
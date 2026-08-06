ALTER TABLE server_private_networks ADD COLUMN dns_name TEXT;
ALTER TABLE server_private_networks ADD COLUMN routes TEXT NOT NULL DEFAULT '[]';
ALTER TABLE server_private_networks ADD COLUMN health_status TEXT NOT NULL DEFAULT 'UNKNOWN'
    CHECK (health_status IN ('UNKNOWN', 'HEALTHY', 'DEGRADED', 'UNREACHABLE', 'CONFIG_DRIFT'));
ALTER TABLE server_private_networks ADD COLUMN health_error TEXT;
ALTER TABLE server_private_networks ADD COLUMN last_health_check_at INTEGER;
ALTER TABLE server_private_networks ADD COLUMN consecutive_failures INTEGER NOT NULL DEFAULT 0;
ALTER TABLE server_private_networks ADD COLUMN operation TEXT;
ALTER TABLE server_private_networks ADD COLUMN operation_lease_until INTEGER;
ALTER TABLE server_private_networks ADD COLUMN config_hash TEXT;
ALTER TABLE server_private_networks ADD COLUMN rotation_state TEXT NOT NULL DEFAULT 'IDLE'
    CHECK (rotation_state IN ('IDLE', 'ROTATING', 'ROLLING_BACK', 'FAILED'));

CREATE UNIQUE INDEX idx_server_private_network_dns_name
    ON server_private_networks(dns_name)
    WHERE dns_name IS NOT NULL;

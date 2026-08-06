DROP INDEX idx_server_private_network_dns_name;

CREATE UNIQUE INDEX idx_server_private_network_dns_name
    ON server_private_networks(dns_name COLLATE NOCASE)
    WHERE dns_name IS NOT NULL;

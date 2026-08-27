CREATE TABLE settings (
    id INTEGER NOT NULL PRIMARY KEY,
    server_ip TEXT,
    certificate_type TEXT NOT NULL,
    custom_cert_resolver TEXT,
    https INTEGER NOT NULL,
    host TEXT,
    lets_encrypt_email TEXT,
    enable_docker_cleanup INTEGER NOT NULL,
    log_cleanup_cron TEXT,
    metrics_config TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

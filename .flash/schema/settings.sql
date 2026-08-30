CREATE TABLE settings (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    server_ip TEXT,
    certificate_type TEXT NOT NULL,
    custom_cert_resolver TEXT,
    https BIGINT NOT NULL,
    host TEXT,
    lets_encrypt_email TEXT,
    enable_docker_cleanup BIGINT NOT NULL,
    log_cleanup_cron TEXT,
    metrics_config TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

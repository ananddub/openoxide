CREATE TABLE settings (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	server_ip TEXT,
	-- certificate_type: NONE | LETSENCRYPT | CUSTOM
	certificate_type TEXT NOT NULL DEFAULT 'NONE',
	custom_cert_resolver TEXT,
	https INTEGER NOT NULL DEFAULT 0,
	host TEXT, -- Domain Name for server
	lets_encrypt_email TEXT,
	enable_docker_cleanup INTEGER NOT NULL DEFAULT 1,
	log_cleanup_cron TEXT DEFAULT '0 0 * * *',
	-- JSON: metrics config object { server: {...}, containers: {...} }
	metrics_config TEXT NOT NULL DEFAULT '',
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT settings_certificate_check CHECK (certificate_type IN ('NONE', 'LETSENCRYPT', 'CUSTOM'))
) STRICT;
CREATE TRIGGER settings_updated_at
AFTER UPDATE ON settings
FOR EACH ROW
BEGIN
	UPDATE settings
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

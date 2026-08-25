-- redirects (Traefik redirection rules)
CREATE TABLE redirects (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	regex TEXT NOT NULL,
	replacement TEXT NOT NULL,
	permanent INTEGER NOT NULL DEFAULT 0,
	unique_config_key INTEGER,
	application_id INTEGER NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- ports (Docker exposed ports)
CREATE TABLE ports (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	published_port INTEGER NOT NULL,
	target_port INTEGER NOT NULL,
	-- protocol: TCP | UDP
	protocol TEXT NOT NULL DEFAULT 'TCP',
	-- publish_mode: INGRESS | HOST
	publish_mode TEXT NOT NULL DEFAULT 'HOST',
	application_id INTEGER NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT port_protocol_check CHECK (protocol IN ('TCP', 'UDP')),
	CONSTRAINT port_publish_mode_check CHECK (publish_mode IN ('INGRESS', 'HOST'))
) STRICT;

-- security (Basic Auth logins)
CREATE TABLE security (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	username TEXT NOT NULL,
	password TEXT NOT NULL,
	application_id INTEGER NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	UNIQUE(username, application_id)
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_redirects_application_id ON redirects(application_id);
CREATE INDEX idx_ports_application_id ON ports(application_id);
CREATE INDEX idx_security_application_id ON security(application_id);

-- Trigger for redirects updated_at
CREATE TRIGGER redirects_updated_at
AFTER UPDATE ON redirects
FOR EACH ROW
BEGIN
	UPDATE redirects
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
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
CREATE INDEX idx_ports_application_id ON ports(application_id);

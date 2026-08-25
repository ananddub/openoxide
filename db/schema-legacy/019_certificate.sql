CREATE TABLE certificates (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	certificate_data TEXT NOT NULL,
	private_key TEXT NOT NULL,
	certificate_path TEXT NOT NULL UNIQUE,
	auto_renew INTEGER NOT NULL DEFAULT 0,
	-- Foreign keys
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	-- Timestamp
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT auto_renew_check CHECK (auto_renew IN (0, 1))
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_certificates_server_id ON certificates(server_id);
CREATE INDEX idx_certificates_organization_id ON certificates(organization_id);

CREATE TABLE certificate_renewals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    certificate_id INTEGER NOT NULL REFERENCES certificates(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED')),
    previous_expires_at INTEGER,
    new_expires_at INTEGER,
    error TEXT,
    started_at INTEGER NOT NULL DEFAULT (unixepoch()),
    finished_at INTEGER
) STRICT;

CREATE INDEX idx_certificate_renewals_certificate_started ON certificate_renewals(certificate_id, started_at DESC);

-- Trigger Function
CREATE TRIGGER certificates_updated_at
AFTER UPDATE ON certificates
FOR EACH ROW
BEGIN
	UPDATE certificates
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
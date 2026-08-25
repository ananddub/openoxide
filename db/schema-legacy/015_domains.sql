CREATE TABLE domains (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	host TEXT NOT NULL,
	https INTEGER NOT NULL DEFAULT 0,
	port INTEGER DEFAULT 3000,
	path TEXT DEFAULT '/',
	internal_path TEXT DEFAULT '/',
	custom_entrypoint TEXT,
	service_name TEXT,
	custom_cert_resolver TEXT,
	strip_path INTEGER NOT NULL DEFAULT 0,
	-- JSON array of middleware names e.g. '[redirect-to-https]'
	middlewares TEXT NOT NULL DEFAULT '[]',
	-- domain_type: application | compose | preview
	domain_type TEXT NOT NULL DEFAULT 'APPLICATION',
	-- certificate_type: letsencrypt | none | custom
	certificate_type TEXT NOT NULL DEFAULT 'NONE',
	-- One of these will be set
	application_id INTEGER REFERENCES applications(id) ON DELETE CASCADE,
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT domain_cert_type_check CHECK (certificate_type IN ('LETSENCRYPT', 'NONE', 'CUSTOM')),
	CONSTRAINT domain_type_check CHECK (domain_type IN ('APPLICATION', 'COMPOSE', 'PREVIEW'))
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_domains_application_id ON domains(application_id);
CREATE INDEX idx_domains_compose_id ON domains(compose_id);
CREATE INDEX idx_domains_host ON domains(host);

-- Trigger Function
CREATE TRIGGER domains_updated_at
AFTER UPDATE ON domains
FOR EACH ROW
BEGIN
	UPDATE domains
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
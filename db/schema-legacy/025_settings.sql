-- Web Server Settings Table
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

-- Ai Settings Table
CREATE TABLE ai_settings (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	api_url TEXT NOT NULL,
	api_key TEXT NOT NULL,
	model TEXT NOT NULL,
	is_enabled INTEGER NOT NULL DEFAULT 1,
	-- Foreign keys
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT ai_enabled_check CHECK (is_enabled IN (0, 1))
) STRICT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_ai_settings_org_name ON ai_settings(organization_id, name COLLATE NOCASE);

CREATE TABLE ai_generations (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	ai_setting_id INTEGER NOT NULL REFERENCES ai_settings(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	prompt TEXT NOT NULL,
	output_json TEXT NOT NULL,
	status TEXT NOT NULL DEFAULT 'DRAFT',
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE SET NULL,
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	CONSTRAINT ai_generation_status_check
		CHECK (status IN ('DRAFT', 'REVIEWED', 'DEPLOYING', 'DEPLOYED'))
) STRICT;

CREATE INDEX idx_ai_generations_organization ON ai_generations(organization_id, created_at DESC);
CREATE INDEX idx_ai_generations_setting ON ai_generations(ai_setting_id, created_at DESC);

-- Indexes for faster queries
CREATE INDEX IF NOT EXISTS idx_ai_settings_organization_id ON ai_settings(organization_id);

-- Trigger Function
CREATE TRIGGER settings_updated_at
AFTER UPDATE ON settings
FOR EACH ROW
BEGIN
	UPDATE settings
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER ai_settings_updated_at
AFTER UPDATE ON ai_settings
FOR EACH ROW
BEGIN
	UPDATE ai_settings
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER ai_generations_updated_at
AFTER UPDATE ON ai_generations
FOR EACH ROW
BEGIN
	UPDATE ai_generations
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
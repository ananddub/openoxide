CREATE UNIQUE INDEX IF NOT EXISTS idx_ai_settings_org_name
ON ai_settings(organization_id, name COLLATE NOCASE);

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

CREATE INDEX idx_ai_generations_organization
ON ai_generations(organization_id, created_at DESC);

CREATE INDEX idx_ai_generations_setting
ON ai_generations(ai_setting_id, created_at DESC);

CREATE TRIGGER ai_generations_updated_at
AFTER UPDATE ON ai_generations
FOR EACH ROW
BEGIN
	UPDATE ai_generations
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

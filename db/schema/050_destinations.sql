CREATE TABLE destinations (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- provider: S3 | R2 | BACKBLAZE | GCS | DO_SPACES
	provider TEXT NOT NULL DEFAULT 'S3',
	access_key TEXT NOT NULL,
	secret_access_key TEXT NOT NULL,
	bucket TEXT NOT NULL,
	region TEXT NOT NULL,
	endpoint TEXT NOT NULL,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	-- Timestamp
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT destination_provider_check CHECK (provider IN ('S3', 'R2', 'BACKBLAZE', 'GCS', 'DO_SPACES'))
) STRICT;

CREATE INDEX idx_destinations_organization_id ON destinations(organization_id);

CREATE TRIGGER destinations_updated_at
AFTER UPDATE ON destinations
FOR EACH ROW
BEGIN
	UPDATE destinations
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

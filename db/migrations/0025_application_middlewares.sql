CREATE TABLE application_middlewares (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    application_id INTEGER NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    middleware_type TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    config TEXT NOT NULL DEFAULT '{}',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CONSTRAINT application_middleware_type_check CHECK (
        middleware_type IN ('COMPRESS', 'HEADERS', 'RATE_LIMIT', 'IP_ALLOWLIST')
    ),
    UNIQUE(application_id, name)
) STRICT;

CREATE INDEX idx_application_middlewares_application_id
    ON application_middlewares(application_id);

CREATE TRIGGER application_middlewares_updated_at
AFTER UPDATE ON application_middlewares
FOR EACH ROW
BEGIN
    UPDATE application_middlewares
    SET updated_at = strftime('%s', 'now')
    WHERE id = OLD.id;
END;

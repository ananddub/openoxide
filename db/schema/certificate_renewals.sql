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

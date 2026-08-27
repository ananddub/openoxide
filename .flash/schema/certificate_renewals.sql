CREATE TABLE certificate_renewals (
    id INTEGER NOT NULL PRIMARY KEY,
    certificate_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    previous_expires_at INTEGER,
    new_expires_at INTEGER,
    error TEXT,
    started_at INTEGER NOT NULL,
    finished_at INTEGER
);

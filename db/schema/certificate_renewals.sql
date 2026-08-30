CREATE TABLE certificate_renewals (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    certificate_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    status TEXT NOT NULL,
    previous_expires_at BIGINT,
    new_expires_at BIGINT,
    error TEXT,
    started_at BIGINT NOT NULL,
    finished_at BIGINT
);

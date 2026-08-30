CREATE TABLE audit_logs (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_email TEXT NOT NULL,
    user_role TEXT NOT NULL,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    resource_name TEXT,
    metadata TEXT,
    organization_id BIGINT,
    user_id BIGINT,
    created_at BIGINT NOT NULL
);

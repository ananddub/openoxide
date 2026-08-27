CREATE TABLE audit_logs (
    id INTEGER NOT NULL PRIMARY KEY,
    user_email TEXT NOT NULL,
    user_role TEXT NOT NULL,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    resource_name TEXT,
    metadata TEXT,
    organization_id INTEGER,
    user_id INTEGER,
    created_at INTEGER NOT NULL
);

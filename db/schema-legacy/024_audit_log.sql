CREATE TABLE audit_logs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	user_email TEXT NOT NULL,
	user_role TEXT NOT NULL,
	action TEXT NOT NULL,
	resource_type TEXT NOT NULL,
	resource_id TEXT,
	resource_name TEXT,
	metadata TEXT, -- Extra info / JSON string
	-- Foreign keys
	organization_id INTEGER REFERENCES organization(id) ON DELETE SET NULL,
	user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_organization_id ON audit_logs(organization_id);
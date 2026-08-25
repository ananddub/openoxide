CREATE TABLE group_policy (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	group_id INTEGER NOT NULL REFERENCES groups(id),
	policy_id INTEGER NOT NULL REFERENCES policy(id),
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;
CREATE UNIQUE INDEX idx_group_policy_unique
ON group_policy(group_id, policy_id);

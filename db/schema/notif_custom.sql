CREATE TABLE notif_custom (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	endpoint TEXT NOT NULL,
	headers TEXT -- JSON object (headers Record<string, string>)
) STRICT;

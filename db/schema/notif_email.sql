CREATE TABLE notif_email (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	smtp_server TEXT NOT NULL,
	smtp_port INTEGER NOT NULL,
	username TEXT NOT NULL,
	password TEXT NOT NULL,
	from_address TEXT NOT NULL,
	to_addresses TEXT NOT NULL DEFAULT '[]' -- JSON array of strings
) STRICT;

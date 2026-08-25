CREATE TABLE notif_resend (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	api_key TEXT NOT NULL,
	from_address TEXT NOT NULL,
	to_addresses TEXT NOT NULL DEFAULT '[]' -- JSON array of strings
) STRICT;

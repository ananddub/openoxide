CREATE TABLE notif_mattermost (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	webhook_url TEXT NOT NULL,
	channel TEXT,
	username TEXT
) STRICT;

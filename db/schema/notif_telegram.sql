CREATE TABLE notif_telegram (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	bot_token TEXT NOT NULL,
	chat_id TEXT NOT NULL,
	message_thread_id TEXT
) STRICT;

CREATE TABLE notif_telegram (
    id INTEGER NOT NULL PRIMARY KEY,
    bot_token TEXT NOT NULL,
    chat_id TEXT NOT NULL,
    message_thread_id TEXT
);

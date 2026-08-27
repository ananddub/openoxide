CREATE TABLE notif_pushover (
    id INTEGER NOT NULL PRIMARY KEY,
    user_key TEXT NOT NULL,
    api_token TEXT NOT NULL,
    priority INTEGER NOT NULL,
    retry INTEGER,
    expire INTEGER
);

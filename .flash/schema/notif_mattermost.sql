CREATE TABLE notif_mattermost (
    id INTEGER NOT NULL PRIMARY KEY,
    webhook_url TEXT NOT NULL,
    channel TEXT,
    username TEXT
);

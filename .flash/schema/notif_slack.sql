CREATE TABLE notif_slack (
    id INTEGER NOT NULL PRIMARY KEY,
    webhook_url TEXT NOT NULL,
    channel TEXT
);

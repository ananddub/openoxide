CREATE TABLE notif_ntfy (
    id INTEGER NOT NULL PRIMARY KEY,
    server_url TEXT NOT NULL,
    topic TEXT NOT NULL,
    access_token TEXT,
    priority INTEGER NOT NULL
);

CREATE TABLE notif_gotify (
    id INTEGER NOT NULL PRIMARY KEY,
    server_url TEXT NOT NULL,
    app_token TEXT NOT NULL,
    priority INTEGER NOT NULL,
    decoration INTEGER NOT NULL
);

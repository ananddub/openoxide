CREATE TABLE security (
    id INTEGER NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    application_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);

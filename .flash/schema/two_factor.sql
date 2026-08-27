CREATE TABLE two_factor (
    id INTEGER NOT NULL PRIMARY KEY,
    secret TEXT NOT NULL,
    backup_codes TEXT NOT NULL,
    user_id INTEGER NOT NULL
);

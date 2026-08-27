CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY,
    email TEXT,
    last_name TEXT,
    first_name TEXT,
    avatar TEXT NOT NULL,
    role TEXT,
    about_me TEXT,
    password TEXT NOT NULL,
    is_email_verify INTEGER,
    email_verify_at INTEGER,
    two_factor_enable INTEGER,
    is_registered INTEGER NOT NULL,
    added_by INTEGER,
    group_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    is_owner INTEGER
);

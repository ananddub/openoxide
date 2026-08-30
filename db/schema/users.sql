CREATE TABLE users (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    email TEXT,
    last_name TEXT,
    first_name TEXT,
    avatar TEXT NOT NULL,
    role TEXT,
    about_me TEXT,
    password TEXT NOT NULL,
    is_email_verify BIGINT,
    email_verify_at BIGINT,
    two_factor_enable BIGINT,
    is_registered BIGINT NOT NULL,
    added_by BIGINT,
    group_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    is_owner BIGINT
);

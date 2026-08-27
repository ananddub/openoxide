CREATE TABLE notif_resend (
    id INTEGER NOT NULL PRIMARY KEY,
    api_key TEXT NOT NULL,
    from_address TEXT NOT NULL,
    to_addresses TEXT NOT NULL
);

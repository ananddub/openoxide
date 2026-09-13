CREATE TABLE ssh_keys (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    private_key TEXT NOT NULL,
    public_key TEXT NOT NULL,
    last_used_at BIGINT,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

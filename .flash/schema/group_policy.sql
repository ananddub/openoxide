CREATE TABLE group_policy (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    group_id BIGINT NOT NULL,
    policy_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);

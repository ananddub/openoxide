CREATE TABLE user_policy (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id BIGINT NOT NULL,
    org_id BIGINT NOT NULL,
    policy_id BIGINT NOT NULL,
    effect TEXT NOT NULL,
    created_at BIGINT NOT NULL
);

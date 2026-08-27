CREATE TABLE user_policy (
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    org_id INTEGER NOT NULL,
    policy_id INTEGER NOT NULL,
    effect TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

-- Normalise the policy catalog to the four standard verbs:
-- read / create / update / delete.
--   :deploy -> :update, :write -> :update, :monitor -> :read
-- Renames keep their policy id; the three resources whose target action
-- already exists (app, database, schedule) merge their group_policy and
-- user_policy references into the target row before the old row is deleted.

-- 1. Id-preserving renames (target action does not exist yet)
UPDATE policy SET action = 'alert:read'       WHERE action = 'alert:monitor';
UPDATE policy SET action = 'alert:update'     WHERE action = 'alert:write';
UPDATE policy SET action = 'deployment:update' WHERE action = 'deployment:deploy';
UPDATE policy SET action = 'env:update'       WHERE action = 'env:write';
UPDATE policy SET action = 'monitoring:read'  WHERE action = 'monitoring:monitor';
UPDATE policy SET action = 'org:update'       WHERE action = 'org:write';
UPDATE policy SET action = 'traefik:update'   WHERE action = 'traefik:write';

-- 2. Merge app:deploy -> app:update
INSERT INTO group_policy (group_id, policy_id, created_at)
SELECT gp.group_id, t.id, gp.created_at
FROM group_policy gp
JOIN policy o ON o.id = gp.policy_id AND o.action = 'app:deploy'
JOIN policy t ON t.action = 'app:update'
WHERE NOT EXISTS (
    SELECT 1 FROM group_policy x
    WHERE x.group_id = gp.group_id AND x.policy_id = t.id
);
INSERT INTO user_policy (user_id, org_id, policy_id, effect, created_at)
SELECT up.user_id, up.org_id, t.id, up.effect, up.created_at
FROM user_policy up
JOIN policy o ON o.id = up.policy_id AND o.action = 'app:deploy'
JOIN policy t ON t.action = 'app:update'
WHERE NOT EXISTS (
    SELECT 1 FROM user_policy x
    WHERE x.user_id = up.user_id AND x.org_id = up.org_id AND x.policy_id = t.id
);
DELETE FROM group_policy WHERE policy_id IN (SELECT id FROM policy WHERE action = 'app:deploy');
DELETE FROM user_policy  WHERE policy_id IN (SELECT id FROM policy WHERE action = 'app:deploy');
DELETE FROM policy WHERE action = 'app:deploy';

-- 3. Merge database:deploy -> database:update
INSERT INTO group_policy (group_id, policy_id, created_at)
SELECT gp.group_id, t.id, gp.created_at
FROM group_policy gp
JOIN policy o ON o.id = gp.policy_id AND o.action = 'database:deploy'
JOIN policy t ON t.action = 'database:update'
WHERE NOT EXISTS (
    SELECT 1 FROM group_policy x
    WHERE x.group_id = gp.group_id AND x.policy_id = t.id
);
INSERT INTO user_policy (user_id, org_id, policy_id, effect, created_at)
SELECT up.user_id, up.org_id, t.id, up.effect, up.created_at
FROM user_policy up
JOIN policy o ON o.id = up.policy_id AND o.action = 'database:deploy'
JOIN policy t ON t.action = 'database:update'
WHERE NOT EXISTS (
    SELECT 1 FROM user_policy x
    WHERE x.user_id = up.user_id AND x.org_id = up.org_id AND x.policy_id = t.id
);
DELETE FROM group_policy WHERE policy_id IN (SELECT id FROM policy WHERE action = 'database:deploy');
DELETE FROM user_policy  WHERE policy_id IN (SELECT id FROM policy WHERE action = 'database:deploy');
DELETE FROM policy WHERE action = 'database:deploy';

-- 4. Merge schedule:deploy -> schedule:update
INSERT INTO group_policy (group_id, policy_id, created_at)
SELECT gp.group_id, t.id, gp.created_at
FROM group_policy gp
JOIN policy o ON o.id = gp.policy_id AND o.action = 'schedule:deploy'
JOIN policy t ON t.action = 'schedule:update'
WHERE NOT EXISTS (
    SELECT 1 FROM group_policy x
    WHERE x.group_id = gp.group_id AND x.policy_id = t.id
);
INSERT INTO user_policy (user_id, org_id, policy_id, effect, created_at)
SELECT up.user_id, up.org_id, t.id, up.effect, up.created_at
FROM user_policy up
JOIN policy o ON o.id = up.policy_id AND o.action = 'schedule:deploy'
JOIN policy t ON t.action = 'schedule:update'
WHERE NOT EXISTS (
    SELECT 1 FROM user_policy x
    WHERE x.user_id = up.user_id AND x.org_id = up.org_id AND x.policy_id = t.id
);
DELETE FROM group_policy WHERE policy_id IN (SELECT id FROM policy WHERE action = 'schedule:deploy');
DELETE FROM user_policy  WHERE policy_id IN (SELECT id FROM policy WHERE action = 'schedule:deploy');
DELETE FROM policy WHERE action = 'schedule:deploy';

-- Per-deployment AI diagnosis of a failed build/deploy. Keyed by deployment,
-- with an error signature for cross-deployment reuse of the same diagnosis.
CREATE TABLE IF NOT EXISTS ai_build_diagnosis (
    deployment_id   INTEGER NOT NULL PRIMARY KEY,
    service_id      INTEGER,
    classification  TEXT    NOT NULL,          -- fixable | code_error | unknown
    root_cause      TEXT    NOT NULL DEFAULT '',
    explanation     TEXT    NOT NULL DEFAULT '',
    fix_target      TEXT,                        -- compose | dockerfile | env
    fix_content     TEXT,
    error_signature TEXT    NOT NULL DEFAULT '',
    created_at      INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_ai_build_diagnosis_sig
    ON ai_build_diagnosis (error_signature);

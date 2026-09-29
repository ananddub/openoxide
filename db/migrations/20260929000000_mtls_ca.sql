-- Per-server isolated mTLS. Each server (by DB id) gets its own CA plus the
-- panel-client and agent-server identities signed by that CA. Nothing is
-- shared between servers, so one server's leaked material cannot impersonate
-- another. Generated once when NULL, then reused forever. The CA private key
-- lives only here (panel DB) and is never sent to an agent. Stored as BLOB
-- (PEM bytes).
ALTER TABLE servers ADD COLUMN mtls_ca_cert    BLOB;
ALTER TABLE servers ADD COLUMN mtls_ca_key     BLOB;
ALTER TABLE servers ADD COLUMN mtls_panel_cert BLOB;
ALTER TABLE servers ADD COLUMN mtls_panel_key  BLOB;
ALTER TABLE servers ADD COLUMN mtls_agent_cert BLOB;
ALTER TABLE servers ADD COLUMN mtls_agent_key  BLOB;

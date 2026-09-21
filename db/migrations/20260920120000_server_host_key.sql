-- Pin each server's SSH host key. The panel captures the key during the first
-- successful setup/audit connection (trust-on-first-capture) and every later
-- deployment/migration connection verifies against this stored value instead of
-- accepting any key. Empty string = not yet captured (falls back to insecure,
-- matching pre-migration behaviour).
ALTER TABLE servers ADD COLUMN host_key_fingerprint TEXT NOT NULL DEFAULT '';

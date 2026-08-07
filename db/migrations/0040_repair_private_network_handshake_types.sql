-- A previous health update bound health_error into last_handshake_at.
-- Clear those invalid TEXT values so the INTEGER model can decode safely.
UPDATE server_private_networks
SET last_handshake_at = NULL
WHERE typeof(last_handshake_at) = 'text';

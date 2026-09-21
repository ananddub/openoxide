-- Store the target's OpenSSH public key ("<algorithm> <base64-blob>") captured
-- at the same time as the host-key fingerprint. rsync shells out to system
-- `ssh`, which authenticates the remote via a `known_hosts` file — that needs
-- the full key, not a bare SHA-256 fingerprint. Prefixed with the connect host
-- at use time so the value stays valid across a server's public/private IPs.
ALTER TABLE servers ADD COLUMN host_key_openssh TEXT NOT NULL DEFAULT '';

use ssh_key::rand_core::{TryCryptoRng, TryRng};
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use std::convert::Infallible;

/// The OS random source, adapted to the `rand_core` traits `ssh-key` expects.
///
/// `rand_core` 0.10 defines the traits but ships no OS-backed implementation,
/// so this bridges `getrandom` — the same syscall wrapper the rest of the
/// ecosystem uses — into them. `Rng` and `CryptoRng` come free via blanket
/// impls once the fallible pair is in place.
struct OsRng;

impl TryRng for OsRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut buf = [0u8; 4];
        self.try_fill_bytes(&mut buf)?;
        Ok(u32::from_ne_bytes(buf))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut buf = [0u8; 8];
        self.try_fill_bytes(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        // A failing OS RNG means the system has no entropy source at all.
        // There is no safe fallback — generating a key from a degraded source
        // would be worse than not generating one.
        getrandom::fill(dest).expect("OS random source unavailable");
        Ok(())
    }
}

impl TryCryptoRng for OsRng {}

/// Generates an SSH keypair. Returns `(private_key, public_key)` in OpenSSH
/// format — the same encoding `ssh-keygen` emits, so stored keys and anything
/// consuming them are unaffected.
///
/// Generated in-process rather than by shelling out to `ssh-keygen`. The old
/// path wrote the private key to a temp file and read it back, which meant the
/// key touched disk before we ever saw it, and required `ssh-keygen` to be
/// installed. Both are gone; the key now exists only in memory.
///
/// RSA keys are 4096-bit, matching the previous `ssh-keygen -b 4096`. That
/// generation is genuinely slow (~0.5-1 s of prime search) and the cost is
/// unchanged — it is arithmetic, not I/O. ed25519 is effectively instant and is
/// the better default.
pub fn generate_keypair(key_type: &str) -> Result<(String, String), std::io::Error> {
    let algorithm = match key_type.to_ascii_lowercase().as_str() {
        "ed25519" => Algorithm::Ed25519,
        "rsa" => Algorithm::Rsa { hash: None },
        other => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("unsupported key type {other:?}: expected \"ed25519\" or \"rsa\""),
            ));
        }
    };

    let private_key = PrivateKey::random(&mut OsRng, algorithm).map_err(|e| {
        std::io::Error::other(format!("could not generate {key_type} keypair: {e}"))
    })?;

    let private_pem = private_key
        .to_openssh(LineEnding::LF)
        .map_err(|e| std::io::Error::other(format!("could not encode private key: {e}")))?
        .to_string();

    let public_openssh = private_key
        .public_key()
        .to_openssh()
        .map_err(|e| std::io::Error::other(format!("could not encode public key: {e}")))?;

    Ok((private_pem, public_openssh))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_an_ed25519_keypair() {
        let (private_key, public_key) = generate_keypair("ed25519").expect("generation");

        assert!(private_key.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"));
        assert!(private_key.ends_with("-----END OPENSSH PRIVATE KEY-----\n"));
        assert!(public_key.starts_with("ssh-ed25519 "));
    }

    #[test]
    fn the_key_type_is_case_insensitive() {
        assert!(generate_keypair("ED25519").is_ok());
        assert!(generate_keypair("Ed25519").is_ok());
    }

    #[test]
    fn rejects_an_unknown_key_type() {
        let error = generate_keypair("dsa").expect_err("dsa is not supported");

        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        assert!(
            error.to_string().contains("dsa"),
            "the message should name the rejected type: {error}"
        );
    }

    #[test]
    fn every_keypair_is_distinct() {
        let (first, _) = generate_keypair("ed25519").unwrap();
        let (second, _) = generate_keypair("ed25519").unwrap();

        assert_ne!(first, second, "keys must not be reused between calls");
    }

    #[test]
    fn the_generated_key_round_trips_through_the_parser() {
        // Proves the output really is valid OpenSSH format rather than merely
        // looking like it.
        let (private_key, public_key) = generate_keypair("ed25519").unwrap();

        let parsed = PrivateKey::from_openssh(&private_key).expect("private key should parse");
        assert_eq!(
            parsed.public_key().to_openssh().unwrap(),
            public_key,
            "the public key must match the one derived from the private key"
        );
    }

    /// RSA-4096 takes roughly a second, so it is kept out of the default run.
    #[test]
    #[ignore = "slow: RSA-4096 key generation"]
    fn generates_an_rsa_keypair() {
        let (private_key, public_key) = generate_keypair("rsa").expect("generation");

        assert!(private_key.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"));
        assert!(public_key.starts_with("ssh-rsa "));

        let parsed = PrivateKey::from_openssh(&private_key).unwrap();
        assert!(matches!(parsed.algorithm(), Algorithm::Rsa { .. }));
    }
}

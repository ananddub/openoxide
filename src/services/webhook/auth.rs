use sha2::{Digest, Sha256};

pub fn verify_hmac_sha256(secret: &[u8], body: &[u8], signature: &str) -> bool {
    let signature = signature.strip_prefix("sha256=").unwrap_or(signature);
    let Some(expected) = decode_hex(signature) else {
        return false;
    };
    constant_time_eq(&hmac_sha256(secret, body), &expected)
}

pub fn sign_hmac_sha256(secret: &[u8], body: &[u8]) -> String {
    hmac_sha256(secret, body)
        .into_iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub fn verify_token(expected: &str, received: Option<&str>) -> bool {
    received.is_some_and(|received| constant_time_eq(expected.as_bytes(), received.as_bytes()))
}

fn hmac_sha256(secret: &[u8], body: &[u8]) -> Vec<u8> {
    const BLOCK: usize = 64;
    let mut key = if secret.len() > BLOCK {
        Sha256::digest(secret).to_vec()
    } else {
        secret.to_vec()
    };
    key.resize(BLOCK, 0);

    let mut inner_pad = [0x36_u8; BLOCK];
    let mut outer_pad = [0x5c_u8; BLOCK];
    for (index, byte) in key.iter().enumerate() {
        inner_pad[index] ^= byte;
        outer_pad[index] ^= byte;
    }

    let inner = Sha256::new()
        .chain_update(inner_pad)
        .chain_update(body)
        .finalize();
    Sha256::new()
        .chain_update(outer_pad)
        .chain_update(inner)
        .finalize()
        .to_vec()
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = (pair[0] as char).to_digit(16)?;
            let low = (pair[1] as char).to_digit(16)?;
            Some(((high << 4) | low) as u8)
        })
        .collect()
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0_u8, |diff, (left, right)| diff | (left ^ right))
        == 0
}

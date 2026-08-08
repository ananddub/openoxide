use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_hmac_sha256(secret: &[u8], body: &[u8], signature: &str) -> bool {
    let signature = signature.strip_prefix("sha256=").unwrap_or(signature);
    let Some(expected) = decode_hex(signature) else {
        return false;
    };
    HmacSha256::new_from_slice(secret).is_ok_and(|mut mac| {
        mac.update(body);
        mac.verify_slice(&expected).is_ok()
    })
}

pub fn sign_hmac_sha256(secret: &[u8], body: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC accepts keys of any size");
    mac.update(body);
    mac.finalize()
        .into_bytes()
        .into_iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub fn verify_token(expected: &str, received: Option<&str>) -> bool {
    received.is_some_and(|received| {
        let mut expected_mac =
            HmacSha256::new_from_slice(expected.as_bytes()).expect("HMAC accepts keys of any size");
        expected_mac.update(b"rustploy-token-verification");
        let expected_tag = expected_mac.finalize().into_bytes();

        let mut received_mac =
            HmacSha256::new_from_slice(received.as_bytes()).expect("HMAC accepts keys of any size");
        received_mac.update(b"rustploy-token-verification");
        received_mac.verify_slice(&expected_tag).is_ok()
    })
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

//! Record content normalisation shared by the provider clients.
//!
//! The panel stores a record value as one plain string ("10 mail.example.com", "0 issue
//! \"letsencrypt.org\""), because that is what an operator types. Providers that want
//! structured fields (Cloudflare `data`, Route53 quoting, Porkbun `prio`) build them here
//! and render them back the same way, so a listed row always matches what was written.

use crate::error::DnsError;

/// Record types every provider client accepts. Kept here so the panel can offer the same
/// list in its form without inventing a second one.
pub const COMMON_TYPES: &[&str] = &["A", "AAAA", "CNAME", "NS", "MX", "TXT", "SRV", "CAA"];

/// Types Cloudflare is willing to proxy; every other type is rejected if `proxied` is set.
pub fn is_proxyable(record_type: &str) -> bool {
    matches!(
        record_type.to_ascii_uppercase().as_str(),
        "A" | "AAAA" | "CNAME"
    )
}

/// A record name as the panel knows it: no trailing dot, no surrounding space.
pub fn bare(name: &str) -> String {
    name.trim().trim_end_matches('.').to_string()
}

/// A record name relative to its zone — "api" for `api.example.com` in `example.com`, and
/// "@" for the apex, which is what several providers expect in a form field.
pub fn relative_name(name: &str, zone: &str) -> String {
    let name = bare(name);
    let zone = bare(zone);
    if name == zone {
        return "@".to_string();
    }
    name.strip_suffix(&format!(".{zone}"))
        .unwrap_or(&name)
        .to_string()
}

/// The FQDN a provider should store for a form value. "@" and the bare zone name both mean
/// the apex, and an empty name is the apex too.
pub fn to_fqdn(name: &str, zone: &str) -> String {
    let name = bare(name);
    let zone = bare(zone);
    if name.is_empty() || name == "@" {
        return zone;
    }
    if name == zone || name.ends_with(&format!(".{zone}")) {
        return name;
    }
    format!("{name}.{zone}")
}

/// Rejects a type the provider does not implement, with the provider's own list in the
/// message so the operator sees what is possible instead of an opaque 400.
pub fn ensure_type(
    provider: &str,
    record_type: &str,
    allowed: &[&str],
) -> Result<String, DnsError> {
    let normalized = record_type.trim().to_ascii_uppercase();
    if allowed
        .iter()
        .any(|kind| kind.eq_ignore_ascii_case(&normalized))
    {
        Ok(normalized)
    } else {
        Err(DnsError::provider(format!(
            "{provider}: unsupported record type '{}'",
            record_type.trim()
        )))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MxValue {
    pub priority: u16,
    pub host: String,
}

/// Splits "10 mail.example.com". A missing preference defaults to 10, matching what the
/// providers apply when the field is left out.
pub fn parse_mx(content: &str) -> Result<MxValue, DnsError> {
    let parts: Vec<&str> = content.split_whitespace().collect();
    match parts.as_slice() {
        [host] => Ok(MxValue {
            priority: 10,
            host: host.to_string(),
        }),
        [priority, host] => Ok(MxValue {
            priority: parse_u16("MX priority", priority)?,
            host: host.to_string(),
        }),
        _ => Err(DnsError::provider(
            "an MX value must be \"priority host\", got an empty or malformed record",
        )),
    }
}

pub fn render_mx(value: &MxValue) -> String {
    format!("{} {}", value.priority, value.host)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrvValue {
    pub priority: u16,
    pub weight: u16,
    pub port: u16,
    pub target: String,
}

/// Splits "10 60 5060 sip.example.com".
pub fn parse_srv(content: &str) -> Result<SrvValue, DnsError> {
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() != 4 {
        return Err(DnsError::provider(format!(
            "an SRV value must be \"priority weight port target\", got {content:?}"
        )));
    }
    Ok(SrvValue {
        priority: parse_u16("SRV priority", parts[0])?,
        weight: parse_u16("SRV weight", parts[1])?,
        port: parse_u16("SRV port", parts[2])?,
        target: parts[3].to_string(),
    })
}

pub fn render_srv(value: &SrvValue) -> String {
    format!(
        "{} {} {} {}",
        value.priority, value.weight, value.port, value.target
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaaValue {
    pub flags: u8,
    pub tag: String,
    pub value: String,
}

/// Splits `0 issue "letsencrypt.org"`. Quotes around the value are optional.
pub fn parse_caa(content: &str) -> Result<CaaValue, DnsError> {
    let trimmed = content.trim();
    let error = || {
        DnsError::provider(format!(
            "a CAA value must be `flags tag \"value\"`, got {content:?}"
        ))
    };
    let mut parts = trimmed.splitn(3, char::is_whitespace);
    let flags = parts.next().ok_or_else(error)?;
    let tag = parts.next().ok_or_else(error)?;
    let value = parts.next().ok_or_else(error)?;
    Ok(CaaValue {
        flags: flags
            .parse::<u8>()
            .map_err(|_| DnsError::provider(format!("invalid CAA flags {flags:?}")))?,
        tag: tag.to_string(),
        value: value.trim().trim_matches('"').to_string(),
    })
}

pub fn render_caa(value: &CaaValue) -> String {
    format!("{} {} \"{}\"", value.flags, value.tag, value.value)
}

/// Route53 stores TXT and CAA values wrapped in double quotes; the panel shows and accepts
/// the bare string, so the quotes come off on the way out.
pub fn unquote(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        return trimmed[1..trimmed.len() - 1].replace("\\\"", "\"");
    }
    trimmed.to_string()
}

/// Inverse of [`unquote`] — Route53 rejects an unquoted TXT/CAA value.
pub fn quote(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        return trimmed.to_string();
    }
    format!("\"{}\"", trimmed.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Wraps a TXT value for the providers that store it quoted (Infomaniak, OVH), leaving every
/// other type as the plain string the operator typed.
pub fn quote_txt(record_type: &str, content: &str) -> String {
    if record_type.eq_ignore_ascii_case("TXT") {
        quote(content)
    } else {
        content.trim().to_string()
    }
}

fn parse_u16(what: &str, raw: &str) -> Result<u16, DnsError> {
    raw.parse::<u16>()
        .map_err(|_| DnsError::provider(format!("invalid {what} {raw:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_normalise_both_ways() {
        assert_eq!(relative_name("api.example.com.", "example.com"), "api");
        assert_eq!(relative_name("example.com", "example.com"), "@");
        assert_eq!(to_fqdn("@", "example.com"), "example.com");
        assert_eq!(to_fqdn("api", "example.com"), "api.example.com");
        // Already-absolute values must not gain a second suffix.
        assert_eq!(to_fqdn("api.example.com", "example.com"), "api.example.com");
    }

    #[test]
    fn structured_values_round_trip() {
        assert_eq!(parse_mx("mail.example.com").unwrap().priority, 10);
        let mx = parse_mx("20 mail.example.com").unwrap();
        assert_eq!(render_mx(&mx), "20 mail.example.com");

        let srv = parse_srv("10 60 5060 sip.example.com").unwrap();
        assert_eq!(render_srv(&srv), "10 60 5060 sip.example.com");
        assert!(parse_srv("10 60 sip.example.com").is_err());

        let caa = parse_caa("0 issue \"letsencrypt.org\"").unwrap();
        assert_eq!(caa.tag, "issue");
        assert_eq!(caa.value, "letsencrypt.org");
        assert_eq!(render_caa(&caa), "0 issue \"letsencrypt.org\"");
    }

    #[test]
    fn txt_quotes_round_trip() {
        assert_eq!(unquote("\"v=spf1 -all\""), "v=spf1 -all");
        assert_eq!(quote("v=spf1 -all"), "\"v=spf1 -all\"");
        assert_eq!(quote("\"v=spf1 -all\""), "\"v=spf1 -all\"");
        assert_eq!(quote(r#"a"b"#), r#""a\"b""#);
    }

    #[test]
    fn unsupported_type_names_the_allowed_list() {
        let error = ensure_type("Cloudflare", "sshfp", COMMON_TYPES).unwrap_err();
        assert!(error.to_string().contains("Cloudflare"), "{error}");
        assert_eq!(ensure_type("Cloudflare", " a ", COMMON_TYPES).unwrap(), "A");
    }
}

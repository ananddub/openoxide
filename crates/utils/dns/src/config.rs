use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

use crate::error::DnsError;

/// Every DNS provider the panel supports.
///
/// This enum is the single source of truth: the `dns_providers.provider_type` column, the
/// `DnsProviderType` proto enum and the web catalog all key off `as_str`, and
/// `credential_fields` drives both server-side validation and the shape the create form
/// collects. Adding a provider means a variant here, a client in `providers/` and a proto
/// value — anything less is a provider that cannot be used, anything more is dead weight.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DnsProviderKind {
    #[default]
    Cloudflare,
    Route53,
    Hostinger,
    Porkbun,
    Infomaniak,
    Ovh,
    Spaceship,
}

/// One credential the provider needs. `key` is the JSON field name stored in
/// `credentials_json` and rendered by the web form; `secret` decides whether the panel
/// masks it on read and asks for it again on write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CredentialField {
    pub key: &'static str,
    pub secret: bool,
    pub required: bool,
}

impl CredentialField {
    const fn field(key: &'static str, secret: bool, required: bool) -> Self {
        Self {
            key,
            secret,
            required,
        }
    }

    const fn required(key: &'static str, secret: bool) -> Self {
        Self::field(key, secret, true)
    }
}

/// The stored JSON key names are camelCase and identical to the ones the web form already
/// sends, so rows created before this crate keep validating.
const CLOUDFLARE_FIELDS: [CredentialField; 1] = [CredentialField::required("apiToken", true)];
const ROUTE53_FIELDS: [CredentialField; 2] = [
    CredentialField::required("accessKeyId", false),
    CredentialField::required("secretAccessKey", true),
];
const HOSTINGER_FIELDS: [CredentialField; 1] = [CredentialField::required("apiToken", true)];
const PORKBUN_FIELDS: [CredentialField; 2] = [
    CredentialField::required("apiKey", false),
    CredentialField::required("secretApiKey", true),
];
const INFOMANIAK_FIELDS: [CredentialField; 1] = [CredentialField::required("apiToken", true)];
const OVH_FIELDS: [CredentialField; 4] = [
    CredentialField::field("endpoint", false, false),
    CredentialField::required("applicationKey", false),
    CredentialField::required("applicationSecret", true),
    CredentialField::required("consumerKey", true),
];
const SPACESHIP_FIELDS: [CredentialField; 2] = [
    CredentialField::required("apiKey", false),
    CredentialField::required("apiSecret", true),
];

impl DnsProviderKind {
    pub const ALL: [Self; 7] = [
        Self::Cloudflare,
        Self::Route53,
        Self::Hostinger,
        Self::Porkbun,
        Self::Infomaniak,
        Self::Ovh,
        Self::Spaceship,
    ];

    /// Wire format: SCREAMING_SNAKE without word-splitting (`DIGITALOCEAN`, not
    /// `DIGITAL_OCEAN`), because these strings are already stored in the database.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cloudflare => "CLOUDFLARE",
            Self::Route53 => "ROUTE53",
            Self::Hostinger => "HOSTINGER",
            Self::Porkbun => "PORKBUN",
            Self::Infomaniak => "INFOMANIAK",
            Self::Ovh => "OVH",
            Self::Spaceship => "SPACESHIP",
        }
    }

    pub fn from_code(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.as_str() == value.trim().to_ascii_uppercase())
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Cloudflare => "Cloudflare DNS",
            Self::Route53 => "Amazon Route53",
            Self::Hostinger => "Hostinger DNS",
            Self::Porkbun => "Porkbun DNS",
            Self::Infomaniak => "Infomaniak DNS",
            Self::Ovh => "OVHcloud DNS",
            Self::Spaceship => "Spaceship DNS",
        }
    }

    /// The credential schema for this provider. Validation and masking both read this list,
    /// so a field cannot be required in one place and ignored in the other.
    pub fn credential_fields(self) -> &'static [CredentialField] {
        match self {
            Self::Cloudflare => &CLOUDFLARE_FIELDS,
            Self::Route53 => &ROUTE53_FIELDS,
            Self::Hostinger => &HOSTINGER_FIELDS,
            Self::Porkbun => &PORKBUN_FIELDS,
            Self::Infomaniak => &INFOMANIAK_FIELDS,
            Self::Ovh => &OVH_FIELDS,
            Self::Spaceship => &SPACESHIP_FIELDS,
        }
    }

    pub fn secret_keys(self) -> impl Iterator<Item = &'static str> {
        self.credential_fields()
            .iter()
            .filter(|field| field.secret)
            .map(|field| field.key)
    }

    /// Parses `credentials_json` and checks every required field carries a value.
    pub fn validate_credentials(self, credentials_json: &str) -> Result<(), DnsError> {
        let parsed: serde_json::Value = serde_json::from_str(credentials_json)
            .map_err(|error| DnsError::Credentials(format!("not a JSON object: {error}")))?;
        let object = parsed
            .as_object()
            .ok_or_else(|| DnsError::Credentials("expected a JSON object".into()))?;
        for field in self.credential_fields() {
            let value = object
                .get(field.key)
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .trim();
            if value.is_empty() && field.required {
                return Err(DnsError::Credentials(format!(
                    "field '{}' is required for {}",
                    field.key,
                    self.label()
                )));
            }
        }
        Ok(())
    }
}

impl Serialize for DnsProviderKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DnsProviderKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::from_code(&raw)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown DNS provider type '{raw}'")))
    }
}

impl FromStr for DnsProviderKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_code(s).ok_or_else(|| format!("Invalid DnsProviderKind: {s}"))
    }
}

impl std::fmt::Display for DnsProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvhEndpoint {
    Eu,
    Ca,
    Us,
    KimsufiEu,
    KimsufiCa,
    SoyoustartEu,
    SoyoustartCa,
}

impl OvhEndpoint {
    fn from_code(code: &str) -> Option<Self> {
        Some(match code {
            "ovh-eu" => Self::Eu,
            "ovh-ca" => Self::Ca,
            "ovh-us" => Self::Us,
            "kimsufi-eu" => Self::KimsufiEu,
            "kimsufi-ca" => Self::KimsufiCa,
            "soyoustart-eu" => Self::SoyoustartEu,
            "soyoustart-ca" => Self::SoyoustartCa,
            _ => return None,
        })
    }

    /// Base for the `/domain/zone` API.
    pub fn base_url(self) -> &'static str {
        match self {
            Self::Eu => "https://eu.api.ovh.com/1.0",
            Self::Ca => "https://ca.api.ovh.com/1.0",
            Self::Us => "https://api.us.ovhcloud.com/1.0",
            Self::KimsufiEu => "https://eu.api.kimsufi.com/1.0",
            Self::KimsufiCa => "https://ca.api.kimsufi.com/1.0",
            Self::SoyoustartEu => "https://eu.api.soyoustart.com/1.0",
            Self::SoyoustartCa => "https://ca.api.soyoustart.com/1.0",
        }
    }

    /// Base for the unauthenticated `/auth/time` clock-skew probe, which lives outside the
    /// `/1.0` API prefix.
    pub fn auth_base_url(self) -> &'static str {
        match self {
            Self::Eu => "https://eu.api.ovh.com",
            Self::Ca => "https://ca.api.ovh.com",
            Self::Us => "https://api.us.ovhcloud.com",
            Self::KimsufiEu => "https://eu.api.kimsufi.com",
            Self::KimsufiCa => "https://ca.api.kimsufi.com",
            Self::SoyoustartEu => "https://eu.api.soyoustart.com",
            Self::SoyoustartCa => "https://ca.api.soyoustart.com",
        }
    }
}

/// Typed credentials for one provider. Each variant is what the JSON blob decodes to, so a
/// provider client never re-reads a string out of a map and never guesses a key name.
#[derive(Debug, Clone)]
pub enum ProviderConfig {
    Cloudflare(CloudflareConfig),
    Route53(Route53Config),
    Hostinger(HostingerConfig),
    Porkbun(PorkbunConfig),
    Infomaniak(InfomaniakConfig),
    Ovh(OvhConfig),
    Spaceship(SpaceshipConfig),
}

impl ProviderConfig {
    /// Decodes a stored credential blob for the given provider.
    pub fn from_json(kind: DnsProviderKind, credentials_json: &str) -> Result<Self, DnsError> {
        kind.validate_credentials(credentials_json)?;
        let config = match kind {
            DnsProviderKind::Cloudflare => Self::Cloudflare(deserialize(credentials_json)?),
            DnsProviderKind::Route53 => Self::Route53(deserialize(credentials_json)?),
            DnsProviderKind::Hostinger => Self::Hostinger(deserialize(credentials_json)?),
            DnsProviderKind::Porkbun => Self::Porkbun(deserialize(credentials_json)?),
            DnsProviderKind::Infomaniak => Self::Infomaniak(deserialize(credentials_json)?),
            // The endpoint picks the host, so an invented one has to fail here rather than sit
            // in a row until a request finds nothing at that address.
            DnsProviderKind::Ovh => {
                let config: OvhConfig = deserialize(credentials_json)?;
                config.endpoint()?;
                Self::Ovh(config)
            }
            DnsProviderKind::Spaceship => Self::Spaceship(deserialize(credentials_json)?),
        };
        Ok(config)
    }

    pub fn kind(&self) -> DnsProviderKind {
        match self {
            Self::Cloudflare(_) => DnsProviderKind::Cloudflare,
            Self::Route53(_) => DnsProviderKind::Route53,
            Self::Hostinger(_) => DnsProviderKind::Hostinger,
            Self::Porkbun(_) => DnsProviderKind::Porkbun,
            Self::Infomaniak(_) => DnsProviderKind::Infomaniak,
            Self::Ovh(_) => DnsProviderKind::Ovh,
            Self::Spaceship(_) => DnsProviderKind::Spaceship,
        }
    }

    /// Reads the config variant a provider client expects. A mismatch means the row's
    /// `provider_type` and its credential blob disagree, which is a bug in the caller rather
    /// than something a provider should paper over.
    pub(crate) fn expect<T>(&self, found: fn(&Self) -> Option<&T>) -> Result<&T, DnsError> {
        found(self).ok_or_else(|| {
            DnsError::Credentials(format!(
                "credentials do not match the {} provider",
                self.kind().label()
            ))
        })
    }

    pub(crate) fn cloudflare(&self) -> Result<&CloudflareConfig, DnsError> {
        self.expect(|config| match config {
            Self::Cloudflare(value) => Some(value),
            _ => None,
        })
    }

    pub(crate) fn route53(&self) -> Result<&Route53Config, DnsError> {
        self.expect(|config| match config {
            Self::Route53(value) => Some(value),
            _ => None,
        })
    }

    pub(crate) fn hostinger(&self) -> Result<&HostingerConfig, DnsError> {
        self.expect(|config| match config {
            Self::Hostinger(value) => Some(value),
            _ => None,
        })
    }

    pub(crate) fn porkbun(&self) -> Result<&PorkbunConfig, DnsError> {
        self.expect(|config| match config {
            Self::Porkbun(value) => Some(value),
            _ => None,
        })
    }

    pub(crate) fn infomaniak(&self) -> Result<&InfomaniakConfig, DnsError> {
        self.expect(|config| match config {
            Self::Infomaniak(value) => Some(value),
            _ => None,
        })
    }

    pub(crate) fn ovh(&self) -> Result<&OvhConfig, DnsError> {
        self.expect(|config| match config {
            Self::Ovh(value) => Some(value),
            _ => None,
        })
    }

    pub(crate) fn spaceship(&self) -> Result<&SpaceshipConfig, DnsError> {
        self.expect(|config| match config {
            Self::Spaceship(value) => Some(value),
            _ => None,
        })
    }
}

fn deserialize<T: for<'de> Deserialize<'de>>(credentials_json: &str) -> Result<T, DnsError> {
    serde_json::from_str(credentials_json).map_err(|error| DnsError::Credentials(error.to_string()))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareConfig {
    pub api_token: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route53Config {
    pub access_key_id: String,
    pub secret_access_key: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostingerConfig {
    pub api_token: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PorkbunConfig {
    pub api_key: String,
    pub secret_api_key: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfomaniakConfig {
    pub api_token: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OvhConfig {
    #[serde(default)]
    pub endpoint: String,
    pub application_key: String,
    pub application_secret: String,
    pub consumer_key: String,
}

impl OvhConfig {
    pub fn endpoint(&self) -> Result<OvhEndpoint, DnsError> {
        let code = self.endpoint.trim();
        let code = if code.is_empty() { "ovh-eu" } else { code };
        OvhEndpoint::from_code(code)
            .ok_or_else(|| DnsError::Credentials(format!("OVH: unknown endpoint '{code}'")))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceshipConfig {
    pub api_key: String,
    pub api_secret: String,
}

#[cfg(test)]
mod tests {
    use super::{DnsProviderKind, ProviderConfig};

    #[test]
    fn every_kind_round_trips_its_wire_code() {
        for kind in DnsProviderKind::ALL {
            assert_eq!(DnsProviderKind::from_code(kind.as_str()), Some(kind));
            assert_eq!(kind.as_str(), kind.to_string());
        }
        assert_eq!(DnsProviderKind::from_code("digitalocean"), None);
    }

    #[test]
    fn required_fields_are_checked_after_trimming() {
        let error = DnsProviderKind::Porkbun
            .validate_credentials(r#"{"apiKey":"  ","secretApiKey":"s"}"#)
            .unwrap_err();
        assert!(error.to_string().contains("apiKey"), "{error}");
        DnsProviderKind::Porkbun
            .validate_credentials(r#"{"apiKey":"k","secretApiKey":"s"}"#)
            .unwrap();
    }

    #[test]
    fn optional_endpoint_does_not_block_parsing() {
        let config = ProviderConfig::from_json(
            DnsProviderKind::Ovh,
            r#"{"applicationKey":"a","applicationSecret":"s","consumerKey":"c"}"#,
        )
        .unwrap();
        let ProviderConfig::Ovh(ovh) = config else {
            panic!("expected an OVH config");
        };
        assert_eq!(
            ovh.endpoint().unwrap().base_url(),
            "https://eu.api.ovh.com/1.0"
        );
    }

    #[test]
    fn unknown_ovh_endpoint_is_rejected() {
        let error = ProviderConfig::from_json(
            DnsProviderKind::Ovh,
            r#"{"endpoint":"ovh-mars","applicationKey":"a","applicationSecret":"s","consumerKey":"c"}"#,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("ovh-mars"), "{error}");
    }
}

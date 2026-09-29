//! Dokploy-style `template.toml` resolution using a real TOML parser.
//! Generators: `${password:N}`, `${base64:N}`, `${hex:N}`, `${uuid}`,
//! `${domain}`, `${email}`, `${username}`, `${timestamp}`. Unknown `${...}`
//! refs are left verbatim for downstream compose interpolation.

use std::collections::HashMap;

use base64::Engine;
use serde::Deserialize;

const DEFAULT_LEN: usize = 32;

#[derive(Debug, Clone)]
pub struct ResolvedDomain {
    pub service_name: String,
    pub port: i64,
    pub host: String,
    pub path: Option<String>,
}

#[derive(Debug, Default)]
pub struct ResolvedTemplate {
    /// `.env`-style KEY=VALUE block for the compose service.
    pub env: String,
    pub domains: Vec<ResolvedDomain>,
    pub mounts: Vec<ResolvedMount>,
}

#[derive(Debug, Clone)]
pub struct ResolvedMount {
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct TemplateToml {
    #[serde(default)]
    variables: HashMap<String, toml::Value>,
    #[serde(default)]
    config: ConfigSection,
}

#[derive(Debug, Default, Deserialize)]
struct ConfigSection {
    #[serde(default)]
    env: EnvSection,
    #[serde(default)]
    domains: Vec<DomainEntry>,
    #[serde(default)]
    mounts: Vec<MountEntry>,
}

#[derive(Debug, Deserialize)]
struct MountEntry {
    #[serde(rename = "filePath", default)]
    file_path: String,
    #[serde(default)]
    content: String,
}

/// `[config].env` is either a table (`KEY = "value"`) or an array whose items
/// are `"KEY=VALUE"` strings or single-pair `{ KEY = value }` tables.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EnvSection {
    Table(HashMap<String, toml::Value>),
    List(Vec<toml::Value>),
}

impl Default for EnvSection {
    fn default() -> Self {
        Self::List(Vec::new())
    }
}

#[derive(Debug, Deserialize)]
struct DomainEntry {
    #[serde(rename = "serviceName", default)]
    service_name: String,
    #[serde(default)]
    port: i64,
    #[serde(default)]
    host: Option<String>,
    #[serde(default)]
    path: Option<String>,
}

/// Expands generator placeholders in a plain KEY=VALUE block (no TOML).
pub fn expand_env_template(env_block: &str) -> String {
    let mut ctx = Ctx::new(None);
    let vars = HashMap::new();
    env_block
        .lines()
        .map(|line| ctx.substitute(line, &vars))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Resolves a Dokploy `template.toml`. `server_ip` (when known) yields a
/// reachable `sslip.io` placeholder domain. `None` when the TOML has no env
/// entries and no domains.
pub fn resolve_template_toml(template_toml: &str, server_ip: Option<&str>) -> Option<ResolvedTemplate> {
    let parsed: TemplateToml = toml::from_str(template_toml).ok()?;
    let mut ctx = Ctx::new(server_ip.map(str::to_owned));

    // Pass 1: generate each [variables] value.
    let mut vars: HashMap<String, String> = HashMap::new();
    for (name, value) in &parsed.variables {
        if let Some(text) = value.as_str() {
            vars.insert(name.clone(), ctx.substitute(text, &HashMap::new()));
        }
    }
    // Pass 2: resolve variables that reference other variables.
    let names: Vec<String> = vars.keys().cloned().collect();
    for name in names {
        let current = vars.get(&name).cloned().unwrap_or_default();
        let resolved = ctx.substitute(&current, &vars);
        vars.insert(name, resolved);
    }

    // Env entries (table or array) -> resolved KEY=VALUE lines.
    let env_lines: Vec<String> = match &parsed.config.env {
        EnvSection::Table(table) => table
            .iter()
            .map(|(key, value)| {
                let raw = value_to_string(value);
                format!("{key}={}", ctx.substitute(&raw, &vars))
            })
            .collect(),
        EnvSection::List(list) => list
            .iter()
            .filter_map(|item| match item {
                toml::Value::String(entry) => Some(ctx.substitute(entry, &vars)),
                // `{ KEY = value }` — take the single pair, resolve refs.
                toml::Value::Table(table) => table.iter().next().map(|(key, value)| {
                    format!("{key}={}", ctx.substitute(&value_to_string(value), &vars))
                }),
                _ => None,
            })
            .collect(),
    };

    // Domains: resolve host refs; port arrives as an integer (8_080 -> 8080).
    let domains: Vec<ResolvedDomain> = parsed
        .config
        .domains
        .into_iter()
        .filter(|domain| !domain.service_name.is_empty())
        .map(|domain| {
            let host = match domain.host {
                Some(host) => ctx.substitute(&host, &vars),
                None => ctx.random_domain(),
            };
            ResolvedDomain {
                service_name: domain.service_name,
                port: domain.port,
                host,
                path: domain.path,
            }
        })
        .collect();

    // Mounts: resolve refs in filePath/content.
    let mounts: Vec<ResolvedMount> = parsed
        .config
        .mounts
        .into_iter()
        .filter(|mount| !mount.file_path.is_empty())
        .map(|mount| ResolvedMount {
            file_path: ctx.substitute(&mount.file_path, &vars),
            content: ctx.substitute(&mount.content, &vars),
        })
        .collect();

    if env_lines.is_empty() && domains.is_empty() && mounts.is_empty() {
        return None;
    }
    Some(ResolvedTemplate {
        env: env_lines.join("\n"),
        domains,
        mounts,
    })
}

fn value_to_string(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Float(f) => f.to_string(),
        other => other.to_string(),
    }
}

/// Resolution context. `${domain}` yields a fresh host each time; a template
/// that wants a stable host names it in `[variables]` (resolved once) and
/// references that variable.
struct Ctx {
    server_ip: Option<String>,
}

impl Ctx {
    fn new(server_ip: Option<String>) -> Self {
        Self { server_ip }
    }

    /// Replaces every `${...}`: a known variable, else a generator, else kept.
    fn substitute(&mut self, input: &str, vars: &HashMap<String, String>) -> String {
        let bytes = input.as_bytes();
        let mut out = String::with_capacity(input.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
                if let Some(end) = find_close_brace(bytes, i + 2) {
                    let inner = input[i + 2..end].trim();
                    if let Some(value) = vars.get(inner) {
                        out.push_str(value);
                    } else if let Some(value) = self.generate(inner, vars) {
                        out.push_str(&value);
                    } else {
                        out.push_str(&input[i..=end]);
                    }
                    i = end + 1;
                    continue;
                }
            }
            let ch_len = utf8_len(bytes[i]);
            out.push_str(&input[i..i + ch_len]);
            i += ch_len;
        }
        out
    }

    fn generate(&mut self, body: &str, vars: &HashMap<String, String>) -> Option<String> {
        let (keyword, arg) = match body.split_once(':') {
            Some((keyword, arg)) => (keyword.trim(), Some(arg.trim())),
            None => (body.trim(), None),
        };
        let len = arg.and_then(|value| value.parse::<usize>().ok());

        Some(match keyword.to_ascii_lowercase().as_str() {
            "password" => random_password(len.unwrap_or(16)),
            "base64" => random_base64(len.unwrap_or(DEFAULT_LEN)),
            "hex" => random_hex(len.unwrap_or(8)),
            "hash" => random_hex(len.unwrap_or(8)),
            "uuid" => uuid::Uuid::new_v4().to_string(),
            "domain" => self.random_domain(),
            "email" => format!("{}@example.com", random_username()),
            "username" => random_username(),
            "jwt" => sign_jwt(arg, vars),
            "randomport" => (10000 + (random_bytes(2)[0] as u32 % 55000)).to_string(),
            "timestamp" | "timestampms" => match arg {
                Some(date) => parse_date_millis(date).to_string(),
                None => chrono::Utc::now().timestamp_millis().to_string(),
            },
            "timestamps" => match arg {
                Some(date) => (parse_date_millis(date) / 1000).to_string(),
                None => chrono::Utc::now().timestamp().to_string(),
            },
            _ => return None,
        })
    }

    /// A reachable `sslip.io` host when the server IP is known (it resolves to
    /// that IP), else a syntactically valid placeholder the user overrides.
    fn random_domain(&self) -> String {
        let hash = random_hex(3);
        match self.server_ip.as_deref().filter(|ip| !ip.trim().is_empty()) {
            Some(ip) => format!("{hash}-{}.sslip.io", ip.replace('.', "-")),
            None => format!("{hash}.sslip.io"),
        }
    }
}

fn utf8_len(first: u8) -> usize {
    match first {
        b if b < 0x80 => 1,
        b if b >> 5 == 0b110 => 2,
        b if b >> 4 == 0b1110 => 3,
        _ => 4,
    }
}

fn find_close_brace(bytes: &[u8], start: usize) -> Option<usize> {
    (start..bytes.len()).find(|&idx| bytes[idx] == b'}')
}

fn random_bytes(count: usize) -> Vec<u8> {
    let mut bytes = vec![0_u8; count.max(1)];
    let _ = getrandom::fill(&mut bytes);
    bytes
}

fn random_password(len: usize) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let len = len.clamp(1, 128);
    random_bytes(len)
        .into_iter()
        .map(|byte| ALPHABET[(byte as usize) % ALPHABET.len()] as char)
        .collect()
}

fn random_base64(bytes: usize) -> String {
    base64::engine::general_purpose::STANDARD.encode(random_bytes(bytes.clamp(1, 256)))
}

fn random_hex(bytes: usize) -> String {
    random_bytes(bytes.clamp(1, 256))
        .into_iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn random_username() -> String {
    format!("user{}", random_hex(4))
}

/// Parses an RFC3339 date into unix milliseconds (0 on failure).
fn parse_date_millis(date: &str) -> i64 {
    chrono::DateTime::parse_from_rfc3339(date.trim())
        .map(|value| value.timestamp_millis())
        .unwrap_or(0)
}

/// Builds a token for `${jwt...}`. `arg` forms: `None`/`N` -> random opaque
/// token; `secret:payload` -> a real HS256 JWT signed with `secret`, where
/// `secret` and `payload` are resolved from `vars` when they name variables.
fn sign_jwt(arg: Option<&str>, vars: &HashMap<String, String>) -> String {
    let Some(arg) = arg else {
        return random_base64(DEFAULT_LEN);
    };
    // `${jwt:N}` — opaque token of ~N bytes.
    if arg.chars().all(|c| c.is_ascii_digit()) && !arg.is_empty() {
        let n = arg.parse::<usize>().unwrap_or(DEFAULT_LEN);
        return random_base64(n);
    }
    // `${jwt:secret:payload}`.
    let (secret_ref, payload_ref) = match arg.split_once(':') {
        Some((secret, payload)) => (secret.trim(), payload.trim()),
        None => (arg.trim(), ""),
    };
    let secret = vars
        .get(secret_ref)
        .cloned()
        .unwrap_or_else(|| secret_ref.to_owned());
    let payload = vars
        .get(payload_ref)
        .cloned()
        .unwrap_or_else(|| payload_ref.to_owned());
    // Payload must be a JSON object; fall back to an empty object.
    let payload = if payload.trim_start().starts_with('{') {
        payload
    } else {
        "{}".to_owned()
    };
    hs256(&secret, &payload)
}

/// HS256 JWT: base64url(header).base64url(payload).base64url(HMAC-SHA256).
fn hs256(secret: &str, payload_json: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let header = base64url(br#"{"alg":"HS256","typ":"JWT"}"#);
    let payload = base64url(payload_json.as_bytes());
    let signing_input = format!("{header}.{payload}");

    let mut mac = <Hmac<Sha256>>::new_from_slice(secret.as_bytes())
        .expect("HMAC accepts any key length");
    mac.update(signing_input.as_bytes());
    let signature = base64url(&mac.finalize().into_bytes());
    format!("{signing_input}.{signature}")
}

fn base64url(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_map(env: &str) -> HashMap<&str, &str> {
        env.lines().filter_map(|line| line.split_once('=')).collect()
    }

    #[test]
    fn expand_env_template_generates_and_keeps_unknowns() {
        let out = expand_env_template("K=${password:24}");
        assert_eq!(out.strip_prefix("K=").unwrap().len(), 24);
        let keep = expand_env_template("U=${BB_OTHER}");
        assert_eq!(keep, "U=${BB_OTHER}");
    }

    const BUDIBASE_TOML: &str = r#"
[variables]
main_domain = "${domain}"
jwt_secret = "${password:32}"
couchdb_password = "${password:32}"
[config]
env = [
  "BB_HOST=${main_domain}",
  "BB_JWT_SECRET=${jwt_secret}",
  "BB_COUCHDB_PASSWORD=${couchdb_password}",
]
mounts = []
"#;

    #[test]
    fn resolves_array_env_and_variable_refs() {
        let out = resolve_template_toml(BUDIBASE_TOML, None).expect("resolved");
        let lines = env_map(&out.env);
        for key in ["BB_HOST", "BB_JWT_SECRET", "BB_COUCHDB_PASSWORD"] {
            let value = lines[key];
            assert!(!value.is_empty() && !value.contains("${"), "{key}={value}");
        }
        assert_eq!(lines["BB_JWT_SECRET"].len(), 32);
    }

    const ALARIK_TOML: &str = r#"
[variables]
console_domain = "${domain}"
api_domain = "${domain}"
[[config.domains]]
serviceName = "alarik"
port = 8_080
host = "${api_domain}"
[[config.domains]]
serviceName = "console"
port = 3_000
host = "${console_domain}"
[config.env]
API_BASE_URL = "http://${api_domain}"
ADMIN_PASSWORD = "${password:16}"
ALLOW_ACCOUNT_CREATION = "false"
"#;

    #[test]
    fn table_env_and_domains_resolve_with_integer_ports() {
        let out = resolve_template_toml(ALARIK_TOML, Some("1.2.3.4")).expect("resolved");
        let lines = env_map(&out.env);

        // Env table entries resolve their variable refs.
        assert!(lines["API_BASE_URL"].starts_with("http://"));
        assert!(!lines["API_BASE_URL"].contains("${"));
        assert_eq!(lines["ADMIN_PASSWORD"].len(), 16);

        // Ports parse as integers: 8_080 -> 8080, 3_000 -> 3000.
        let alarik = out.domains.iter().find(|d| d.service_name == "alarik").unwrap();
        assert_eq!(alarik.port, 8080);
        let console = out.domains.iter().find(|d| d.service_name == "console").unwrap();
        assert_eq!(console.port, 3000);

        // Hosts resolve to a reachable sslip.io host built from the server IP.
        assert!(alarik.host.ends_with("1-2-3-4.sslip.io"), "host={}", alarik.host);
        assert!(!alarik.host.contains("${"));
    }

    #[test]
    fn same_domain_ref_is_stable_within_a_resolution() {
        let toml = r#"
[variables]
d = "${domain}"
[config.env]
A = "${d}"
B = "${d}"
"#;
        let out = resolve_template_toml(toml, Some("9.9.9.9")).unwrap();
        let lines = env_map(&out.env);
        assert_eq!(lines["A"], lines["B"]);
    }

    #[test]
    fn two_domain_variables_resolve_to_distinct_hosts() {
        let toml = r#"
[variables]
api_domain = "${domain}"
console_domain = "${domain}"
[[config.domains]]
serviceName = "api"
port = 8000
host = "${api_domain}"
[[config.domains]]
serviceName = "console"
port = 3000
host = "${console_domain}"
"#;
        let out = resolve_template_toml(toml, Some("1.2.3.4")).unwrap();
        assert_eq!(out.domains.len(), 2);
        assert_ne!(out.domains[0].host, out.domains[1].host, "hosts must differ");
    }

    #[test]
    fn empty_toml_yields_none() {
        assert!(resolve_template_toml("[variables]\nx = \"${password}\"\n", None).is_none());
    }

    #[test]
    fn env_array_of_objects_is_supported() {
        let toml = r#"
[variables]
pw = "${password:12}"
[config]
env = [
  "PLAIN=value",
  { KEYED = "${pw}" },
]
"#;
        let out = resolve_template_toml(toml, None).unwrap();
        let lines = env_map(&out.env);
        assert_eq!(lines["PLAIN"], "value");
        assert_eq!(lines["KEYED"].len(), 12);
        assert!(!lines["KEYED"].contains("${"));
    }

    #[test]
    fn hash_generator_resolves_inside_a_value() {
        let toml = r#"
[variables]
prefix = "${hash:3}_akaunting"
[config.env]
APP_KEY = "${prefix}"
"#;
        let out = resolve_template_toml(toml, None).unwrap();
        let lines = env_map(&out.env);
        assert!(!lines["APP_KEY"].contains("${"), "unresolved: {}", lines["APP_KEY"]);
        assert!(lines["APP_KEY"].ends_with("_akaunting"));
    }

    #[test]
    fn supabase_jwt_mounts_and_date_timestamps_resolve() {
        let toml = r#"
[variables]
jwt_secret = "${password:32}"
anon_payload = """{ "role": "anon", "exp": ${timestamps:2030-01-01T00:00:00Z} }"""
anon_key = "${jwt:jwt_secret:anon_payload}"
[config]
env = [
  "JWT_SECRET=${jwt_secret}",
  "ANON_KEY=${anon_key}",
]
[[config.mounts]]
filePath = "/volumes/db/roles.sql"
content = "ALTER USER x WITH PASSWORD 'y';"
"#;
        let out = resolve_template_toml(toml, None).expect("resolved");
        let lines = env_map(&out.env);

        // JWT is a real 3-part HS256 token, fully resolved.
        let jwt = lines["ANON_KEY"];
        assert_eq!(jwt.split('.').count(), 3, "not a JWT: {jwt}");
        assert!(!jwt.contains("${"), "unresolved: {jwt}");

        // Mount was parsed and its content preserved.
        assert_eq!(out.mounts.len(), 1);
        assert_eq!(out.mounts[0].file_path, "/volumes/db/roles.sql");
        assert!(out.mounts[0].content.contains("ALTER USER"));
    }
}

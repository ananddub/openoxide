//! mTLS material for panel↔agent gRPC: one CA signs an agent server cert and
//! a panel client cert. The agent trusts the CA to verify the panel's client
//! cert (`client_ca_root`); the panel trusts the same CA to verify the agent's
//! server cert and presents its client cert during the handshake.

use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, ExtendedKeyUsagePurpose, IsCa,
    KeyPair, KeyUsagePurpose, SanType,
};

pub struct Pem {
    pub cert: String,
    pub key: String,
}

pub struct MtlsBundle {
    /// CA certificate PEM — trusted by both sides.
    pub ca_cert: String,
    /// Agent gRPC server identity (SANs cover its reachable names).
    pub agent_server: Pem,
    /// Panel gRPC client identity.
    pub panel_client: Pem,
}

fn distinguished(cn: &str) -> DistinguishedName {
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, cn);
    dn.push(DnType::OrganizationName, "OpenOxide");
    dn
}

/// A freshly generated CA: its cert + private key in PEM. Persist both so the
/// same CA can sign future leaf certs.
pub struct CaMaterial {
    pub cert_pem: String,
    pub key_pem: String,
}

/// Builds a standalone CA (cert + key). Store both; the key must never leave
/// the panel.
pub fn generate_ca() -> Result<CaMaterial, String> {
    let mut ca_params =
        CertificateParams::new(Vec::new()).map_err(|e| format!("ca params: {e}"))?;
    ca_params.distinguished_name = distinguished("OpenOxide mTLS CA");
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca_params.key_usages = vec![
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::CrlSign,
        KeyUsagePurpose::DigitalSignature,
    ];
    let ca_key = KeyPair::generate().map_err(|e| format!("ca key: {e}"))?;
    let ca = ca_params
        .self_signed(&ca_key)
        .map_err(|e| format!("ca sign: {e}"))?;
    Ok(CaMaterial {
        cert_pem: ca.pem(),
        key_pem: ca_key.serialize_pem(),
    })
}

/// Reloads a persisted CA and signs a fresh agent-server + panel-client set.
/// `agent_sans` are the names/IPs the panel uses to reach the agent.
pub fn sign_leaves(
    ca_cert_pem: &str,
    ca_key_pem: &str,
    agent_sans: &[String],
) -> Result<MtlsBundle, String> {
    let ca_key = KeyPair::from_pem(ca_key_pem).map_err(|e| format!("load ca key: {e}"))?;
    let ca_params = CertificateParams::from_ca_cert_pem(ca_cert_pem)
        .map_err(|e| format!("load ca cert: {e}"))?;
    let ca = ca_params
        .self_signed(&ca_key)
        .map_err(|e| format!("rebuild ca: {e}"))?;

    let mut server_params =
        CertificateParams::new(Vec::new()).map_err(|e| format!("server params: {e}"))?;
    server_params.distinguished_name = distinguished("openoxide-agent");
    server_params.subject_alt_names = san_list(agent_sans);
    server_params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyEncipherment,
    ];
    server_params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    let server_key = KeyPair::generate().map_err(|e| format!("server key: {e}"))?;
    let server_cert = server_params
        .signed_by(&server_key, &ca, &ca_key)
        .map_err(|e| format!("server sign: {e}"))?;

    let mut client_params =
        CertificateParams::new(Vec::new()).map_err(|e| format!("client params: {e}"))?;
    client_params.distinguished_name = distinguished("openoxide-panel");
    client_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    client_params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth];
    let client_key = KeyPair::generate().map_err(|e| format!("client key: {e}"))?;
    let client_cert = client_params
        .signed_by(&client_key, &ca, &ca_key)
        .map_err(|e| format!("client sign: {e}"))?;

    Ok(MtlsBundle {
        ca_cert: ca_cert_pem.to_string(),
        agent_server: Pem {
            cert: format!("{}{}", server_cert.pem(), ca_cert_pem),
            key: server_key.serialize_pem(),
        },
        panel_client: Pem {
            cert: format!("{}{}", client_cert.pem(), ca_cert_pem),
            key: client_key.serialize_pem(),
        },
    })
}

/// Builds a fresh CA + leaf set in one shot (CA not retained). Prefer
/// `generate_ca` + `sign_leaves` when the CA is persisted.
pub fn generate(agent_sans: &[String]) -> Result<MtlsBundle, String> {
    let ca = generate_ca()?;
    sign_leaves(&ca.cert_pem, &ca.key_pem, agent_sans)
}

/// A complete per-server identity set. Every field is unique to one server so a
/// leak is contained to that server. PEM strings.
#[derive(Clone)]
pub struct ServerMtls {
    pub ca_cert: String,
    pub ca_key: String,
    pub panel_cert: String,
    pub panel_key: String,
    pub agent_cert: String,
    pub agent_key: String,
}

/// Generates a brand-new per-server set: its own CA, plus panel-client and
/// agent-server identities signed by that CA. Called once per server.
pub fn generate_server_mtls(agent_sans: &[String]) -> Result<ServerMtls, String> {
    let ca = generate_ca()?;
    let bundle = sign_leaves(&ca.cert_pem, &ca.key_pem, agent_sans)?;
    Ok(ServerMtls {
        ca_cert: ca.cert_pem,
        ca_key: ca.key_pem,
        panel_cert: bundle.panel_client.cert,
        panel_key: bundle.panel_client.key,
        agent_cert: bundle.agent_server.cert,
        agent_key: bundle.agent_server.key,
    })
}

fn san_list(sans: &[String]) -> Vec<SanType> {
    let mut out = Vec::new();
    for value in sans {
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        match value.parse::<std::net::IpAddr>() {
            Ok(ip) => out.push(SanType::IpAddress(ip)),
            Err(_) => {
                if let Ok(dns) = value.to_owned().try_into() {
                    out.push(SanType::DnsName(dns));
                }
            }
        }
    }
    if out.is_empty()
        && let Ok(dns) = "openoxide-agent".to_owned().try_into()
    {
        out.push(SanType::DnsName(dns));
    }
    out
}

fn write_secret(path: &std::path::Path, contents: &str) -> Result<(), String> {
    std::fs::write(path, contents).map_err(|e| format!("write {}: {e}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

/// Per-server mTLS store. Each server (by DB id) owns an isolated CA plus its
/// panel-client and agent-server identities, all persisted as BLOB columns on
/// the `servers` row. Generated once (when NULL) and reused forever, so a
/// server's trust is stable across restarts and never shared with any other
/// server.
pub struct MtlsCaStore {
    queries: std::sync::Arc<crate::db::queries::FlashQueries>,
}

/// Paths of the per-server files written under `dir/server-{id}/`.
pub struct ServerMtlsPaths {
    pub dir: std::path::PathBuf,
    pub ca_cert: std::path::PathBuf,
    pub agent_cert: std::path::PathBuf,
    pub agent_key: std::path::PathBuf,
    pub panel_cert: std::path::PathBuf,
    pub panel_key: std::path::PathBuf,
}

#[auto_di::singleton]
impl MtlsCaStore {
    fn new(queries: std::sync::Arc<crate::db::queries::FlashQueries>) -> Self {
        Self { queries }
    }

    /// Returns the server's persisted mTLS set, generating and storing it once
    /// on first use. A concurrent first-boot writer is de-duped by the
    /// `WHERE ... IS NULL` guard and a re-read.
    pub async fn load_or_create(
        &self,
        server_id: i64,
        agent_sans: &[String],
    ) -> Result<ServerMtls, String> {
        if let Some(existing) = self.read(server_id).await? {
            return Ok(existing);
        }
        let fresh = generate_server_mtls(agent_sans)?;
        self.queries
            .provision_server_mtls(&crate::flash_gen::servers::ProvisionServerMtlsParams {
                mtls_ca_cert: fresh.ca_cert.into_bytes(),
                mtls_ca_key: fresh.ca_key.into_bytes(),
                mtls_panel_cert: fresh.panel_cert.into_bytes(),
                mtls_panel_key: fresh.panel_key.into_bytes(),
                mtls_agent_cert: fresh.agent_cert.into_bytes(),
                mtls_agent_key: fresh.agent_key.into_bytes(),
                id: server_id,
            })
            .await
            .map_err(|e| format!("persist server {server_id} mtls: {e}"))?;
        self.read(server_id)
            .await?
            .ok_or_else(|| format!("server {server_id} not found while persisting mtls"))
    }

    async fn read(&self, server_id: i64) -> Result<Option<ServerMtls>, String> {
        let row = match self.queries.get_server_mtls(server_id).await {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(e) => return Err(format!("read server {server_id} mtls: {e}")),
        };
        let Some(ca_cert) = row.mtls_ca_cert else {
            return Ok(None);
        };
        let utf8 = |name: &str, b: Option<Vec<u8>>| -> Result<String, String> {
            let bytes = b.ok_or_else(|| format!("{name} missing"))?;
            String::from_utf8(bytes).map_err(|e| format!("{name} not utf8: {e}"))
        };
        Ok(Some(ServerMtls {
            ca_cert: String::from_utf8(ca_cert)
                .map_err(|e| format!("mtls_ca_cert not utf8: {e}"))?,
            ca_key: utf8("mtls_ca_key", row.mtls_ca_key)?,
            panel_cert: utf8("mtls_panel_cert", row.mtls_panel_cert)?,
            panel_key: utf8("mtls_panel_key", row.mtls_panel_key)?,
            agent_cert: utf8("mtls_agent_cert", row.mtls_agent_cert)?,
            agent_key: utf8("mtls_agent_key", row.mtls_agent_key)?,
        }))
    }

    /// Writes the server's cert files under `base_dir/server-{id}/` (0600) for
    /// mounting/distribution and returns their paths.
    pub fn write_files(
        &self,
        server_id: i64,
        base_dir: &std::path::Path,
        material: &ServerMtls,
    ) -> Result<ServerMtlsPaths, String> {
        let dir = base_dir.join(format!("server-{server_id}"));
        std::fs::create_dir_all(&dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
        let paths = ServerMtlsPaths {
            ca_cert: dir.join("ca.crt"),
            agent_cert: dir.join("agent.crt"),
            agent_key: dir.join("agent.key"),
            panel_cert: dir.join("panel-client.crt"),
            panel_key: dir.join("panel-client.key"),
            dir,
        };
        write_secret(&paths.ca_cert, &material.ca_cert)?;
        write_secret(&paths.agent_cert, &material.agent_cert)?;
        write_secret(&paths.agent_key, &material.agent_key)?;
        write_secret(&paths.panel_cert, &material.panel_cert)?;
        write_secret(&paths.panel_key, &material.panel_key)?;
        Ok(paths)
    }

    /// Load-or-create the server's material and write its files in one call.
    pub async fn ensure_server(
        &self,
        server_id: i64,
        base_dir: &std::path::Path,
        agent_sans: &[String],
    ) -> Result<ServerMtlsPaths, String> {
        let material = self.load_or_create(server_id, agent_sans).await?;
        self.write_files(server_id, base_dir, &material)
    }

    /// Deletes `server-{id}/` from `OPENOXIDE_MTLS_DIR` after the server row
    /// is removed — without this the private key material for deleted servers
    /// lingers on disk forever.
    pub fn remove_server_files(&self, server_id: i64) {
        let base_dir =
            std::env::var("OPENOXIDE_MTLS_DIR").unwrap_or_else(|_| "/app/data/mtls".to_string());
        let dir = std::path::Path::new(&base_dir).join(format!("server-{server_id}"));
        if dir.exists()
            && let Err(error) = std::fs::remove_dir_all(&dir)
        {
            tracing::warn!(%error, path = %dir.display(), "could not remove server mtls files");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_a_ca_signed_bundle() {
        let bundle = generate(&["openoxide-agent".into(), "127.0.0.1".into()]).unwrap();
        assert!(bundle.ca_cert.contains("BEGIN CERTIFICATE"));
        assert!(bundle.agent_server.cert.contains("BEGIN CERTIFICATE"));
        assert!(bundle.agent_server.key.contains("BEGIN PRIVATE KEY"));
        assert!(bundle.panel_client.cert.contains("BEGIN CERTIFICATE"));
        assert!(bundle.panel_client.key.contains("BEGIN PRIVATE KEY"));
        // Leaf + CA chain: two certificate blocks in the agent server PEM.
        assert_eq!(
            bundle
                .agent_server
                .cert
                .matches("BEGIN CERTIFICATE")
                .count(),
            2
        );
    }

    #[test]
    fn per_server_material_is_isolated() {
        use rcgen::{CertificateParams, KeyPair};
        // Two servers → two independent CAs.
        let a = generate_server_mtls(&["10.0.0.1".into()]).unwrap();
        let b = generate_server_mtls(&["10.0.0.2".into()]).unwrap();
        assert_ne!(a.ca_cert, b.ca_cert, "each server must get its own CA");
        assert_ne!(a.ca_key, b.ca_key);
        assert_ne!(
            a.panel_key, b.panel_key,
            "each server has its own panel key"
        );
        assert_ne!(
            a.agent_key, b.agent_key,
            "each server has its own agent key"
        );
        // Every field is real PEM.
        for pem in [
            &a.ca_cert,
            &a.ca_key,
            &a.panel_cert,
            &a.panel_key,
            &a.agent_cert,
            &a.agent_key,
        ] {
            assert!(pem.contains("BEGIN"));
        }
        // Server A's panel cert chains to A's CA, not B's (isolation): the
        // chain embeds A's CA, and A's CA reloads only with A's key.
        assert!(a.panel_cert.ends_with(&a.ca_cert));
        assert!(!a.panel_cert.ends_with(&b.ca_cert));
        // B's CA cannot be reconstructed with A's CA key (different keypair).
        let params_b = CertificateParams::from_ca_cert_pem(&b.ca_cert).unwrap();
        let key_a = KeyPair::from_pem(&a.ca_key).unwrap();
        let rebuilt = params_b.self_signed(&key_a).unwrap();
        assert_ne!(
            rebuilt.pem(),
            b.ca_cert,
            "B's CA must not be forgeable with A's key"
        );
    }

    #[test]
    fn one_ca_signs_repeated_leaf_bundles() {
        let ca = generate_ca().unwrap();
        assert!(ca.cert_pem.contains("BEGIN CERTIFICATE"));
        assert!(ca.key_pem.contains("BEGIN PRIVATE KEY"));
        let first = sign_leaves(&ca.cert_pem, &ca.key_pem, &["openoxide-agent".into()]).unwrap();
        let second = sign_leaves(&ca.cert_pem, &ca.key_pem, &["openoxide-agent".into()]).unwrap();
        // Same persisted CA is embedded in both bundles' chains.
        assert_eq!(first.ca_cert, ca.cert_pem);
        assert_eq!(second.ca_cert, ca.cert_pem);
        assert!(first.agent_server.cert.ends_with(&ca.cert_pem));
        assert!(second.panel_client.cert.ends_with(&ca.cert_pem));
        // Fresh leaf keys each call.
        assert_ne!(first.agent_server.key, second.agent_server.key);
    }
}

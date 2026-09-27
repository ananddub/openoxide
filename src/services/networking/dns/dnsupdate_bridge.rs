use super::{
    DnsProviderType, DnsRecordDto, DnsRecordTargetDto, DnsService, DnsServiceError,
    DnsTestResultDto, DnsZoneDto, UpdateDnsRecordDto, UpsertDnsRecordDto,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use dns_update::providers::{
    azuredns::AzureEnvironment, joker::JokerAuth, oraclecloud::OracleCloudConfig, ovh::OvhEndpoint,
};
use dns_update::{
    CAARecord, DnsRecord, DnsRecordType, DnsUpdater, MXRecord, SRVRecord, TsigAlgorithm, ZoneRecord,
};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};

fn err(message: impl Into<String>) -> DnsServiceError {
    DnsServiceError::ProviderError(message.into())
}

impl From<dns_update::Error> for DnsServiceError {
    fn from(error: dns_update::Error) -> Self {
        err(error.to_string())
    }
}

fn cred_str(obj: &Map<String, Value>, key: &str) -> String {
    obj.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn cred_opt(obj: &Map<String, Value>, key: &str) -> Option<String> {
    let value = cred_str(obj, key);
    (!value.is_empty()).then_some(value)
}

fn cred_bool(obj: &Map<String, Value>, key: &str) -> bool {
    match obj.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => {
            matches!(s.trim().to_ascii_lowercase().as_str(), "true" | "1" | "yes")
        }
        _ => false,
    }
}

fn cred_u64(obj: &Map<String, Value>, key: &str) -> Option<u64> {
    obj.get(key).and_then(|v| {
        v.as_u64()
            .or_else(|| v.as_str().and_then(|s| s.trim().parse::<u64>().ok()))
    })
}

fn required(obj: &Map<String, Value>, key: &str) -> Result<String, DnsServiceError> {
    cred_opt(obj, key).ok_or_else(|| {
        err(format!(
            "DNS credentials field '{key}' is required for this provider"
        ))
    })
}

fn parse_credentials(credentials_json: &str) -> Result<Map<String, Value>, DnsServiceError> {
    serde_json::from_str::<Value>(credentials_json)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .ok_or_else(|| err("Invalid DNS credentials JSON object"))
}

/// Builds the dns-update engine for every provider that is not hand-wired in
/// `providers/`. Credential JSON keys are camelCase and mirror the web catalog.
fn build_updater(
    provider_type: DnsProviderType,
    credentials_json: &str,
) -> Result<DnsUpdater, DnsServiceError> {
    let c = parse_credentials(credentials_json)?;
    let updater = match provider_type {
        DnsProviderType::Desec => DnsUpdater::new_desec(required(&c, "apiToken")?, None)?,
        DnsProviderType::Bunny => DnsUpdater::new_bunny(required(&c, "apiKey")?, None)?,
        DnsProviderType::Constellix => {
            DnsUpdater::new_constellix(required(&c, "apiKey")?, required(&c, "secretKey")?, None)?
        }
        DnsProviderType::DnsMadeEasy => {
            DnsUpdater::new_dnsmadeeasy(required(&c, "apiKey")?, required(&c, "apiSecret")?, None)?
        }
        DnsProviderType::Exoscale => {
            DnsUpdater::new_exoscale(required(&c, "apiKey")?, required(&c, "apiSecret")?, None)?
        }
        DnsProviderType::Nifcloud => {
            DnsUpdater::new_nifcloud(required(&c, "accessKey")?, required(&c, "secretKey")?, None)?
        }
        DnsProviderType::WebSupport => {
            DnsUpdater::new_websupport(required(&c, "apiKey")?, required(&c, "apiSecret")?, None)?
        }
        DnsProviderType::Spaceship => {
            DnsUpdater::new_spaceship(required(&c, "apiKey")?, required(&c, "apiSecret")?, None)?
        }
        DnsProviderType::Dnsimple => {
            DnsUpdater::new_dnsimple(required(&c, "apiToken")?, required(&c, "accountId")?, None)?
        }
        DnsProviderType::GandiV5 => DnsUpdater::new_gandiv5(required(&c, "apiToken")?, None)?,
        DnsProviderType::NameDotCom => {
            DnsUpdater::new_namedotcom(required(&c, "username")?, required(&c, "apiToken")?, None)?
        }
        DnsProviderType::NameSilo => DnsUpdater::new_namesilo(required(&c, "apiKey")?, None)?,
        DnsProviderType::DuckDns => DnsUpdater::new_duckdns(required(&c, "token")?, None)?,
        DnsProviderType::FreeMyIp => DnsUpdater::new_freemyip(required(&c, "token")?, None)?,
        DnsProviderType::Ipv64 => DnsUpdater::new_ipv64(required(&c, "apiKey")?, None)?,
        DnsProviderType::Ddnss => DnsUpdater::new_ddnss(required(&c, "key")?, None)?,
        DnsProviderType::Dynu => DnsUpdater::new_dynu(required(&c, "apiKey")?, None)?,
        DnsProviderType::Plesk => {
            DnsUpdater::new_plesk(required(&c, "baseUrl")?, required(&c, "apiKey")?, None)?
        }
        DnsProviderType::Cpanel => DnsUpdater::new_cpanel(
            required(&c, "baseUrl")?,
            required(&c, "username")?,
            required(&c, "apiToken")?,
            None,
        )?,
        DnsProviderType::GoogleCloudDns => {
            let mut config = dns_update::providers::google_cloud_dns::GoogleCloudDnsConfig {
                service_account_json: required(&c, "serviceAccountJson")?,
                project_id: required(&c, "projectId")?,
                managed_zone: cred_opt(&c, "managedZone"),
                private_zone: cred_bool(&c, "privateZone"),
                impersonate_service_account: cred_opt(&c, "impersonateServiceAccount"),
                request_timeout: None,
            };
            let _ = &mut config;
            DnsUpdater::new_google_cloud_dns(config)?
        }
        DnsProviderType::Ionos => DnsUpdater::new_ionos(required(&c, "apiKey")?, None)?,
        DnsProviderType::HostingDe => DnsUpdater::new_hostingde(required(&c, "apiKey")?, None)?,
        DnsProviderType::Netcup => DnsUpdater::new_netcup(
            required(&c, "customerNumber")?,
            required(&c, "apiKey")?,
            required(&c, "apiPassword")?,
            None,
        )?,
        DnsProviderType::Netlify => DnsUpdater::new_netlify(required(&c, "accessToken")?, None)?,
        DnsProviderType::HuaweiCloud => DnsUpdater::new_huaweicloud(
            required(&c, "accessKey")?,
            required(&c, "secretKey")?,
            cred_opt(&c, "region").unwrap_or_else(|| "cn-north-4".to_string()),
            None,
        )?,
        DnsProviderType::BaiduCloud => DnsUpdater::new_baiducloud(
            required(&c, "accessKey")?,
            required(&c, "secretKey")?,
            None,
        )?,
        DnsProviderType::AzureDns => {
            let config = dns_update::providers::azuredns::AzureDnsConfig {
                tenant_id: required(&c, "tenantId")?,
                client_id: required(&c, "clientId")?,
                client_secret: required(&c, "clientSecret")?,
                subscription_id: required(&c, "subscriptionId")?,
                resource_group: required(&c, "resourceGroup")?,
                environment: match cred_opt(&c, "environment") {
                    Some(env) => AzureEnvironment::from_str_lossy(&env),
                    None => AzureEnvironment::Public,
                },
                request_timeout: None,
            };
            DnsUpdater::new_azuredns(config)?
        }
        DnsProviderType::IbmCloud => {
            DnsUpdater::new_ibmcloud(required(&c, "username")?, required(&c, "apiKey")?, None)?
        }
        DnsProviderType::Hurricane => {
            let mut credentials = HashMap::new();
            for (key, value) in &c {
                if key != "zone"
                    && key != "token"
                    && let Some(v) = value.as_str()
                {
                    credentials.insert(key.clone(), v.to_string());
                }
            }
            // Flat "zone" + "token" form from the panel form: map token by zone name.
            if let (Some(zone), Some(token)) = (cred_opt(&c, "zone"), cred_opt(&c, "token")) {
                credentials.insert(zone, token);
            }
            DnsUpdater::new_hurricane(credentials, None)?
        }
        DnsProviderType::Hostinger => DnsUpdater::new_hostinger(required(&c, "apiToken")?, None)?,
        DnsProviderType::Hetzner => DnsUpdater::new_hetzner(required(&c, "apiToken")?, None)?,
        DnsProviderType::DigitalOcean => {
            DnsUpdater::new_digitalocean(required(&c, "apiToken")?, None)?
        }
        DnsProviderType::Infomaniak => DnsUpdater::new_infomaniak(required(&c, "apiToken")?, None)?,
        DnsProviderType::Godaddy => {
            DnsUpdater::new_godaddy(required(&c, "apiKey")?, required(&c, "apiSecret")?, None)?
        }
        DnsProviderType::Porkbun => {
            DnsUpdater::new_porkbun(required(&c, "apiKey")?, required(&c, "secretApiKey")?, None)?
        }
        DnsProviderType::Ovh => {
            let endpoint = cred_opt(&c, "endpoint").unwrap_or_else(|| "ovh-eu".to_string());
            let endpoint: OvhEndpoint = endpoint
                .parse()
                .map_err(|_| err(format!("OVH: unknown endpoint '{endpoint}'")))?;
            DnsUpdater::new_ovh(
                required(&c, "applicationKey")?,
                required(&c, "applicationSecret")?,
                required(&c, "consumerKey")?,
                endpoint,
                None,
            )?
        }
        DnsProviderType::Autodns => DnsUpdater::new_autodns(
            required(&c, "username")?,
            required(&c, "password")?,
            cred_u64(&c, "context").map(|v| v as u32),
            None,
        )?,
        DnsProviderType::Lightsail => {
            let config = dns_update::providers::lightsail::LightsailConfig {
                access_key_id: required(&c, "accessKeyId")?,
                secret_access_key: required(&c, "secretAccessKey")?,
                session_token: cred_opt(&c, "sessionToken"),
                region: cred_opt(&c, "region"),
                domain: cred_opt(&c, "domain"),
                request_timeout: None,
            };
            DnsUpdater::new_lightsail(config)?
        }
        DnsProviderType::EdgeDns => {
            let config = dns_update::providers::edgedns::EdgeDnsConfig {
                host: required(&c, "host")?,
                client_token: required(&c, "clientToken")?,
                client_secret: required(&c, "clientSecret")?,
                access_token: required(&c, "accessToken")?,
                account_switch_key: cred_opt(&c, "accountSwitchKey"),
                request_timeout: None,
            };
            DnsUpdater::new_edgedns(config)?
        }
        DnsProviderType::Scaleway => DnsUpdater::new_scaleway(required(&c, "apiToken")?, None)?,
        DnsProviderType::Gcore => DnsUpdater::new_gcore(required(&c, "apiToken")?, None)?,
        DnsProviderType::Vercel => {
            DnsUpdater::new_vercel(required(&c, "authToken")?, cred_opt(&c, "teamId"), None)?
        }
        DnsProviderType::Vultr => DnsUpdater::new_vultr(required(&c, "apiKey")?, None)?,
        DnsProviderType::BluecatV2 => {
            let config = dns_update::providers::bluecatv2::BluecatV2Config {
                server_url: required(&c, "serverUrl")?,
                username: required(&c, "username")?,
                password: required(&c, "password")?,
                config_name: required(&c, "configName")?,
                view_name: cred_opt(&c, "viewName").unwrap_or_else(|| "default".to_string()),
                skip_deploy: cred_bool(&c, "skipDeploy"),
                request_timeout: None,
            };
            DnsUpdater::new_bluecatv2(config)?
        }
        DnsProviderType::EasyDns => {
            DnsUpdater::new_easydns(required(&c, "token")?, required(&c, "key")?, None)?
        }
        DnsProviderType::Joker => {
            let auth = match (cred_opt(&c, "apiKey"), cred_opt(&c, "username")) {
                (Some(api_key), _) => JokerAuth::ApiKey(api_key),
                (_, Some(username)) => JokerAuth::UsernamePassword {
                    username,
                    password: required(&c, "password")?,
                },
                (None, None) => {
                    return Err(err(
                        "DNS credentials require 'apiKey' or 'username' + 'password'",
                    ));
                }
            };
            DnsUpdater::new_joker(auth, None)?
        }
        DnsProviderType::MythicBeasts => DnsUpdater::new_mythicbeasts(
            required(&c, "username")?,
            required(&c, "password")?,
            None,
        )?,
        DnsProviderType::Namecheap => DnsUpdater::new_namecheap(
            required(&c, "apiUser")?,
            required(&c, "apiKey")?,
            required(&c, "clientIp")?,
            cred_opt(&c, "username"),
            None,
        )?,
        DnsProviderType::Transip => DnsUpdater::new_transip(
            required(&c, "login")?,
            required(&c, "privateKeyPem")?,
            cred_bool(&c, "globalKey"),
            None,
        )?,
        DnsProviderType::Ns1 => DnsUpdater::new_ns1(required(&c, "apiKey")?, None)?,
        DnsProviderType::LuaDns => DnsUpdater::new_luadns(
            required(&c, "apiUsername")?,
            required(&c, "apiToken")?,
            None,
        )?,
        DnsProviderType::ClouDns => DnsUpdater::new_cloudns(
            cred_opt(&c, "authId"),
            cred_opt(&c, "subAuthId"),
            required(&c, "authPassword")?,
            None,
        )?,
        DnsProviderType::Glesys => {
            DnsUpdater::new_glesys(required(&c, "apiUser")?, required(&c, "apiKey")?, None)?
        }
        DnsProviderType::Dreamhost => DnsUpdater::new_dreamhost(required(&c, "apiKey")?, None)?,
        DnsProviderType::Alidns => DnsUpdater::new_alidns(
            required(&c, "accessKey")?,
            required(&c, "secretKey")?,
            cred_opt(&c, "region"),
            cred_opt(&c, "securityToken"),
            cred_opt(&c, "line"),
            None,
        )?,
        DnsProviderType::TencentCloud => DnsUpdater::new_tencentcloud(
            required(&c, "secretId")?,
            required(&c, "secretKey")?,
            cred_opt(&c, "region"),
            cred_opt(&c, "sessionToken"),
            None,
        )?,
        DnsProviderType::Volcengine => {
            let config = dns_update::providers::volcengine::VolcengineConfig {
                access_key: required(&c, "accessKey")?,
                secret_key: required(&c, "secretKey")?,
                region: cred_opt(&c, "region"),
                host: cred_opt(&c, "host"),
                scheme: cred_opt(&c, "scheme"),
                request_timeout: None,
            };
            DnsUpdater::new_volcengine(config)?
        }
        DnsProviderType::YandexCloud => {
            let config = dns_update::providers::yandexcloud::YandexCloudConfig {
                iam_token_b64: required(&c, "iamTokenB64")?,
                folder_id: required(&c, "folderId")?,
                request_timeout: None,
            };
            DnsUpdater::new_yandexcloud(config)?
        }
        DnsProviderType::Domeneshop => {
            DnsUpdater::new_domeneshop(required(&c, "apiToken")?, required(&c, "apiSecret")?, None)?
        }
        DnsProviderType::Safedns => DnsUpdater::new_safedns(required(&c, "apiToken")?, None)?,
        DnsProviderType::SimplyCom => {
            DnsUpdater::new_simplycom(required(&c, "accountName")?, required(&c, "apiKey")?, None)?
        }
        DnsProviderType::ArvanCloud => DnsUpdater::new_arvancloud(required(&c, "apiKey")?, None)?,
        DnsProviderType::Inwx => DnsUpdater::new_inwx(
            required(&c, "username")?,
            required(&c, "password")?,
            cred_bool(&c, "sandbox"),
            None,
        )?,
        DnsProviderType::UltraDns => DnsUpdater::new_ultradns(
            required(&c, "username")?,
            required(&c, "password")?,
            cred_opt(&c, "endpoint"),
            None,
        )?,
        DnsProviderType::Infoblox => {
            let config = dns_update::providers::infoblox::InfobloxConfig {
                host: required(&c, "host")?,
                port: cred_opt(&c, "port"),
                username: required(&c, "username")?,
                password: required(&c, "password")?,
                wapi_version: cred_opt(&c, "wapiVersion"),
                dns_view: cred_opt(&c, "dnsView"),
                request_timeout: None,
            };
            DnsUpdater::new_infoblox(config)?
        }
        DnsProviderType::OracleCloud => {
            let config = OracleCloudConfig {
                tenancy_ocid: required(&c, "tenancyOcid")?,
                user_ocid: required(&c, "userOcid")?,
                fingerprint: required(&c, "fingerprint")?,
                private_key_pem: required(&c, "privateKeyPem")?,
                private_key_password: cred_opt(&c, "privateKeyPassword"),
                region: required(&c, "region")?,
                compartment_ocid: required(&c, "compartmentOcid")?,
                request_timeout: None,
            };
            DnsUpdater::new_oraclecloud(config)?
        }
        DnsProviderType::Rfc2136 => {
            let host = required(&c, "server")?;
            let addr = match cred_opt(&c, "port") {
                Some(port) => format!("{host}:{port}"),
                None => host,
            };
            let key = BASE64
                .decode(required(&c, "key")?.replace(['\n', '\r', ' '], ""))
                .map_err(|e| err(format!("RFC2136 'key' must be base64: {e}")))?;
            let algorithm = match cred_opt(&c, "algorithm").as_deref() {
                Some("hmac-md5") => TsigAlgorithm::HmacMd5,
                Some("hmac-sha1") => TsigAlgorithm::HmacSha1,
                Some("hmac-sha224") => TsigAlgorithm::HmacSha224,
                Some("hmac-sha384") => TsigAlgorithm::HmacSha384,
                Some("hmac-sha512") => TsigAlgorithm::HmacSha512,
                Some("gss") => TsigAlgorithm::Gss,
                Some(other) => {
                    return Err(err(format!("unknown RFC2136 TSIG algorithm '{other}'")));
                }
                None => TsigAlgorithm::HmacSha256,
            };
            DnsUpdater::new_rfc2136_tsig(addr.as_str(), required(&c, "keyName")?, key, algorithm)?
        }
        // Hand-wired providers never reach the bridge.
        _ => {
            return Err(err(
                "DNS provider is not routed through the dns-update bridge",
            ));
        }
    };
    Ok(updater)
}

fn parse_record_type(value: &str) -> Result<DnsRecordType, DnsServiceError> {
    Ok(match value.trim().to_ascii_uppercase().as_str() {
        "A" => DnsRecordType::A,
        "AAAA" => DnsRecordType::AAAA,
        "CNAME" => DnsRecordType::CNAME,
        "NS" => DnsRecordType::NS,
        "MX" => DnsRecordType::MX,
        "TXT" => DnsRecordType::TXT,
        "SRV" => DnsRecordType::SRV,
        "CAA" => DnsRecordType::CAA,
        other => return Err(err(format!("record type '{other}' is not supported"))),
    })
}

fn parse_record(record_type: DnsRecordType, content: &str) -> Result<DnsRecord, DnsServiceError> {
    let content = content.trim();
    Ok(match record_type {
        DnsRecordType::A => DnsRecord::A(
            content
                .parse::<Ipv4Addr>()
                .map_err(|_| err(format!("'{content}' is not a valid IPv4 address")))?,
        ),
        DnsRecordType::AAAA => DnsRecord::AAAA(
            content
                .parse::<Ipv6Addr>()
                .map_err(|_| err(format!("'{content}' is not a valid IPv6 address")))?,
        ),
        DnsRecordType::CNAME => DnsRecord::CNAME(content.to_string()),
        DnsRecordType::NS => DnsRecord::NS(content.to_string()),
        DnsRecordType::TXT => DnsRecord::TXT(content.to_string()),
        DnsRecordType::MX => {
            let (priority, exchange) = content
                .split_once(char::is_whitespace)
                .ok_or_else(|| err("MX content must be '<priority> <exchange>'"))?;
            DnsRecord::MX(MXRecord {
                exchange: exchange.trim().to_string(),
                priority: priority
                    .trim()
                    .parse()
                    .map_err(|_| err("MX priority must be a number"))?,
            })
        }
        DnsRecordType::SRV => {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() != 4 {
                return Err(err(
                    "SRV content must be '<priority> <weight> <port> <target>'",
                ));
            }
            DnsRecord::SRV(SRVRecord {
                target: parts[3].to_string(),
                priority: parts[0]
                    .parse()
                    .map_err(|_| err("SRV priority must be a number"))?,
                weight: parts[1]
                    .parse()
                    .map_err(|_| err("SRV weight must be a number"))?,
                port: parts[2]
                    .parse()
                    .map_err(|_| err("SRV port must be a number"))?,
            })
        }
        DnsRecordType::CAA => {
            let parts: Vec<&str> = content.splitn(3, char::is_whitespace).collect();
            if parts.len() < 2 {
                return Err(err("CAA content must be '<flags> <tag> [value]'"));
            }
            let flags: u8 = parts[0].trim().parse().unwrap_or(0);
            let value = parts.get(2).map(|v| v.trim().trim_matches('"').to_string());
            match parts[1].trim() {
                "issuewild" => DnsRecord::CAA(CAARecord::IssueWild {
                    issuer_critical: flags & 1 != 0,
                    name: value,
                    options: Vec::new(),
                }),
                "iodef" => DnsRecord::CAA(CAARecord::Iodef {
                    issuer_critical: flags & 1 != 0,
                    url: value.unwrap_or_default(),
                }),
                _ => DnsRecord::CAA(CAARecord::Issue {
                    issuer_critical: flags & 1 != 0,
                    name: value,
                    options: Vec::new(),
                }),
            }
        }
        DnsRecordType::TLSA => return Err(err("TLSA records are not supported")),
    })
}

fn to_fqdn(name: &str, zone: &str) -> String {
    let zone = zone.trim().trim_end_matches('.');
    match name.trim().trim_end_matches('.') {
        "" | "@" | "." => zone.to_string(),
        other if other.eq_ignore_ascii_case(zone) => other.to_string(),
        other if other.to_lowercase().ends_with(&zone.to_lowercase()) => other.to_string(),
        other => format!("{other}.{zone}"),
    }
}

fn zone_of(credentials: &Map<String, Value>, explicit: &str) -> Option<String> {
    let zone = if explicit.trim().is_empty() {
        cred_str(credentials, "zone")
    } else {
        explicit.trim().to_string()
    };
    (!zone.is_empty()).then_some(zone)
}

impl DnsService {
    pub(super) async fn bridge_test_credentials(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let updater = build_updater(provider_type, credentials_json)?;
        let credentials = parse_credentials(credentials_json)?;
        match updater.list_zones().await {
            Ok(zones) => Ok(DnsTestResultDto {
                success: true,
                message: format!("Connection successful — {} zone(s) accessible", zones.len()),
            }),
            Err(dns_update::Error::Unsupported(_)) => {
                match zone_of(&credentials, "").map(|zone| {
                    let updater = &updater;
                    async move { updater.list_zone_records(&zone).await.map(|r| r.len()) }
                }) {
                    Some(fut) => match fut.await {
                        Ok(count) => Ok(DnsTestResultDto {
                            success: true,
                            message: format!("Connection successful — {count} record(s) in zone"),
                        }),
                        Err(error) => Ok(DnsTestResultDto {
                            success: false,
                            message: error.to_string(),
                        }),
                    },
                    None => Ok(DnsTestResultDto {
                        success: true,
                        message: "Credentials accepted — this provider does not list zones; set the 'zone' credential to manage records".to_string(),
                    }),
                }
            }
            Err(error) => Ok(DnsTestResultDto {
                success: false,
                message: error.to_string(),
            }),
        }
    }

    pub(super) async fn bridge_list_zones(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let updater = build_updater(provider_type, credentials_json)?;
        let credentials = parse_credentials(credentials_json)?;
        match updater.list_zones().await {
            Ok(zones) => Ok(zones
                .into_iter()
                .map(|z| DnsZoneDto {
                    id: z.name.clone(),
                    name: z.name,
                    status: z.status,
                })
                .collect()),
            Err(dns_update::Error::Unsupported(_)) => match zone_of(&credentials, "") {
                Some(zone) => Ok(vec![DnsZoneDto {
                    id: zone.clone(),
                    name: zone,
                    status: None,
                }]),
                None => Err(err(
                    "This provider cannot list zones — edit the provider and set the 'zone' credential to the domain you manage",
                )),
            },
            Err(error) => Err(DnsServiceError::ProviderError(error.to_string())),
        }
    }

    pub(super) async fn bridge_list_records(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let updater = build_updater(provider_type, credentials_json)?;
        let records = updater
            .list_zone_records(zone_id)
            .await
            .map_err(|e| err(e.to_string()))?;
        Ok(records
            .into_iter()
            .map(|r| DnsRecordDto {
                id: r.id,
                zone_id: zone_id.to_string(),
                record_type: r.record_type,
                name: r.name,
                content: r.content,
                ttl: (r.ttl > 0).then_some(r.ttl),
                proxied: None,
            })
            .collect())
    }

    pub(super) async fn bridge_upsert_record(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let updater = build_updater(provider_type, credentials_json)?;
        let record_type = parse_record_type(&body.record_type)?;
        let record = parse_record(record_type, &body.content)?;
        let ttl = body.ttl.filter(|t| *t > 0).unwrap_or(3600);
        let name = to_fqdn(&body.name, &body.zone_id);
        // Append, never replace: an rrset already holds sibling values (a second A record,
        // another MX preference) and overwriting the set would silently retire them.
        updater
            .add_to_rrset(name, record_type, ttl, vec![record], body.zone_id.as_str())
            .await
            .map_err(|e| err(e.to_string()))?;
        Ok(DnsRecordDto {
            // No provider id comes back from a create; the panel refetches the list, and
            // inventing one here would point the next delete at a row that never existed.
            id: String::new(),
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name: body.name.clone(),
            content: body.content.clone(),
            ttl: Some(ttl),
            proxied: None,
        })
    }

    pub(super) async fn bridge_update_record(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
        body: &UpdateDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let updater = build_updater(provider_type, credentials_json)?;
        let zone = body.target.zone_id.trim().to_string();
        let rows = updater
            .list_zone_records(&zone)
            .await
            .map_err(|e| err(e.to_string()))?;
        let old = pick_bridge_row(&rows, &body.target, &zone)?;

        let next_type = parse_record_type(&body.next.record_type)?;
        let next_record = parse_record(next_type, &body.next.content)?;
        let old_type = parse_record_type(&old.record_type)?;
        let ttl = body
            .next
            .ttl
            .filter(|t| *t > 0)
            .or_else(|| (old.ttl > 0).then_some(old.ttl))
            .unwrap_or(3600);

        updater
            .remove_from_rrset(to_fqdn(&old.name, &zone), old_type, vec![old.record], &zone)
            .await
            .map_err(|e| err(e.to_string()))?;
        updater
            .add_to_rrset(
                to_fqdn(&body.next.name, &zone),
                next_type,
                ttl,
                vec![next_record],
                &zone,
            )
            .await
            .map_err(|e| err(e.to_string()))?;

        // The row now carries a fresh provider id; without it the next delete would aim at
        // a record that no longer exists.
        let after = updater
            .list_zone_records(&zone)
            .await
            .map_err(|e| err(e.to_string()))?;
        let untouched = old.content == body.next.content
            && old.record_type.eq_ignore_ascii_case(&body.next.record_type)
            && to_fqdn(&old.name, &zone) == to_fqdn(&body.next.name, &zone);
        if !untouched && still_listed(&after, &old.name, &old.record_type, &old.content, &zone) {
            return Err(err(format!(
                "the provider still lists {} '{}' after the update — the old value was not replaced",
                old.record_type, old.name
            )));
        }
        let id = after
            .into_iter()
            .find(|row| {
                row.content == body.next.content && row.record_type == body.next.record_type
            })
            .map(|row| row.id)
            .unwrap_or_default();
        Ok(DnsRecordDto {
            id,
            zone_id: zone.clone(),
            record_type: body.next.record_type.clone(),
            name: body.next.name.clone(),
            content: body.next.content.clone(),
            ttl: Some(ttl),
            proxied: None,
        })
    }

    pub(super) async fn bridge_delete_record(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
        target: &DnsRecordTargetDto,
    ) -> Result<bool, DnsServiceError> {
        let updater = build_updater(provider_type, credentials_json)?;
        let zone = target.zone_id.trim().to_string();
        let rows = updater
            .list_zone_records(&zone)
            .await
            .map_err(|e| err(e.to_string()))?;
        let row = pick_bridge_row(&rows, target, &zone)?;
        let record_type = parse_record_type(&row.record_type)?;
        let (name, listed_type, content) = (
            row.name.clone(),
            row.record_type.clone(),
            row.content.clone(),
        );
        updater
            .remove_from_rrset(to_fqdn(&name, &zone), record_type, vec![row.record], &zone)
            .await
            .map_err(|e| err(e.to_string()))?;
        let after = updater
            .list_zone_records(&zone)
            .await
            .map_err(|e| err(e.to_string()))?;
        if still_listed(&after, &name, &listed_type, &content, &zone) {
            return Err(err(format!(
                "the provider still lists {listed_type} '{name}' after the delete — the record was not removed"
            )));
        }
        Ok(true)
    }
}

/// A provider that answers 200 to a write has not necessarily changed anything: some of
/// them store a value in a different form than the one they list it back in, and the row
/// the caller matched then survives every delete. Reading the zone back is the only check
/// that holds across all of them.
fn still_listed(
    rows: &[ZoneRecord],
    name: &str,
    record_type: &str,
    content: &str,
    zone: &str,
) -> bool {
    rows.iter().any(|row| {
        row.content == content
            && row.record_type.eq_ignore_ascii_case(record_type)
            && (row.name == name || to_fqdn(&row.name, zone) == to_fqdn(name, zone))
    })
}

/// Delete and update need one row, and every provider hands back a different key for it.
/// Prefer the id the list call returned; fall back to the full name+type+value triple,
/// because one name holds several types and one type holds several values.
fn pick_bridge_row(
    rows: &[ZoneRecord],
    target: &DnsRecordTargetDto,
    zone: &str,
) -> Result<ZoneRecord, DnsServiceError> {
    let matched = |predicate: &dyn Fn(&ZoneRecord) -> bool| -> Vec<&ZoneRecord> {
        rows.iter().filter(|row| predicate(row)).collect()
    };
    // An id the provider no longer recognises is a stale row, not a wrong row — the
    // name+type+value triple still identifies what the user clicked.
    if !target.id.is_empty() {
        let by_id = matched(&|row| row.id == target.id);
        match by_id.as_slice() {
            [] => {}
            [one] => return Ok((*one).clone()),
            [several @ ..] => {
                return Err(err(format!(
                    "{} records share id '{}' — refresh the record list",
                    several.len(),
                    target.id
                )));
            }
        }
    }
    let by_tuple = matched(&|row| bridge_row_matches(row, target, zone));
    match by_tuple.as_slice() {
        [] => Err(err(format!(
            "record '{}' is no longer listed by this provider — refresh the record list",
            target.name
        ))),
        [one] => Ok((*one).clone()),
        [several @ ..] => Err(err(format!(
            "{} records match '{}' — delete them one at a time",
            several.len(),
            target.name
        ))),
    }
}

fn bridge_row_matches(row: &ZoneRecord, target: &DnsRecordTargetDto, zone: &str) -> bool {
    let same_name =
        row.name == target.name.trim() || to_fqdn(&row.name, zone) == to_fqdn(&target.name, zone);
    same_name
        && (target.record_type.is_empty()
            || row.record_type.eq_ignore_ascii_case(&target.record_type))
        && (target.content.is_empty() || row.content == target.content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_builds_migrated_providers() {
        // Credential keys must match what the native providers stored in the DB.
        for (provider_type, credentials) in [
            (DnsProviderType::Hetzner, r#"{"apiToken":"t"}"#),
            (DnsProviderType::DigitalOcean, r#"{"apiToken":"t"}"#),
            (DnsProviderType::Infomaniak, r#"{"apiToken":"t"}"#),
            (
                DnsProviderType::Godaddy,
                r#"{"apiKey":"k","apiSecret":"s"}"#,
            ),
            (
                DnsProviderType::Porkbun,
                r#"{"apiKey":"k","secretApiKey":"s"}"#,
            ),
            (
                DnsProviderType::Ovh,
                r#"{"applicationKey":"k","applicationSecret":"s","consumerKey":"c"}"#,
            ),
            (
                DnsProviderType::Ovh,
                r#"{"applicationKey":"k","applicationSecret":"s","consumerKey":"c","endpoint":"soyoustart-ca"}"#,
            ),
        ] {
            assert!(
                build_updater(provider_type, credentials).is_ok(),
                "{provider_type:?} should build from {credentials}"
            );
        }
    }

    #[test]
    fn bridge_rejects_bad_provider_credentials() {
        assert!(build_updater(DnsProviderType::Hetzner, r#"{}"#).is_err());
        assert!(
            build_updater(
                DnsProviderType::Ovh,
                r#"{"applicationKey":"k","applicationSecret":"s","consumerKey":"c","endpoint":"nope"}"#
            )
            .is_err()
        );
    }

    fn row(id: &str, name: &str, record_type: &str, content: &str) -> ZoneRecord {
        ZoneRecord {
            id: id.to_string(),
            name: name.to_string(),
            record_type: record_type.to_string(),
            content: content.to_string(),
            ttl: 3600,
            record: DnsRecord::TXT(content.to_string()),
        }
    }

    fn listed_zone() -> Vec<ZoneRecord> {
        vec![
            row("1", "www", "A", "192.0.2.1"),
            row("2", "www", "A", "192.0.2.2"),
            row("3", "www", "TXT", "hello"),
        ]
    }

    /// The panel used to forward the record name as the provider's record id, which made
    /// every delete miss. A bare name is ambiguous and must never resolve to one row.
    #[test]
    fn bridge_never_resolves_a_bare_name() {
        let rows = listed_zone();
        let by_name = DnsRecordTargetDto {
            zone_id: "example.com".into(),
            name: "www".into(),
            ..Default::default()
        };
        assert!(pick_bridge_row(&rows, &by_name, "example.com").is_err());
    }

    #[test]
    fn bridge_resolves_a_row_by_value_or_id() {
        let rows = listed_zone();
        let by_name = DnsRecordTargetDto {
            zone_id: "example.com".into(),
            name: "www".into(),
            ..Default::default()
        };
        let by_value = DnsRecordTargetDto {
            record_type: "A".into(),
            content: "192.0.2.2".into(),
            ..by_name.clone()
        };
        assert_eq!(
            pick_bridge_row(&rows, &by_value, "example.com").unwrap().id,
            "2"
        );
        let by_id = DnsRecordTargetDto {
            id: "3".into(),
            ..by_name.clone()
        };
        assert_eq!(
            pick_bridge_row(&rows, &by_id, "example.com")
                .unwrap()
                .record_type,
            "TXT"
        );
    }

    /// Rows come back relative to the zone while the form may carry the full name.
    #[test]
    fn bridge_matches_relative_and_absolute_names() {
        let rows = listed_zone();
        let absolute = DnsRecordTargetDto {
            zone_id: "example.com".into(),
            name: "www.example.com".into(),
            record_type: "A".into(),
            content: "192.0.2.1".into(),
            id: String::new(),
        };
        assert_eq!(
            pick_bridge_row(&rows, &absolute, "example.com").unwrap().id,
            "1"
        );
    }

    #[test]
    fn bridge_rejects_a_row_that_is_gone() {
        let rows = listed_zone();
        let gone = DnsRecordTargetDto {
            zone_id: "example.com".into(),
            name: "ftp".into(),
            record_type: "A".into(),
            content: "192.0.2.9".into(),
            id: String::new(),
        };
        let error = pick_bridge_row(&rows, &gone, "example.com")
            .unwrap_err()
            .to_string();
        assert!(error.contains("no longer listed"), "{error}");
    }

    /// A row edited elsewhere keeps its name and value but loses its id. The stale id must
    /// not block the delete.
    #[test]
    fn bridge_falls_back_from_a_stale_id() {
        let rows = listed_zone();
        let stale_id = DnsRecordTargetDto {
            zone_id: "example.com".into(),
            id: "does-not-exist".into(),
            name: "www".into(),
            record_type: "TXT".into(),
            content: "hello".into(),
        };
        assert_eq!(
            pick_bridge_row(&rows, &stale_id, "example.com").unwrap().id,
            "3"
        );
    }
}

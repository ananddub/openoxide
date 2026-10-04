//! The panel side of a DNS provider row: what a create stores, what a read hands back to the
//! form, and what an edit does to a secret the form sent back masked. The providers' own HTTP
//! behaviour is covered by `crates/dns`; these tests cover the wiring above it, on a database
//! built by the same migrations a real panel runs.

use std::sync::Arc;

use sqlx::Row;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::str::FromStr;

use super::{
    CreateDnsProviderDto, DNS_SECRET_MASK, DnsProviderType, DnsService, DnsServiceError,
    UpdateDnsProviderDto,
};
use crate::db::queries::FlashQueries;
use crate::flash_gen::models::DnsProviders;

/// A fresh pool migrated exactly like production, plus the service reading it.
async fn fixture(tag: &str) -> (DnsService, sqlx::SqlitePool) {
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();
    let path = std::env::temp_dir().join(format!("openoxide-dns-{tag}.sqlite3"));
    let _ = std::fs::remove_file(&path);
    let url = format!("sqlite://{}", path.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            SqliteConnectOptions::from_str(&url)
                .unwrap()
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .foreign_keys(true),
        )
        .await
        .unwrap();
    sqlx::migrate!("./db/migrations/")
        .run(&pool)
        .await
        .expect("panel migrations should build the dns_providers table");
    let service = DnsService::new(Arc::new(FlashQueries::new(Arc::new(pool.clone()))));
    (service, pool)
}

fn cloudflare(name: &str, token: &str) -> CreateDnsProviderDto {
    CreateDnsProviderDto {
        name: name.to_string(),
        provider_type: DnsProviderType::Cloudflare,
        credentials_json: format!(r#"{{"apiToken":"{token}"}}"#),
    }
}

/// Read the raw row: what the panel stored is what a provider call will later use.
async fn stored_credentials(pool: &sqlx::SqlitePool, id: i64) -> serde_json::Value {
    let raw: String = sqlx::query("SELECT credentials_json FROM dns_providers WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
        .get("credentials_json");
    serde_json::from_str(&raw).unwrap()
}

#[tokio::test]
async fn a_masked_edit_keeps_the_stored_secret() {
    let (service, pool) = fixture("mask-round-trip").await;
    let created = service
        .create_provider(1, cloudflare("edge", "real-token"))
        .await
        .unwrap();

    // The row the form gets back never carries the credential itself.
    assert_eq!(
        created.credentials_json,
        format!(r#"{{"apiToken":"{DNS_SECRET_MASK}"}}"#)
    );

    let updated = service
        .update_provider(
            created.id,
            1,
            UpdateDnsProviderDto {
                name: Some("edge-renamed".into()),
                provider_type: None,
                credentials_json: Some(format!(r#"{{"apiToken":"{DNS_SECRET_MASK}"}}"#)),
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.name, "edge-renamed");
    assert_eq!(
        stored_credentials(&pool, created.id).await["apiToken"],
        serde_json::json!("real-token")
    );

    // And the edit form's next read is masked again, so the secret cannot leak by round-tripping.
    let reread = service.get_provider(created.id, 1).await.unwrap();
    assert_eq!(reread.credentials_json, created.credentials_json);
}

#[tokio::test]
async fn a_mask_for_a_field_that_was_never_stored_is_refused() {
    let (service, _pool) = fixture("unstored-mask").await;
    let created = service
        .create_provider(1, cloudflare("edge", "real-token"))
        .await
        .unwrap();
    let error = service
        .update_provider(
            created.id,
            1,
            UpdateDnsProviderDto {
                name: None,
                provider_type: None,
                credentials_json: Some(format!(
                    r#"{{"apiToken":"real-token","tenantSecret":"{DNS_SECRET_MASK}"}}"#
                )),
            },
        )
        .await
        .unwrap_err()
        .to_string();
    assert!(error.contains("tenantSecret"), "{error}");
}

#[tokio::test]
async fn the_database_pins_the_provider_set_and_one_name_per_organization() {
    let (service, pool) = fixture("constraints").await;
    service
        .create_provider(1, cloudflare("edge", "real-token"))
        .await
        .unwrap();

    // A second provider with the same name in the same organization is the UNIQUE case the
    // panel turns into "already exists" rather than a 500.
    let duplicate = service
        .create_provider(1, cloudflare("edge", "other-token"))
        .await
        .unwrap_err();
    assert!(
        matches!(
            &duplicate,
            DnsServiceError::Database(sqlx::Error::Database(db)) if db.is_unique_violation()
        ),
        "{duplicate}"
    );

    // Out-of-set rows are now refused by the table itself, so no reader can meet one.
    let rejected = sqlx::query(
        "INSERT INTO dns_providers (name, provider_type, credentials_json, organization_id, \
         created_at, updated_at) VALUES ('legacy', 'HETZNER', '{}', 1, 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap_err()
    .to_string();
    assert!(rejected.contains("dns_provider_type_check"), "{rejected}");

    // A row that predates the CHECK — reachable only on a database that was never migrated —
    // is an error, never a silent Cloudflare.
    let legacy = DnsProviders {
        id: 9,
        name: "legacy".into(),
        provider_type: "HETZNER".into(),
        credentials_json: r#"{"apiToken":"x"}"#.into(),
        organization_id: 1,
        created_at: 0,
        updated_at: 0,
    };
    assert!(matches!(
        DnsService::into_dto(legacy),
        Err(DnsServiceError::ProviderError(_))
    ));
}

#[tokio::test]
async fn every_supported_provider_has_a_client_and_understands_its_credentials() {
    let client = reqwest::Client::new();
    for kind in DnsProviderType::ALL {
        // The row's type picks the client; the row's JSON picks the credential shape.
        assert!(
            dns::open(kind, &client).is_ok(),
            "{} has no client",
            kind.as_str()
        );
        let credentials = kind
            .credential_fields()
            .iter()
            .map(|field| {
                // OVH's endpoint is a host selector, not a free string: an invented one is
                // refused, so the sample has to be a value the provider knows.
                let value = if field.key == "endpoint" {
                    "ovh-eu"
                } else {
                    "sample"
                };
                format!(r#""{}":"{value}""#, field.key)
            })
            .collect::<Vec<_>>()
            .join(",");
        let parsed = dns::ProviderConfig::from_json(kind, &format!("{{{credentials}}}"));
        assert!(parsed.is_ok(), "{}: {}", kind.as_str(), parsed.unwrap_err());
    }
}

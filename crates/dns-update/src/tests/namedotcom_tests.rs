/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#[cfg(test)]
mod tests {
    use crate::{
        CAARecord, DnsRecord, DnsRecordType, Error, MXRecord, SRVRecord, TLSARecord, TlsaCertUsage,
        TlsaMatching, TlsaSelector, providers::namedotcom::NameDotComProvider,
    };
    use mockito::{Matcher, Mock, ServerGuard};
    use serde_json::json;
    use std::time::Duration;

    fn setup_provider(endpoint: String) -> NameDotComProvider {
        NameDotComProvider::new("user", "token", Some(Duration::from_secs(1)))
            .unwrap()
            .with_endpoint(endpoint)
    }

    fn basic_auth_value() -> String {
        let credentials =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, "user:token");
        format!("Basic {credentials}")
    }

    fn mock_list_page(
        server: &mut ServerGuard,
        domain: &str,
        page: u32,
        body: serde_json::Value,
    ) -> Mock {
        server
            .mock(
                "GET",
                format!("/domains/{domain}/records?page={page}").as_str(),
            )
            .match_header("authorization", basic_auth_value().as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&body).unwrap())
            .create()
    }

    #[tokio::test]
    async fn test_set_rrset_creates_when_owner_is_empty() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({"records": [], "nextPage": 0}),
        );

        let create_1 = server
            .mock("POST", "/domains/example.com/records")
            .match_body(Matcher::Json(json!({
                "host": "mail",
                "type": "A",
                "answer": "1.1.1.1",
                "ttl": 300,
            })))
            .with_status(200)
            .with_body(r#"{"id":100}"#)
            .create();
        let create_2 = server
            .mock("POST", "/domains/example.com/records")
            .match_body(Matcher::Json(json!({
                "host": "mail",
                "type": "A",
                "answer": "2.2.2.2",
                "ttl": 300,
            })))
            .with_status(200)
            .with_body(r#"{"id":101}"#)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "mail.example.com",
                DnsRecordType::A,
                300,
                vec![
                    DnsRecord::A("1.1.1.1".parse().unwrap()),
                    DnsRecord::A("2.2.2.2".parse().unwrap()),
                ],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
        create_1.assert();
        create_2.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_is_noop_when_already_matches() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 1, "host": "test", "fqdn": "test.example.com.", "type": "A", "answer": "1.1.1.1"}
                ],
                "nextPage": 0,
            }),
        );
        let _no_post = server
            .mock("POST", "/domains/example.com/records")
            .expect(0)
            .create();
        let _no_delete = server
            .mock(
                "DELETE",
                Matcher::Regex("^/domains/example.com/records/".to_string()),
            )
            .expect(0)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "test.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::A("1.1.1.1".parse().unwrap())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_deletes_extras_and_keeps_matching() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 10, "host": "host", "fqdn": "host.example.com.", "type": "A", "answer": "1.1.1.1"},
                    {"id": 11, "host": "host", "fqdn": "host.example.com.", "type": "A", "answer": "9.9.9.9"}
                ],
                "nextPage": 0,
            }),
        );

        let delete_stale = server
            .mock("DELETE", "/domains/example.com/records/11")
            .with_status(200)
            .with_body("{}")
            .create();
        let create_new = server
            .mock("POST", "/domains/example.com/records")
            .match_body(Matcher::Json(json!({
                "host": "host",
                "type": "A",
                "answer": "8.8.8.8",
                "ttl": 300,
            })))
            .with_status(200)
            .with_body(r#"{"id":12}"#)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "host.example.com",
                DnsRecordType::A,
                300,
                vec![
                    DnsRecord::A("1.1.1.1".parse().unwrap()),
                    DnsRecord::A("8.8.8.8".parse().unwrap()),
                ],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
        delete_stale.assert();
        create_new.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_empty_records_deletes_all_of_type() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 20, "host": "gone", "fqdn": "gone.example.com.", "type": "A", "answer": "1.2.3.4"},
                    {"id": 21, "host": "gone", "fqdn": "gone.example.com.", "type": "A", "answer": "5.6.7.8"}
                ],
                "nextPage": 0,
            }),
        );

        let delete_a = server
            .mock("DELETE", "/domains/example.com/records/20")
            .with_status(200)
            .with_body("{}")
            .create();
        let delete_b = server
            .mock("DELETE", "/domains/example.com/records/21")
            .with_status(200)
            .with_body("{}")
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "gone.example.com",
                DnsRecordType::A,
                300,
                vec![],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
        delete_a.assert();
        delete_b.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_cross_type_isolation() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 30, "host": "shared", "fqdn": "shared.example.com.", "type": "A", "answer": "1.1.1.1"},
                    {"id": 31, "host": "shared", "fqdn": "shared.example.com.", "type": "TXT", "answer": "keep-me"},
                    {"id": 32, "host": "shared", "fqdn": "shared.example.com.", "type": "MX", "answer": "mail.example.com", "priority": 10}
                ],
                "nextPage": 0,
            }),
        );

        let _no_delete_others = server
            .mock(
                "DELETE",
                Matcher::Regex("^/domains/example.com/records/(31|32)$".to_string()),
            )
            .expect(0)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "shared.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::A("1.1.1.1".parse().unwrap())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_type_mismatch_returns_error() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "test.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::TXT("not-an-A".to_string())],
                "example.com",
            )
            .await;
        assert!(matches!(result, Err(Error::Api(_))), "got {result:?}");
    }

    #[tokio::test]
    async fn test_set_rrset_tlsa_rejected() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "_25._tcp.mail.example.com",
                DnsRecordType::TLSA,
                300,
                vec![DnsRecord::TLSA(TLSARecord {
                    cert_usage: TlsaCertUsage::DaneEe,
                    selector: TlsaSelector::Spki,
                    matching: TlsaMatching::Sha256,
                    cert_data: vec![0x00],
                })],
                "example.com",
            )
            .await;
        assert!(
            matches!(result, Err(Error::Unsupported(ref msg)) if msg.contains("TLSA")),
            "expected TLSA refusal, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_set_rrset_paginates_list() {
        let mut server = mockito::Server::new_async().await;
        let page_1 = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 1, "host": "test", "fqdn": "test.example.com.", "type": "A", "answer": "1.1.1.1"}
                ],
                "nextPage": 2,
            }),
        );
        let page_2 = mock_list_page(
            &mut server,
            "example.com",
            2,
            json!({
                "records": [
                    {"id": 2, "host": "test", "fqdn": "test.example.com.", "type": "A", "answer": "2.2.2.2"}
                ],
                "nextPage": 0,
            }),
        );

        let delete = server
            .mock("DELETE", "/domains/example.com/records/2")
            .with_status(200)
            .with_body("{}")
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "test.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::A("1.1.1.1".parse().unwrap())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        page_1.assert();
        page_2.assert();
        delete.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_apex_sends_empty_host() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({"records": [], "nextPage": 0}),
        );

        let create = server
            .mock("POST", "/domains/example.com/records")
            .match_body(Matcher::Json(json!({
                "host": "",
                "type": "MX",
                "answer": "mail.example.com.",
                "ttl": 3600,
                "priority": 10,
            })))
            .with_status(200)
            .with_body(r#"{"id":50}"#)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "example.com",
                DnsRecordType::MX,
                3600,
                vec![DnsRecord::MX(MXRecord {
                    exchange: "mail.example.com.".to_string(),
                    priority: 10,
                })],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
        create.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_apex_filters_listed_host_at_sign() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 60, "host": "@", "fqdn": "example.com.", "type": "MX", "answer": "mail.example.com.", "priority": 10}
                ],
                "nextPage": 0,
            }),
        );
        let _no_create = server
            .mock("POST", "/domains/example.com/records")
            .expect(0)
            .create();
        let _no_delete = server
            .mock(
                "DELETE",
                Matcher::Regex("^/domains/example.com/records/".to_string()),
            )
            .expect(0)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "example.com",
                DnsRecordType::MX,
                3600,
                vec![DnsRecord::MX(MXRecord {
                    exchange: "mail.example.com.".to_string(),
                    priority: 10,
                })],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_mx_diffs_by_priority_and_exchange() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 70, "host": "", "fqdn": "example.com.", "type": "MX", "answer": "mx1.example.com.", "priority": 10}
                ],
                "nextPage": 0,
            }),
        );

        let create = server
            .mock("POST", "/domains/example.com/records")
            .match_body(Matcher::Json(json!({
                "host": "",
                "type": "MX",
                "answer": "mx2.example.com.",
                "ttl": 3600,
                "priority": 20,
            })))
            .with_status(200)
            .with_body(r#"{"id":71}"#)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "example.com",
                DnsRecordType::MX,
                3600,
                vec![
                    DnsRecord::MX(MXRecord {
                        exchange: "mx1.example.com.".to_string(),
                        priority: 10,
                    }),
                    DnsRecord::MX(MXRecord {
                        exchange: "mx2.example.com.".to_string(),
                        priority: 20,
                    }),
                ],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
        create.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_srv_packs_weight_port_target() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({"records": [], "nextPage": 0}),
        );

        let create = server
            .mock("POST", "/domains/example.com/records")
            .match_body(Matcher::Json(json!({
                "host": "_imaps._tcp",
                "type": "SRV",
                "answer": "5 993 mail.example.com",
                "ttl": 3600,
                "priority": 10,
            })))
            .with_status(200)
            .with_body(r#"{"id":80}"#)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "_imaps._tcp.example.com",
                DnsRecordType::SRV,
                3600,
                vec![DnsRecord::SRV(SRVRecord {
                    priority: 10,
                    weight: 5,
                    port: 993,
                    target: "mail.example.com".to_string(),
                })],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset returned: {result:?}");
        list.assert();
        create.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_caa_rejected_by_core_api() {
        let mut server = mockito::Server::new_async().await;
        let _no_call = server.mock("GET", Matcher::Any).expect(0).create();
        let _no_post = server.mock("POST", Matcher::Any).expect(0).create();

        let provider = setup_provider(server.url());
        let result = provider
            .set_rrset(
                "example.com",
                DnsRecordType::CAA,
                3600,
                vec![DnsRecord::CAA(CAARecord::Issue {
                    issuer_critical: false,
                    name: Some("letsencrypt.org".to_string()),
                    options: vec![],
                })],
                "example.com",
            )
            .await;

        assert!(
            matches!(result, Err(Error::Unsupported(ref msg)) if msg.contains("CAA")),
            "expected CAA refusal, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_add_to_rrset_skips_existing_values() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 100, "host": "_acme", "fqdn": "_acme.example.com.", "type": "TXT", "answer": "existing"}
                ],
                "nextPage": 0,
            }),
        );

        let create = server
            .mock("POST", "/domains/example.com/records")
            .match_body(Matcher::Json(json!({
                "host": "_acme",
                "type": "TXT",
                "answer": "new-token",
                "ttl": 60,
            })))
            .with_status(200)
            .with_body(r#"{"id":101}"#)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .add_to_rrset(
                "_acme.example.com",
                DnsRecordType::TXT,
                60,
                vec![
                    DnsRecord::TXT("existing".to_string()),
                    DnsRecord::TXT("new-token".to_string()),
                ],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "add_to_rrset returned: {result:?}");
        list.assert();
        create.assert();
    }

    #[tokio::test]
    async fn test_add_to_rrset_empty_is_noop() {
        let mut server = mockito::Server::new_async().await;
        let _no_call = server.mock("GET", Matcher::Any).expect(0).create();
        let _no_post = server.mock("POST", Matcher::Any).expect(0).create();

        let provider = setup_provider(server.url());
        let result = provider
            .add_to_rrset(
                "test.example.com",
                DnsRecordType::A,
                300,
                vec![],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "add_to_rrset returned: {result:?}");
    }

    #[tokio::test]
    async fn test_remove_from_rrset_deletes_only_matching() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 110, "host": "_acme", "fqdn": "_acme.example.com.", "type": "TXT", "answer": "keep-me"},
                    {"id": 111, "host": "_acme", "fqdn": "_acme.example.com.", "type": "TXT", "answer": "drop-me"}
                ],
                "nextPage": 0,
            }),
        );

        let delete = server
            .mock("DELETE", "/domains/example.com/records/111")
            .with_status(200)
            .with_body("{}")
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .remove_from_rrset(
                "_acme.example.com",
                DnsRecordType::TXT,
                vec![DnsRecord::TXT("drop-me".to_string())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "remove_from_rrset returned: {result:?}");
        list.assert();
        delete.assert();
    }

    #[tokio::test]
    async fn test_remove_from_rrset_noop_when_absent() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 120, "host": "test", "fqdn": "test.example.com.", "type": "A", "answer": "1.1.1.1"}
                ],
                "nextPage": 0,
            }),
        );
        let _no_delete = server
            .mock(
                "DELETE",
                Matcher::Regex("^/domains/example.com/records/".to_string()),
            )
            .expect(0)
            .create();

        let provider = setup_provider(server.url());
        let result = provider
            .remove_from_rrset(
                "test.example.com",
                DnsRecordType::A,
                vec![DnsRecord::A("9.9.9.9".parse().unwrap())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "remove_from_rrset returned: {result:?}");
        list.assert();
    }

    #[tokio::test]
    async fn test_remove_from_rrset_empty_is_noop() {
        let mut server = mockito::Server::new_async().await;
        let _no_call = server.mock("GET", Matcher::Any).expect(0).create();
        let _no_delete = server.mock("DELETE", Matcher::Any).expect(0).create();

        let provider = setup_provider(server.url());
        let result = provider
            .remove_from_rrset("test.example.com", DnsRecordType::A, vec![], "example.com")
            .await;

        assert!(result.is_ok(), "remove_from_rrset returned: {result:?}");
    }

    #[tokio::test]
    async fn test_list_rrset_returns_filtered_records() {
        let mut server = mockito::Server::new_async().await;
        let list = mock_list_page(
            &mut server,
            "example.com",
            1,
            json!({
                "records": [
                    {"id": 130, "host": "host", "fqdn": "host.example.com.", "type": "A", "answer": "1.1.1.1"},
                    {"id": 131, "host": "host", "fqdn": "host.example.com.", "type": "A", "answer": "2.2.2.2"},
                    {"id": 132, "host": "host", "fqdn": "host.example.com.", "type": "TXT", "answer": "ignored"},
                    {"id": 133, "host": "other", "fqdn": "other.example.com.", "type": "A", "answer": "3.3.3.3"}
                ],
                "nextPage": 0,
            }),
        );

        let provider = setup_provider(server.url());
        let result = provider
            .list_rrset("host.example.com", DnsRecordType::A, "example.com")
            .await
            .expect("list_rrset");
        list.assert();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&DnsRecord::A("1.1.1.1".parse().unwrap())));
        assert!(result.contains(&DnsRecord::A("2.2.2.2".parse().unwrap())));
    }

    #[tokio::test]
    async fn test_list_rrset_tlsa_rejected() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url());
        let result = provider
            .list_rrset("a.example.com", DnsRecordType::TLSA, "example.com")
            .await;
        assert!(
            matches!(result, Err(Error::Unsupported(ref msg)) if msg.contains("TLSA")),
            "got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_add_to_rrset_tlsa_rejected() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url());
        let result = provider
            .add_to_rrset(
                "_25._tcp.mail.example.com",
                DnsRecordType::TLSA,
                300,
                vec![DnsRecord::TLSA(TLSARecord {
                    cert_usage: TlsaCertUsage::DaneEe,
                    selector: TlsaSelector::Spki,
                    matching: TlsaMatching::Sha256,
                    cert_data: vec![0x00],
                })],
                "example.com",
            )
            .await;
        assert!(
            matches!(result, Err(Error::Unsupported(ref msg)) if msg.contains("TLSA")),
            "got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_list_zone_infos_maps_domain_names() {
        let mut server = mockito::Server::new_async().await;
        let list_1 = server
            .mock("GET", "/domains?page=1")
            .match_header("authorization", basic_auth_value().as_str())
            .with_status(200)
            .with_body(
                json!({
                    "domains": [
                        {"domainName": "example.com"},
                        {"domainName": "other.com"}
                    ],
                    "nextPage": 2,
                })
                .to_string(),
            )
            .create();
        let list_2 = server
            .mock("GET", "/domains?page=2")
            .with_status(200)
            .with_body(
                json!({
                    "domains": [{"domainName": "third.com"}],
                    "nextPage": 0,
                })
                .to_string(),
            )
            .create();

        let provider = setup_provider(server.url());
        let zones = provider.list_zone_infos().await.expect("list_zone_infos");
        list_1.assert();
        list_2.assert();
        assert_eq!(zones.len(), 3);
        assert_eq!(zones[0].name, "example.com");
        assert_eq!(zones[0].id, "example.com");
        assert_eq!(zones[2].name, "third.com");
    }

    #[tokio::test]
    async fn test_list_zone_records_returns_core_api_shapes() {
        let mut server = mockito::Server::new_async().await;
        let list = server
            .mock("GET", "/domains/example.com/records?page=1")
            .match_header("authorization", basic_auth_value().as_str())
            .with_status(200)
            .with_body(
                json!({
                    "records": [
                        {"id": 1, "host": "", "fqdn": "example.com.", "type": "A", "answer": "1.1.1.1", "ttl": 300},
                        {"id": 2, "host": "www", "fqdn": "www.example.com.", "type": "CNAME", "answer": "example.com.", "ttl": 3600},
                        {"id": 3, "host": null, "fqdn": "example.com.", "type": "SOA", "answer": "ignored", "ttl": 3600}
                    ],
                    "nextPage": 0,
                })
                .to_string(),
            )
            .create();

        let provider = setup_provider(server.url());
        let records = provider
            .list_zone_records("example.com")
            .await
            .expect("list_zone_records");
        list.assert();
        assert_eq!(
            records.len(),
            2,
            "unsupported SOA must be skipped: {records:?}"
        );
        assert_eq!(records[0].id, "1");
        assert_eq!(records[0].name, "@");
        assert_eq!(records[0].record_type, "A");
        assert_eq!(records[0].ttl, 300);
        assert_eq!(records[0].record, DnsRecord::A("1.1.1.1".parse().unwrap()));
        assert_eq!(records[1].name, "www");
        assert_eq!(records[1].record_type, "CNAME");
    }
}

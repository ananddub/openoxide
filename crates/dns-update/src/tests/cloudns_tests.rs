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
        DnsRecord, DnsRecordType, DnsUpdater, Error, MXRecord, providers::cloudns::ClouDnsProvider,
    };
    use std::time::Duration;

    fn setup_provider(endpoint: &str) -> ClouDnsProvider {
        ClouDnsProvider::new(
            Some("auth_user"),
            None::<&str>,
            "pw",
            Some(Duration::from_secs(5)),
        )
        .unwrap()
        .with_endpoint(endpoint)
    }

    #[test]
    fn test_provider_requires_auth() {
        let result = ClouDnsProvider::new(None::<&str>, None::<&str>, "pw", None);
        assert!(matches!(result, Err(Error::Api(_))));
    }

    #[tokio::test]
    async fn test_set_rrset_empty_deletes_all() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "1": {"id": "1", "type": "A", "host": "www", "record": "1.1.1.1"},
                    "2": {"id": "2", "type": "A", "host": "www", "record": "2.2.2.2"}
                }"#,
            )
            .create();

        let del1 = server
            .mock("POST", "/delete-record.json")
            .match_body(mockito::Matcher::UrlEncoded("record-id".into(), "1".into()))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"removed"}"#)
            .expect_at_least(1)
            .create();
        let del2 = server
            .mock("POST", "/delete-record.json")
            .match_body(mockito::Matcher::UrlEncoded("record-id".into(), "2".into()))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"removed"}"#)
            .expect_at_least(1)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                Vec::new(),
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset(empty) failed: {result:?}");
        lookup.assert();
        del1.assert();
        del2.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_empty_on_empty_rrset_noop() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body("[]")
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                Vec::new(),
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset(empty) failed: {result:?}");
        lookup.assert();
    }

    #[tokio::test]
    async fn test_in_body_rate_limit_is_retried() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body("[]")
            .create();

        let add_throttled = server
            .mock("POST", "/add-record.json")
            .with_status(200)
            .with_body(
                r#"{"status":"Failed","statusDescription":"Request blocked, 1.2.3.4 is sending more than 20 requests per second."}"#,
            )
            .expect(1)
            .create();

        let add_ok = server
            .mock("POST", "/add-record.json")
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"added"}"#)
            .expect(1)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .add_to_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::A("3.3.3.3".parse().unwrap())],
                "example.com",
            )
            .await;

        assert!(
            result.is_ok(),
            "add_to_rrset should retry throttle: {result:?}"
        );
        lookup.assert();
        add_throttled.assert();
        add_ok.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_diff_adds_and_deletes() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "1": {"id": "1", "type": "A", "host": "www", "record": "1.1.1.1"},
                    "2": {"id": "2", "type": "A", "host": "www", "record": "2.2.2.2"}
                }"#,
            )
            .create();

        let delete_stale = server
            .mock("POST", "/delete-record.json")
            .match_body(mockito::Matcher::UrlEncoded("record-id".into(), "2".into()))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"removed"}"#)
            .create();

        let add_new = server
            .mock("POST", "/add-record.json")
            .match_body(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("host".into(), "www".into()),
                mockito::Matcher::UrlEncoded("record-type".into(), "A".into()),
                mockito::Matcher::UrlEncoded("record".into(), "3.3.3.3".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"added"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                vec![
                    DnsRecord::A("1.1.1.1".parse().unwrap()),
                    DnsRecord::A("3.3.3.3".parse().unwrap()),
                ],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset failed: {result:?}");
        lookup.assert();
        delete_stale.assert();
        add_new.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_idempotent_no_writes() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "1": {"id": "1", "type": "A", "host": "www", "record": "1.1.1.1"}
                }"#,
            )
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::A("1.1.1.1".parse().unwrap())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset failed: {result:?}");
        lookup.assert();
    }

    #[tokio::test]
    async fn test_add_to_rrset_skips_existing() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "1": {"id": "1", "type": "A", "host": "www", "record": "1.1.1.1"}
                }"#,
            )
            .create();

        let add_new = server
            .mock("POST", "/add-record.json")
            .match_body(mockito::Matcher::UrlEncoded(
                "record".into(),
                "2.2.2.2".into(),
            ))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"added"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .add_to_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                vec![
                    DnsRecord::A("1.1.1.1".parse().unwrap()),
                    DnsRecord::A("2.2.2.2".parse().unwrap()),
                ],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "add_to_rrset failed: {result:?}");
        lookup.assert();
        add_new.assert();
    }

    #[tokio::test]
    async fn test_add_to_rrset_empty_is_noop() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url().as_str());
        let result = provider
            .add_to_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                Vec::new(),
                "example.com",
            )
            .await;
        assert!(result.is_ok(), "add_to_rrset(empty) failed: {result:?}");
    }

    #[tokio::test]
    async fn test_remove_from_rrset_deletes_matching() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "1": {"id": "1", "type": "A", "host": "www", "record": "1.1.1.1"},
                    "2": {"id": "2", "type": "A", "host": "www", "record": "2.2.2.2"}
                }"#,
            )
            .create();

        let delete = server
            .mock("POST", "/delete-record.json")
            .match_body(mockito::Matcher::UrlEncoded("record-id".into(), "1".into()))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"removed"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .remove_from_rrset(
                "www.example.com",
                DnsRecordType::A,
                vec![
                    DnsRecord::A("1.1.1.1".parse().unwrap()),
                    DnsRecord::A("9.9.9.9".parse().unwrap()),
                ],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "remove_from_rrset failed: {result:?}");
        lookup.assert();
        delete.assert();
    }

    #[tokio::test]
    async fn test_remove_from_rrset_matches_a_cname_the_api_dotted() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "7": {"id": "7", "type": "CNAME", "host": "www", "record": "target.example.com."}
                }"#,
            )
            .create();
        let delete = server
            .mock("POST", "/delete-record.json")
            .match_body(mockito::Matcher::UrlEncoded("record-id".into(), "7".into()))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"removed"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .remove_from_rrset(
                "www.example.com",
                DnsRecordType::CNAME,
                vec![DnsRecord::CNAME("target.example.com".to_string())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "remove_from_rrset failed: {result:?}");
        lookup.assert();
        delete.assert();
    }

    #[tokio::test]
    async fn test_remove_from_rrset_empty_is_noop() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url().as_str());
        let result = provider
            .remove_from_rrset(
                "www.example.com",
                DnsRecordType::A,
                Vec::new(),
                "example.com",
            )
            .await;
        assert!(
            result.is_ok(),
            "remove_from_rrset(empty) failed: {result:?}"
        );
    }

    #[tokio::test]
    async fn test_list_rrset_returns_records() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "1": {"id": "1", "type": "A", "host": "www", "record": "1.1.1.1"},
                    "2": {"id": "2", "type": "A", "host": "www", "record": "2.2.2.2"}
                }"#,
            )
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .list_rrset("www.example.com", DnsRecordType::A, "example.com")
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&DnsRecord::A("1.1.1.1".parse().unwrap())));
        assert!(result.contains(&DnsRecord::A("2.2.2.2".parse().unwrap())));
        lookup.assert();
    }

    #[tokio::test]
    async fn test_list_rrset_empty() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body("[]")
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .list_rrset("www.example.com", DnsRecordType::A, "example.com")
            .await
            .unwrap();
        assert!(result.is_empty());
        lookup.assert();
    }

    #[tokio::test]
    async fn test_type_mismatch_rejected() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "www.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::TXT("nope".to_string())],
                "example.com",
            )
            .await;
        assert!(matches!(result, Err(Error::Api(_))));
    }

    #[tokio::test]
    async fn test_set_rrset_rejects_tlsa() {
        let server = mockito::Server::new_async().await;
        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "www.example.com",
                DnsRecordType::TLSA,
                300,
                Vec::new(),
                "example.com",
            )
            .await;
        assert!(matches!(result, Err(Error::Unsupported(_))));
    }

    #[tokio::test]
    async fn test_set_rrset_normalizes_trailing_dot_on_cname() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body("[]")
            .create();

        let add_new = server
            .mock("POST", "/add-record.json")
            .match_body(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("record-type".into(), "CNAME".into()),
                mockito::Matcher::UrlEncoded("record".into(), "target.example.org".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"added"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "www.example.com",
                DnsRecordType::CNAME,
                300,
                vec![DnsRecord::CNAME("target.example.org.".to_string())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset(cname) failed: {result:?}");
        lookup.assert();
        add_new.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_normalizes_trailing_dot_on_mx_exchange() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body("[]")
            .create();

        let add_new = server
            .mock("POST", "/add-record.json")
            .match_body(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("record-type".into(), "MX".into()),
                mockito::Matcher::UrlEncoded("record".into(), "mail.example.org".into()),
                mockito::Matcher::UrlEncoded("priority".into(), "10".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"added"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "example.com",
                DnsRecordType::MX,
                300,
                vec![DnsRecord::MX(MXRecord {
                    exchange: "mail.example.org.".to_string(),
                    priority: 10,
                })],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset(mx) failed: {result:?}");
        lookup.assert();
        add_new.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_multi_record_diff_for_widened_finder() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body(
                r#"{
                    "1": {"id": "1", "type": "A", "host": "multi", "record": "1.1.1.1"},
                    "2": {"id": "2", "type": "A", "host": "multi", "record": "2.2.2.2"},
                    "3": {"id": "3", "type": "A", "host": "multi", "record": "3.3.3.3"}
                }"#,
            )
            .create();

        let del2 = server
            .mock("POST", "/delete-record.json")
            .match_body(mockito::Matcher::UrlEncoded("record-id".into(), "2".into()))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"removed"}"#)
            .create();
        let del3 = server
            .mock("POST", "/delete-record.json")
            .match_body(mockito::Matcher::UrlEncoded("record-id".into(), "3".into()))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"removed"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "multi.example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::A("1.1.1.1".parse().unwrap())],
                "example.com",
            )
            .await;

        assert!(result.is_ok(), "set_rrset failed: {result:?}");
        lookup.assert();
        del2.assert();
        del3.assert();
    }

    #[tokio::test]
    async fn test_set_rrset_apex_uses_empty_host() {
        let mut server = mockito::Server::new_async().await;
        let lookup = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("host".into(), "".into()),
                mockito::Matcher::UrlEncoded("type".into(), "A".into()),
                // ClouDNS records.json accepts rows-per-page of only
                // 10, 20, 30, 50 or 100 (https://www.cloudns.net/wiki/article/57/).
                mockito::Matcher::UrlEncoded("rows-per-page".into(), "100".into()),
            ]))
            .with_status(200)
            .with_body("[]")
            .create();

        let add = server
            .mock("POST", "/add-record.json")
            .match_body(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("host".into(), "".into()),
                mockito::Matcher::UrlEncoded("record-type".into(), "A".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"Success","statusDescription":"added"}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .set_rrset(
                "example.com",
                DnsRecordType::A,
                300,
                vec![DnsRecord::A("1.2.3.4".parse().unwrap())],
                "example.com",
            )
            .await;
        assert!(result.is_ok(), "apex set_rrset failed: {result:?}");
        lookup.assert();
        add.assert();
    }

    #[tokio::test]
    async fn test_list_zone_infos_parses_array_response() {
        // list-zones.json returns a JSON *array* of zone objects
        // (https://www.cloudns.net/wiki/article/50/).
        let mut server = mockito::Server::new_async().await;
        let zones_mock = server
            .mock("GET", mockito::Matcher::Regex("^/list-zones.json".into()))
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("page".into(), "1".into()),
                mockito::Matcher::UrlEncoded("rows-per-page".into(), "100".into()),
            ]))
            .with_status(200)
            .with_body(
                r#"[{"id":1,"name":"example.com","status":"active"},
                   {"id":2,"name":"example.org","status":"deleged"}]"#,
            )
            .create();

        let provider = setup_provider(server.url().as_str());
        let zones = provider.list_zone_infos().await.unwrap();
        assert_eq!(zones.len(), 2);
        assert_eq!(zones[0].id, "1");
        assert_eq!(zones[0].name, "example.com");
        assert_eq!(zones[0].status.as_deref(), Some("active"));
        assert_eq!(zones[1].name, "example.org");
        zones_mock.assert();
    }

    #[tokio::test]
    async fn test_list_zone_infos_surfaces_error_envelope() {
        let mut server = mockito::Server::new_async().await;
        let zones_mock = server
            .mock("GET", mockito::Matcher::Regex("^/list-zones.json".into()))
            .with_status(200)
            .with_body(r#"{"status":"Failed","statusDescription":"Invalid authentication."}"#)
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider.list_zone_infos().await;
        assert!(matches!(result, Err(Error::Api(_))), "got {result:?}");
        zones_mock.assert();
    }

    #[tokio::test]
    async fn test_list_zone_records_paginates_until_short_page() {
        // records.json is an object keyed by record id; a full page of 100
        // must trigger a second request, which returns "[]" to stop.
        let mut page1 = serde_json::Map::new();
        for i in 1..=100 {
            page1.insert(
                i.to_string(),
                serde_json::json!({
                    "id": i.to_string(),
                    "type": "A",
                    "host": format!("www{i}"),
                    "record": format!("10.0.0.{}", i % 256),
                    "ttl": 60,
                }),
            );
        }
        let page1 = serde_json::Value::Object(page1).to_string();

        let mut server = mockito::Server::new_async().await;
        let first = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .match_query(mockito::Matcher::UrlEncoded("page".into(), "1".into()))
            .with_status(200)
            .with_body(page1)
            .create();
        let second = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .match_query(mockito::Matcher::UrlEncoded("page".into(), "2".into()))
            .with_status(200)
            .with_body("[]")
            .create();

        let provider = setup_provider(server.url().as_str());
        let records = provider.list_zone_records("example.com").await.unwrap();
        assert_eq!(records.len(), 100);
        assert_eq!(records[0].record_type, "A");
        first.assert();
        second.assert();
    }

    #[tokio::test]
    async fn test_list_zone_records_empty() {
        let mut server = mockito::Server::new_async().await;
        let recs_mock = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .with_status(200)
            .with_body("[]")
            .create();

        let provider = setup_provider(server.url().as_str());
        let records = provider.list_zone_records("example.com").await.unwrap();
        assert!(records.is_empty());
        recs_mock.assert();
    }

    #[tokio::test]
    async fn test_list_at_follows_pagination() {
        // An rrset wider than one 100-record page must be collected across
        // pages instead of being truncated to the first 100.
        let mut page1 = serde_json::Map::new();
        for i in 1..=100 {
            page1.insert(
                i.to_string(),
                serde_json::json!({
                    "id": i.to_string(),
                    "type": "A",
                    "host": "many",
                    "record": format!("10.0.0.{}", i % 256),
                }),
            );
        }
        let page1 = serde_json::Value::Object(page1).to_string();

        let mut server = mockito::Server::new_async().await;
        let first = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .match_query(mockito::Matcher::UrlEncoded("page".into(), "1".into()))
            .with_status(200)
            .with_body(page1)
            .create();
        let second = server
            .mock("GET", mockito::Matcher::Regex("^/records.json".into()))
            .match_query(mockito::Matcher::UrlEncoded("page".into(), "2".into()))
            .with_status(200)
            .with_body(
                r#"{"101": {"id": "101", "type": "A", "host": "many", "record": "10.0.0.101"}}"#,
            )
            .create();

        let provider = setup_provider(server.url().as_str());
        let result = provider
            .list_rrset("many.example.com", DnsRecordType::A, "example.com")
            .await
            .unwrap();
        assert_eq!(result.len(), 101);
        first.assert();
        second.assert();
    }

    #[test]
    fn dns_updater_creation() {
        let updater = DnsUpdater::new_cloudns(
            Some("user"),
            None::<&str>,
            "pw",
            Some(Duration::from_secs(30)),
        );
        assert!(matches!(updater, Ok(DnsUpdater::ClouDns(..))));
    }
}

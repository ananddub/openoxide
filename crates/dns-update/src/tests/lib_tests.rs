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

use crate::utils::{strip_origin_from_name, txt_chunks_to_text};

#[test]
fn test_txt_chunks_to_text_short() {
    let mut out = String::new();
    txt_chunks_to_text(&mut out, "hello", " ");
    assert_eq!(out, "\"hello\"");
}

#[test]
fn test_txt_chunks_to_text_empty() {
    let mut out = String::new();
    txt_chunks_to_text(&mut out, "", " ");
    assert_eq!(out, "\"\"");
}

#[test]
fn test_txt_chunks_to_text_escapes_backslash_and_quote() {
    let mut out = String::new();
    txt_chunks_to_text(&mut out, r#"a"b\c"#, " ");
    assert_eq!(out, r#""a\"b\\c""#);
}

#[test]
fn test_txt_chunks_to_text_exact_255() {
    let s = "a".repeat(255);
    let mut out = String::new();
    txt_chunks_to_text(&mut out, &s, " ");
    assert_eq!(out, format!("\"{}\"", s));
}

#[test]
fn test_txt_chunks_to_text_just_over_255() {
    let s = "a".repeat(256);
    let mut out = String::new();
    txt_chunks_to_text(&mut out, &s, " ");
    assert_eq!(out, format!("\"{}\" \"a\"", "a".repeat(255)));
}

#[test]
fn test_txt_chunks_to_text_dkim_sized() {
    let s = "A".repeat(400);
    let mut out = String::new();
    txt_chunks_to_text(&mut out, &s, " ");
    assert_eq!(
        out,
        format!("\"{}\" \"{}\"", "A".repeat(255), "A".repeat(145))
    );
}

#[test]
fn test_txt_chunks_to_text_respects_utf8_boundary() {
    let mut s: String = "a".repeat(254);
    s.push('€');
    let mut out = String::new();
    txt_chunks_to_text(&mut out, &s, " ");
    assert_eq!(out, format!("\"{}\" \"€\"", "a".repeat(254)));
}

#[test]
fn test_txt_chunks_to_text_custom_separator() {
    let s = "a".repeat(300);
    let mut out = String::new();
    txt_chunks_to_text(&mut out, &s, "\n    ");
    assert_eq!(
        out,
        format!("\"{}\"\n    \"{}\"", "a".repeat(255), "a".repeat(45))
    );
}

#[test]
fn test_txt_chunks_to_text_appends_to_existing() {
    let mut out = String::from("prefix: ");
    txt_chunks_to_text(&mut out, "hello", " ");
    assert_eq!(out, "prefix: \"hello\"");
}

#[test]
fn test_strip_origin_from_name() {
    assert_eq!(
        strip_origin_from_name("www.example.com", "example.com", None),
        "www"
    );
    assert_eq!(
        strip_origin_from_name("example.com", "example.com", None),
        "@"
    );
    assert_eq!(
        strip_origin_from_name("api.v1.example.com", "example.com", None),
        "api.v1"
    );
    assert_eq!(
        strip_origin_from_name("example.com", "google.com", None),
        "example.com"
    );
    assert_eq!(
        strip_origin_from_name("example.com", "example.com", Some("")),
        ""
    );
}

#[cfg(feature = "aws-lc-rs")]
#[tokio::test]
async fn test_https_mock() {
    if cfg!(all(feature = "aws-lc-rs", feature = "ring")) {
        panic!("Cannot enable both aws-lc-rs and ring features simultaneously");
    }
    #[cfg(feature = "aws-lc-rs")]
    {
        ::rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Failed to set AWS LC RS provider");
    }
    #[cfg(feature = "ring")]
    {
        ::rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to set ring provider");
    }
    #[cfg(not(any(feature = "aws-lc-rs", feature = "ring")))]
    {
        panic!("No TLS backend feature enabled");
    }
    let server = httpmock::MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(httpmock::Method::GET).path("/test");
        then.status(200).body("hello");
    });
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let resp = client
        .get(server.base_url() + "/test")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "hello");
    mock.assert_calls(1);
}

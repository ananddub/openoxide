use super::*;

#[test]
fn verifies_github_signature() {
    assert!(verify_hmac_sha256(
        b"secret",
        b"payload",
        "sha256=b82fcb791acec57859b989b430a826488ce2e479fdf92326bd0a2e8375a42ba4"
    ));
    assert!(!verify_hmac_sha256(
        b"secret",
        b"modified",
        "sha256=b82fcb791acec57859b989b430a826488ce2e479fdf92326bd0a2e8375a42ba4"
    ));
}

#[test]
fn parses_github_push_and_deduplicates_paths() {
    let body = br#"{"ref":"refs/heads/main","before":"a","after":"b","repository":{"name":"api","owner":{"login":"acme"}},"commits":[{"added":["src/main.rs"],"modified":["src/main.rs","Cargo.toml"],"removed":[]}]}"#;
    let event = parse_push(GitProviderKind::Github, body).unwrap();
    assert_eq!(event.owner, "acme");
    assert_eq!(event.repository, "api");
    assert_eq!(event.branch, "main");
    assert_eq!(event.changed_paths, vec!["Cargo.toml", "src/main.rs"]);
}

#[test]
fn parses_gitlab_namespace() {
    let body = br#"{"ref":"refs/heads/develop","before":"a","after":"b","project":{"path":"api","path_with_namespace":"acme/platform/api"},"commits":[]}"#;
    let event = parse_push(GitProviderKind::Gitlab, body).unwrap();
    assert_eq!(event.owner, "acme/platform");
    assert_eq!(event.branch, "develop");
}

#[test]
fn parses_github_tag_push() {
    let body = br#"{"ref":"refs/tags/v1.2.3","before":"a","after":"b","repository":{"name":"api","owner":{"login":"acme"}},"commits":[]}"#;
    let event = parse_event(GitProviderKind::Github, "push", body).unwrap();
    let WebhookEvent::GitRef(event) = event else {
        panic!("expected git ref event");
    };
    assert_eq!(event.trigger, GitTrigger::Tag);
    assert_eq!(event.branch, "v1.2.3");
}

#[test]
fn parses_github_pull_request_contract() {
    let body = br#"{"action":"opened","number":42,"repository":{"name":"api","owner":{"login":"acme"}},"pull_request":{"head":{"ref":"feature/test","sha":"abc","repo":{"name":"api-fork","owner":{"login":"alice"}}},"base":{"ref":"main"},"user":{"login":"alice"}}}"#;
    let event = parse_event(GitProviderKind::Github, "pull_request", body).unwrap();
    let WebhookEvent::PullRequest(event) = event else {
        panic!("expected pull request event");
    };
    assert_eq!(event.number, "42");
    assert_eq!(event.action, "opened");
    assert_eq!(event.source_branch, "feature/test");
    assert_eq!(event.source_owner.as_deref(), Some("alice"));
    assert_eq!(event.source_repository.as_deref(), Some("api-fork"));
    assert_eq!(event.target_branch, "main");
    assert_eq!(event.author.as_deref(), Some("alice"));
}

#[test]
fn parses_gitlab_merge_request_contract() {
    let body = br#"{"project":{"path":"api","path_with_namespace":"acme/platform/api"},"source":{"path":"api-fork","path_with_namespace":"alice/api-fork"},"user":{"username":"alice"},"object_attributes":{"iid":7,"action":"update","source_branch":"feature","target_branch":"main","last_commit":{"id":"def"}}}"#;
    let event = parse_event(GitProviderKind::Gitlab, "Merge Request Hook", body).unwrap();
    let WebhookEvent::PullRequest(event) = event else {
        panic!("expected merge request event");
    };
    assert_eq!(event.owner, "acme/platform");
    assert_eq!(event.number, "7");
    assert_eq!(event.source_owner.as_deref(), Some("alice"));
    assert_eq!(event.source_repository.as_deref(), Some("api-fork"));
    assert_eq!(event.commit.as_deref(), Some("def"));
}

#[test]
fn parses_bitbucket_fork_pull_request_contract() {
    let body = br#"{"repository":{"name":"api","owner":{"nickname":"acme"}},"pullrequest":{"id":9,"source":{"branch":{"name":"feature"},"commit":{"hash":"fed"},"repository":{"name":"api-fork","full_name":"alice/api-fork"}},"destination":{"branch":{"name":"main"}},"author":{"nickname":"alice"}}}"#;
    let event = parse_event(GitProviderKind::Bitbucket, "pullrequest:updated", body).unwrap();
    let WebhookEvent::PullRequest(event) = event else {
        panic!("expected pull request event");
    };
    assert_eq!(event.action, "updated");
    assert_eq!(event.source_owner.as_deref(), Some("alice"));
    assert_eq!(event.source_repository.as_deref(), Some("api-fork"));
    assert_eq!(event.commit.as_deref(), Some("fed"));
}

#[test]
fn signed_values_reject_tampering() {
    let signature = sign_hmac_sha256(b"secret", b"1.123");
    assert!(verify_hmac_sha256(b"secret", b"1.123", &signature));
    assert!(!verify_hmac_sha256(b"secret", b"2.123", &signature));
}

#[test]
fn accepts_provider_ping_without_parsing_repository_data() {
    assert_eq!(
        parse_event(GitProviderKind::Github, "ping", b"{}").unwrap(),
        WebhookEvent::Ping
    );
}

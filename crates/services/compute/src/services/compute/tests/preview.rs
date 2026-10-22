use super::create_application;
use crate::services::compute::preview::types::PreviewDeploymentView;
use crate::services::compute::test_support::*;
use crate::services::integrations::git::webhooks::PullRequestEvent;
use crate::utils::provider::GitProviderType;

fn pull_request_event(action: &str, number: &str) -> PullRequestEvent {
    PullRequestEvent {
        provider: GitProviderType::Github,
        owner: "acme".into(),
        repository: "app".into(),
        number: number.into(),
        action: action.into(),
        source_branch: "feature".into(),
        source_owner: Some("acme".into()),
        source_repository: Some("app".into()),
        target_branch: "main".into(),
        commit: Some("abc123".into()),
        author: Some("dev".into()),
    }
}

async fn configure_preview_target(db: &TestDb, application_id: i64, preview_limit: Option<i64>) {
    sqlx::query(
        "UPDATE service_git SET source_type = 'GITHUB', github_provider_id = 1, \
             owner = 'acme', repository = 'app', branch = 'main' \
         WHERE service_id = ?",
    )
    .bind(application_id)
    .execute(&*db.pool)
    .await
    .unwrap();
    sqlx::query(
        "UPDATE service_application SET is_preview_deployments_active = 1, preview_limit = ?, \
             preview_require_collaborator_permissions = 0 \
         WHERE service_id = ?",
    )
    .bind(preview_limit)
    .bind(application_id)
    .execute(&*db.pool)
    .await
    .unwrap();
}

async fn insert_preview(
    db: &TestDb,
    base_service_id: i64,
    preview_service_id: Option<i64>,
    status: &str,
    number: &str,
) -> i64 {
    db.insert_preview(
        base_service_id,
        preview_service_id,
        status,
        number,
        &format!("pr-{number}.example.com"),
    )
    .await
}

async fn preview_status(db: &TestDb, id: i64) -> (String, Option<i64>) {
    sqlx::query_as("SELECT status, preview_service_id FROM preview_deployments WHERE id = ?")
        .bind(id)
        .fetch_one(&*db.pool)
        .await
        .unwrap()
}

#[tokio::test]
async fn list_and_get_views_expose_rows() {
    let db = TestDb::new().await;
    let previews = db.preview();
    let base = create_application(&db, "Preview Base", ENV_ID, Some(SERVER_ID)).await;
    let active = insert_preview(&db, base.id, None, "RUNNING", "12").await;
    let closed = insert_preview(&db, base.id, None, "CLOSED", "13").await;

    let open = previews.list(true).await.unwrap();
    assert_eq!(open.len(), 1);
    assert_eq!(open[0].id, active);
    assert_eq!(open[0].status, "RUNNING");

    let all = previews.list(false).await.unwrap();
    assert_eq!(all.len(), 2);
    let ids = all.iter().map(|view| view.id).collect::<Vec<_>>();
    assert!(ids.contains(&active));
    assert!(ids.contains(&closed));

    let view: PreviewDeploymentView = previews.get(active).await.unwrap();
    assert_eq!(view.base_application_id, base.id);
    assert_eq!(view.preview_application_id, None);
    assert_eq!(view.provider_type, "GITHUB");
    assert_eq!(view.provider_id, 1);
    assert_eq!(view.owner, "acme");
    assert_eq!(view.repository, "app");
    assert_eq!(view.pull_request_number, "12");
    assert_eq!(view.source_branch, "feature");
    assert_eq!(view.source_owner, None);
    assert_eq!(view.source_repository, None);
    assert_eq!(view.target_branch, "main");
    assert_eq!(view.commit_sha, None);
    assert_eq!(view.author, None);
    assert_eq!(view.status, "RUNNING");
    assert_eq!(view.domain, "pr-12.example.com");
    assert_eq!(view.last_deployment_id, None);
    assert!(view.created_at > 0);
    assert!(view.updated_at > 0);

    assert!(matches!(
        previews.get(999).await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn remove_closes_or_deletes_and_guards_missing() {
    let db = TestDb::new().await;
    let previews = db.preview();
    let base = create_application(&db, "Remove Base", ENV_ID, Some(SERVER_ID)).await;

    assert_eq!(
        previews.remove(999, false).await.unwrap_err(),
        "Preview deployment not found"
    );

    let closed_id = insert_preview(&db, base.id, None, "RUNNING", "20").await;
    previews.remove(closed_id, false).await.unwrap();
    let (status, link) = preview_status(&db, closed_id).await;
    assert_eq!(status, "CLOSED");
    assert_eq!(link, None);

    let deleted_id = insert_preview(&db, base.id, None, "RUNNING", "21").await;
    previews.remove(deleted_id, true).await.unwrap();
    assert!(matches!(
        previews.get(deleted_id).await,
        Err(sqlx::Error::RowNotFound)
    ));
}

#[tokio::test]
async fn remove_for_base_application_hard_deletes_open_rows() {
    let db = TestDb::new().await;
    let previews = db.preview();
    let base = create_application(&db, "Bulk Base", ENV_ID, Some(SERVER_ID)).await;
    let other = create_application(&db, "Other Base", ENV_ID, Some(SERVER_ID)).await;
    let first = insert_preview(&db, base.id, None, "RUNNING", "30").await;
    let second = insert_preview(&db, base.id, None, "FAILED", "31").await;
    let already_closed = insert_preview(&db, base.id, None, "CLOSED", "32").await;
    let foreign = insert_preview(&db, other.id, None, "RUNNING", "33").await;

    previews.remove_for_base_application(base.id).await.unwrap();
    assert!(matches!(
        previews.get(first).await,
        Err(sqlx::Error::RowNotFound)
    ));
    assert!(matches!(
        previews.get(second).await,
        Err(sqlx::Error::RowNotFound)
    ));
    assert_eq!(preview_status(&db, already_closed).await.0, "CLOSED");
    assert_eq!(preview_status(&db, foreign).await.0, "RUNNING");
}

#[tokio::test]
async fn ensure_application_can_be_deleted_checks_preview_links() {
    let db = TestDb::new().await;
    let previews = db.preview();
    let base = create_application(&db, "Guard Base", ENV_ID, Some(SERVER_ID)).await;
    let preview_app = create_application(&db, "Guard Preview", ENV_ID, Some(SERVER_ID)).await;
    insert_preview(&db, base.id, Some(preview_app.id), "RUNNING", "40").await;

    assert_eq!(
        previews
            .ensure_application_can_be_deleted(preview_app.id)
            .await
            .unwrap_err(),
        "Preview applications must be deleted through the preview deployment API"
    );
    previews
        .ensure_application_can_be_deleted(base.id)
        .await
        .unwrap();
    previews
        .ensure_application_can_be_deleted(999)
        .await
        .unwrap();
}

#[tokio::test]
async fn handle_pull_request_close_actions_cleanup_open_previews() {
    let db = TestDb::new().await;
    let previews = db.preview();
    let base = create_application(&db, "Close Base", ENV_ID, Some(SERVER_ID)).await;
    let merged = insert_preview(&db, base.id, None, "RUNNING", "50").await;
    let closed = insert_preview(&db, base.id, None, "RUNNING", "51").await;
    let untouched = insert_preview(&db, base.id, None, "RUNNING", "52").await;

    let outcome = previews
        .handle_pull_request(&pull_request_event("merged", "50"))
        .await
        .unwrap();
    assert_eq!(outcome.removed, 1);
    assert_eq!(preview_status(&db, merged).await.0, "CLOSED");
    assert_eq!(preview_status(&db, untouched).await.0, "RUNNING");

    let mut event = pull_request_event("closed", "51");
    event.owner = "ACME".into();
    event.repository = "APP".into();
    let outcome = previews.handle_pull_request(&event).await.unwrap();
    assert_eq!(outcome.removed, 1);
    assert_eq!(preview_status(&db, closed).await.0, "CLOSED");

    let outcome = previews
        .handle_pull_request(&pull_request_event("closed", "999"))
        .await
        .unwrap();
    assert_eq!(outcome.removed, 0);
    assert_eq!(preview_status(&db, untouched).await.0, "RUNNING");
}

#[tokio::test]
async fn deploy_application_rejects_unconfigured_targets_and_limits() {
    let db = TestDb::new().await;
    let previews = db.preview();
    let base = create_application(&db, "Deploy Base", ENV_ID, Some(SERVER_ID)).await;

    let error = previews
        .deploy_application(base.id, &pull_request_event("opened", "60"))
        .await
        .unwrap_err();
    assert_eq!(
        error,
        "Application is not configured for this pull request preview"
    );

    configure_preview_target(&db, base.id, Some(1)).await;
    insert_preview(&db, base.id, None, "RUNNING", "61").await;
    let error = previews
        .deploy_application(base.id, &pull_request_event("opened", "62"))
        .await
        .unwrap_err();
    assert_eq!(error, "Preview deployment limit reached");

    let mut event = pull_request_event("opened", "63");
    event.repository = "other".into();
    let error = previews
        .deploy_application(base.id, &event)
        .await
        .unwrap_err();
    assert_eq!(
        error,
        "Application is not configured for this pull request preview"
    );
}

#[tokio::test]
async fn redeploy_rejects_missing_closed_unsupported_and_unconfigured_rows() {
    let db = TestDb::new().await;
    let previews = db.preview();
    let base = create_application(&db, "Redeploy Base", ENV_ID, Some(SERVER_ID)).await;

    assert_eq!(
        previews.redeploy(999).await.unwrap_err(),
        "Preview deployment not found"
    );

    let closed = insert_preview(&db, base.id, None, "CLOSED", "70").await;
    assert_eq!(
        previews.redeploy(closed).await.unwrap_err(),
        "Closed preview deployment cannot be redeployed"
    );

    let unsupported = insert_preview(&db, base.id, None, "RUNNING", "71").await;
    sqlx::query("UPDATE preview_deployments SET provider_type = 'FOO' WHERE id = ?")
        .bind(unsupported)
        .execute(&*db.pool)
        .await
        .unwrap();
    assert_eq!(
        previews.redeploy(unsupported).await.unwrap_err(),
        "Unsupported preview provider"
    );

    let open = insert_preview(&db, base.id, None, "RUNNING", "72").await;
    assert_eq!(
        previews.redeploy(open).await.unwrap_err(),
        "Application is not configured for this pull request preview"
    );
}

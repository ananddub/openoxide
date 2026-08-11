/// Route Integration Test
/// Tests html! macro with the exact pattern used in rustploy controllers
/// Runs a real axum server and makes HTTP requests to it.
use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use html_macro::html;

// ── Fake DB types (no actual DB needed) ──────────────────────────────────────

#[derive(Clone)]
struct App {
    #[allow(dead_code)]
    id: i64,
    name: String,
    status: String,
}

async fn db_get_app(id: i64) -> Option<App> {
    if id == 1 {
        Some(App {
            id,
            name: "my-app".into(),
            status: "running".into(),
        })
    } else {
        None
    }
}

async fn db_get_apps() -> Vec<Option<App>> {
    vec![
        Some(App {
            id: 1,
            name: "app-alpha".into(),
            status: "running".into(),
        }),
        Some(App {
            id: 2,
            name: "app-beta".into(),
            status: "stopped".into(),
        }),
        Some(App {
            id: 3,
            name: "app-gamma".into(),
            status: "running".into(),
        }),
    ]
}

// ── Controller-style handler ──────────────────────────────────────────────────

async fn app_detail_page(Path(id): Path<i64>) -> impl IntoResponse {
    html! {
        <!DOCTYPE html>
        <html>
        <head><title>"App Detail"</title></head>
        <body>
            <div signals={ is_modal_open: false }>
                <button toggle:is_modal_open>"Toggle Modal"</button>

                // Single async app load
                @if let Some(app) = db_get_app(id).await {
                    <h1>{&app.name}</h1>
                    <span class="status">{&app.status}</span>
                    <button on:click={"/applications/restart"}>"Restart"</button>
                } @else {
                    <div class="skeleton">"Loading app..."</div>
                }

                // Async list of all apps with per-item skeleton
                <section>
                    @for app in db_get_apps().await {
                        @if let Some(a) = app {
                            <div class="card">{&a.name}</div>
                        } @else {
                            <div class="skeleton-card">"Loading..."</div>
                        }
                    }
                </section>

                <div show:is_modal_open class="modal">
                    "Modal Content"
                </div>
            </div>
        </body>
        </html>
    }
}

async fn app_list_page() -> impl IntoResponse {
    html! {
        <!DOCTYPE html>
        <html>
        <head><title>"Apps"</title></head>
        <body>
            <h1>"Applications"</h1>
            @for app in db_get_apps().await {
                @if let Some(a) = app {
                    <div class="app-row">
                        <span>{&a.name}</span>
                        <span class="badge">{&a.status}</span>
                    </div>
                } @else {
                    <div class="skeleton-row">"Loading..."</div>
                }
            }
        </body>
        </html>
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_app_detail_page_initial_render() {
    use axum::body::to_bytes;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let app = Router::new().route("/apps/{id}", get(app_detail_page));

    // GET /apps/1 — existing app
    let response = app
        .clone()
        .oneshot(
            Request::get("/apps/1")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let html = String::from_utf8_lossy(&body);

    println!("\n=== GET /apps/1 ===\n{}\n", &html);

    // Slot div should be present (async block)
    assert!(html.contains("__slot_"), "slot div missing");
    // Skeleton shown initially (async not resolved yet in initial render)
    assert!(html.contains("Loading app..."), "skeleton missing");
    // signals attribute
    assert!(html.contains("data-signals="), "signals missing");
    // toggle button
    assert!(html.contains("data-on:click="), "toggle missing");
    // show modal
    assert!(html.contains("data-show="), "show missing");
    // DOCTYPE
    assert!(html.contains("<!DOCTYPE html>"), "doctype missing");
}

#[tokio::test]
async fn test_app_list_page_initial_render() {
    use axum::body::to_bytes;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    let app = Router::new().route("/apps", get(app_list_page));

    let response = app
        .oneshot(
            Request::get("/apps")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let html = String::from_utf8_lossy(&body);

    println!("\n=== GET /apps ===\n{}\n", &html);

    assert!(html.contains("Applications"), "title missing");
    assert!(html.contains("__slot_"), "for async slot missing");
}

#[tokio::test]
async fn test_async_patch_received() {
    let markup = html! {
        @for app in db_get_apps().await {
            @if let Some(a) = app {
                <div>{&a.name}</div>
            } @else {
                <div>"Loading..."</div>
            }
        }
    };

    assert!(markup.0.contains("__slot_"));
    assert!(markup.0.contains("/_rustploy/html/events/"));
}

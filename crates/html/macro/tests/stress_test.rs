/// Comprehensive Stress & Edge Case Test Suite for html-macro
/// Tests multiple async blocks, nested async, html escaping, control flows,
/// matching, attribute variants, and error conditions.
use axum::response::IntoResponse;
use html_macro::html;
use html_rt::Markup;

// ── Structs for testing ──────────────────────────────────────────────────────

#[allow(dead_code)]
struct User {
    id: u64,
    name: String,
    role: String,
}

#[allow(dead_code)]
struct Project {
    id: u64,
    title: String,
}

enum Status {
    Active,
    Pending(String),
    Inactive,
}

// ── Mock Async Functions ─────────────────────────────────────────────────────

async fn fetch_user(id: u64) -> Option<User> {
    if id == 100 || id == 200 {
        Some(User {
            id,
            name: format!("User {}", id),
            role: "Admin".into(),
        })
    } else {
        None
    }
}

async fn fetch_projects(_user_id: u64) -> Vec<Option<Project>> {
    vec![
        Some(Project {
            id: 1,
            title: "Alpha & Omega".into(),
        }),
        Some(Project {
            id: 2,
            title: "Project <X>".into(),
        }),
        None, // Loading or placeholder item
    ]
}

// ── Test 1: Multiple Async Slots in a single template ─────────────────────────

#[tokio::test]
async fn test_multiple_async_slots_uniqueness() {
    let markup = html! {
        <div>
            // Slot 1
            @if let Some(u) = fetch_user(100).await {
                <span>{&u.name}</span>
            } @else {
                <span>"Loading user 100..."</span>
            }

            // Slot 2
            @for p in fetch_projects(100).await {
                @if let Some(proj) = p {
                    <div>{&proj.title}</div>
                } @else {
                    <div>"Skeleton..."</div>
                }
            }

            // Slot 3 (User 200 returns Some so async task resolves and patches)
            @if let Some(u2) = fetch_user(200).await {
                <span>{&u2.name}</span>
            } @else {
                <span>"Loading user 200..."</span>
            }
        </div>
    };

    let html_str = markup.0;

    // Must have 3 distinct slot IDs
    assert!(html_str.contains("id=\"__slot_"), "Slot div missing");

    assert_eq!(html_str.matches("id=\"__slot_").count(), 3);
    assert!(html_str.contains("/_rustploy/html/events/"));
}

// ── Test 2: HTML Escaping Safety ──────────────────────────────────────────────

#[test]
fn test_html_escaping_security() {
    let malicious_input = "<script>alert('xss & attack')</script>";
    let m = html! {
        <div class="user-content">
            <p>{malicious_input}</p>
            <input value={malicious_input} />
        </div>
    };

    let html = m.0;
    println!("Escaped HTML: {}", html);

    // Verify raw <script> is NOT present
    assert!(
        !html.contains("<script>"),
        "XSS vulnerability: unescaped script tag!"
    );
    assert!(
        html.contains("&lt;script&gt;"),
        "Script tag not properly escaped"
    );
    assert!(html.contains("&amp;"), "Ampersand not properly escaped");
}

// ── Test 3: Complex Control Flow (@if, @else if, @else, @match) ───────────────

#[test]
fn test_complex_control_flow() {
    let status = Status::Pending("verification required".into());
    let _active = Status::Active;
    let _inactive = Status::Inactive;
    let items = vec!["one", "two", "three"];

    let m = html! {
        <div class="status-box">
            @match &status {
                Status::Active => { <span class="green">"Active"</span> },
                Status::Pending(reason) => {
                    <span class="yellow">"Pending: "{reason}</span>
                },
                Status::Inactive => { <span class="red">"Inactive"</span> },
            }

            @if items.len() > 5 {
                <p>"Many items"</p>
            } @else if items.len() == 3 {
                <p>"Exactly 3 items"</p>
            } @else {
                <p>"Few items"</p>
            }

            <ul>
                @for (idx, item) in items.iter().enumerate() {
                    <li data-index={idx}>{item}</li>
                }
            </ul>
        </div>
    };

    let html = m.0;
    println!("Control Flow Output: {}", html);

    assert!(html.contains("Pending: verification required"));
    assert!(html.contains("Exactly 3 items"));
    assert!(html.contains("data-index=\"0\""));
    assert!(html.contains("data-index=\"2\""));
}

// ── Test 4: Axum IntoResponse Headers & Content-Type ───────────────────────────

#[tokio::test]
async fn test_axum_into_response_content_type() {
    use axum::body::to_bytes;
    use axum::http::header::CONTENT_TYPE;

    let m: Markup = html! {
        <!DOCTYPE html>
        <html><body>"Hello World"</body></html>
    };

    let response = m.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .expect("Content-Type header missing");
    assert_eq!(content_type.to_str().unwrap(), "text/html; charset=utf-8");

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8_lossy(&bytes);
    assert_eq!(
        body_str,
        "<!DOCTYPE html><html><body>Hello World</body></html>"
    );
}

// ── Test 5: All Datastar Shorthands Combined ───────────────────────────────────

#[test]
fn test_all_datastar_shorthands_combined() {
    struct TestController;
    impl TestController {
        #[allow(non_upper_case_globals, dead_code)]
        pub const __PATH_submit: &'static str = "/api/submit";
    }

    let m = html! {
        <form signals={ username: "", count: 0, is_valid: false }
              bind:username=""
              on:submit={TestController::submit}>

            <input bind:count="" />
            <button toggle:is_valid>"Toggle Valid"</button>

            <div show:is_valid class="success">"Valid!"</div>
            <div hide:is_valid class="error">"Invalid!"</div>
        </form>
    };

    let html = m.0;
    println!("Datastar Form HTML: {}", html);

    assert!(html.contains("data-signals="));
    assert!(html.contains("data-bind:username"));
    assert!(html.contains("data-bind:count"));
    assert!(html.contains("data-on:submit=\"@post('/api/submit')\""));
    assert!(html.contains("data-on:click=\"$is_valid = !$is_valid\""));
    assert!(html.contains("data-show=\"$is_valid\""));
    assert!(html.contains("data-show=\"!$is_valid\""));
}

use html_macro::html;

#[tokio::test]
async fn test_async_if_let_some_slot_generated() {
    async fn get_user(id: u32) -> Option<String> {
        Some(format!("User_{}", id))
    }

    let markup = html! {
        <div>
            @if let Some(user) = get_user(42).await {
                <span>{&user}</span>
            } @else {
                <div class="skeleton">"Loading..."</div>
            }
        </div>
    };

    let html_str = markup.0;
    assert!(
        html_str.contains("id=\"__slot_"),
        "slot div missing: {}",
        html_str
    );
    assert!(
        html_str.contains("Loading...") || html_str.contains("skeleton"),
        "skeleton not in initial render: {}",
        html_str
    );

    assert!(html_str.contains("/_rustploy/html/events/"));
}

#[tokio::test]
async fn test_async_for_simple() {
    async fn get_items() -> Vec<String> {
        vec!["Alpha".into(), "Beta".into()]
    }

    let markup = html! {
        <ul>
            @for item in get_items().await {
                <li>{&item}</li>
            }
        </ul>
    };

    let html_str = markup.0;
    assert!(
        html_str.contains("id=\"__slot_"),
        "slot div missing: {}",
        html_str
    );

    assert!(html_str.contains("/_rustploy/html/events/"));
}

#[tokio::test]
async fn test_async_table_hint_indexing() {
    async fn get_apps() -> Vec<String> {
        vec!["App1".into(), "App2".into()]
    }

    async fn get_user(id: u32) -> Option<String> {
        Some(format!("User_{}", id))
    }

    let markup = html! {
        <div>
            @for app in get_apps().await["applications"] {
                <div>{&app}</div>
            }

            @if let Some(user) = get_user(10).await["users"] {
                <span>{&user}</span>
            } @else {
                <div>"Loading..."</div>
            }
        </div>
    };

    let html_str = markup.0;
    assert!(html_str.contains("id=\"__slot_"));

    assert!(html_str.contains("/_rustploy/html/events/"));
}

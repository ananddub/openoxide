use html_macro::html;

#[test]
fn test_signals_attr() {
    let m = html! {
        <div signals={ is_open: false, count: 0 }></div>
    };
    let html = m.0;
    assert!(html.contains("data-signals="), "signals: {}", html);
    assert!(html.contains("is_open"), "is_open: {}", html);
}

#[test]
fn test_toggle_show_hide() {
    let m = html! {
        <div>
            <button toggle:is_open>"T"</button>
            <div show:is_open>"S"</div>
            <div hide:is_open>"H"</div>
        </div>
    };
    let html = m.0;
    assert!(
        html.contains("data-on:click=\"$is_open = !$is_open\""),
        "toggle: {}",
        html
    );
    assert!(html.contains("data-show=\"$is_open\""), "show: {}", html);
    assert!(html.contains("data-show=\"!$is_open\""), "hide: {}", html);
}

#[test]
fn test_bind_attr() {
    let m = html! {
        <input bind:username="" />
    };
    let html = m.0;
    assert!(html.contains("data-bind:username"), "bind: {}", html);
}

#[test]
fn test_on_click() {
    const MY_PATH: &str = "/apps/restart";
    let m = html! {
        <button on:click={MY_PATH}>"Restart"</button>
    };
    let html = m.0;
    assert!(html.contains("data-on:click"), "on:click: {}", html);
    assert!(html.contains("/apps/restart"), "path: {}", html);
}

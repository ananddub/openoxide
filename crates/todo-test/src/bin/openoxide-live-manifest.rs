use openoxide_todo_test::{controller::todo_live, models::Todo};

fn main() {
    let subscription = todo_live::list_subscription().expect("generate live subscription");
    let hook = format!("use{}", pascal_case(subscription.client_name()));
    println!(
        "{}",
        serde_json::json!({
            "types": [{"name": "Todo", "definition": Todo::TYPESCRIPT}],
            "endpoints": [{
                "hook": hook,
                "namespace": subscription.namespace(),
                "endpoint": subscription.endpoint(),
                "event": subscription.event(),
                "parameters": "",
                "result": "Todo[]"
            }]
        })
    );
}

fn pascal_case(value: &str) -> String {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut characters = part.chars();
            characters
                .next()
                .map(|first| first.to_ascii_uppercase().to_string() + characters.as_str())
                .unwrap_or_default()
        })
        .collect()
}

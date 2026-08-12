use openoxide_todo_test::{
    controller::TodoController,
    models::{ActivityEvent, MetricSample, Todo},
};

fn main() {
    let subscription = TodoController::todos_subscription().expect("generate live subscription");
    let metrics = TodoController::metrics_subscription().expect("generate metrics subscription");
    let activity = TodoController::activity_subscription().expect("generate activity subscription");
    println!(
        "{}",
        serde_json::json!({
            "types": [
                {"name": "Todo", "definition": Todo::TYPESCRIPT},
                {"name": "MetricSample", "definition": MetricSample::TYPESCRIPT},
                {"name": "ActivityEvent", "definition": ActivityEvent::TYPESCRIPT}
            ],
            "endpoints": [{
                "hook": format!("use{}", pascal_case(subscription.client_name())),
                "namespace": subscription.namespace(),
                "endpoint": subscription.endpoint(),
                "event": subscription.event(),
                "parameters": "",
                "result": "Todo[]"
            }, {
                "hook": format!("use{}", pascal_case(metrics.client_name())),
                "namespace": metrics.namespace(), "endpoint": metrics.endpoint(), "event": metrics.event(),
                "parameters": "", "result": "MetricSample"
            }, {
                "hook": format!("use{}", pascal_case(activity.client_name())),
                "namespace": activity.namespace(), "endpoint": activity.endpoint(), "event": activity.event(),
                "parameters": "", "result": "ActivityEvent"
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

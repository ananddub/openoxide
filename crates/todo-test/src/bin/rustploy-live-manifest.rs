use rustploy_todo_test::{controller::todo_live, models::Todo};

fn main() {
    let subscription = todo_live::list_subscription().expect("generate live subscription");
    println!(
        "{}",
        serde_json::json!({
            "types": [{"name": "Todo", "definition": Todo::TYPESCRIPT}],
            "endpoints": [{
                "hook": "useTodos",
                "namespace": subscription.namespace(),
                "endpoint": subscription.endpoint(),
                "event": subscription.event(),
                "parameters": "",
                "result": "Todo[]"
            }]
        })
    );
}

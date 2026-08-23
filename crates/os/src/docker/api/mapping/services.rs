fn service_create(args: &[String]) -> Option<ApiRequest> {
    let image = positional_arg(args, 2, &["--name", "--replicas", "--env"])?;
    let mut body = serde_json::json!({"Name": "", "TaskTemplate": {"ContainerSpec": {"Image": image, "Env": []}}, "Mode": {"Replicated": {"Replicas": 1}}});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--name" => body["Name"] = serde_json::json!(pair[1]),
            "--replicas" => {
                body["Mode"]["Replicated"]["Replicas"] =
                    serde_json::json!(pair[1].parse::<u64>().ok()?)
            }
            "--env" => body["TaskTemplate"]["ContainerSpec"]["Env"]
                .as_array_mut()?
                .push(serde_json::json!(pair[1])),
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: "/services/create".into(),
        body: Some(body),
        response_shape: ResponseShape::CreatedId,
        start_created: false,
    })
}

fn service_update(args: &[String]) -> Option<ApiRequest> {
    let id = positional_arg(
        args,
        2,
        &[
            "--image",
            "--replicas",
            "--env-add",
            "--label-add",
            "--limit-memory",
        ],
    )?;
    let mut body = serde_json::json!({"TaskTemplate": {"ContainerSpec": {}}});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--image" => {
                body["TaskTemplate"]["ContainerSpec"]["Image"] = serde_json::json!(pair[1])
            }
            "--replicas" => {
                body["Mode"] =
                    serde_json::json!({"Replicated": {"Replicas": pair[1].parse::<u64>().ok()?}})
            }
            "--env-add" => body["TaskTemplate"]["ContainerSpec"]["Env"]
                .as_array_mut()?
                .push(serde_json::json!(pair[1])),
            "--label-add" => {
                let (k, v) = pair[1].split_once('=')?;
                body["Labels"][k] = serde_json::json!(v);
            }
            "--limit-memory" => {
                body["TaskTemplate"]["Resources"] =
                    serde_json::json!({"Limits": {"MemoryBytes": parse_bytes(&pair[1])}})
            }
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: format!("/services/{id}/update?version=0"),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn service_scale(args: &[String]) -> Option<ApiRequest> {
    let item = args.iter().skip(2).find(|arg| arg.contains('='))?;
    let (id, replicas) = item.split_once('=')?;
    Some(ApiRequest {
        method: "POST",
        path: format!("/services/{id}/update?version=0"),
        body: Some(
            serde_json::json!({"Mode": {"Replicated": {"Replicas": replicas.parse::<u64>().ok()?}}}),
        ),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn resource_action(args: &[String], prefix: &str, action: &str) -> Option<ApiRequest> {
    let id = args.get(2)?;
    Some(ApiRequest {
        method: "POST",
        path: format!("{prefix}/{id}/{action}"),
        body: Some(serde_json::json!({})),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn service_subcommand(args: &[String]) -> Option<ApiRequest> {
    let id = args.get(2)?;
    let action = args.get(1)?;
    let endpoint = if action == "ps" { "tasks" } else { "logs" };
    Some(ApiRequest {
        method: "GET",
        path: format!("/services/{id}/{endpoint}"),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn node_role(args: &[String]) -> Option<ApiRequest> {
    let id = args.get(2)?;
    let role = if args[1] == "promote" {
        "manager"
    } else {
        "worker"
    };
    Some(ApiRequest {
        method: "POST",
        path: format!("/nodes/{id}/update?version=0"),
        body: Some(serde_json::json!({"Role": role})),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn node_update(args: &[String]) -> Option<ApiRequest> {
    let id = args.get(2)?;
    let mut body = serde_json::json!({});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--availability" => body["Availability"] = serde_json::json!(pair[1]),
            "--role" => body["Role"] = serde_json::json!(pair[1]),
            "--label-add" => {
                let (k, v) = pair[1].split_once('=')?;
                body["Labels"][k] = serde_json::json!(v);
            }
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: format!("/nodes/{id}/update?version=0"),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn node_tasks(args: &[String]) -> Option<ApiRequest> {
    let id = args.get(2)?;
    let filters = args
        .windows(2)
        .filter_map(|pair| (pair[0] == "--filter" || pair[0] == "-f").then(|| pair[1].clone()))
        .collect::<Vec<_>>();
    let suffix = if filters.is_empty() {
        String::new()
    } else {
        format!(
            "?filters={}",
            encode_query(&serde_json::json!({"desired-state": filters}).to_string())
        )
    };
    Some(ApiRequest {
        method: "GET",
        path: format!("/nodes/{id}/tasks{suffix}"),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}


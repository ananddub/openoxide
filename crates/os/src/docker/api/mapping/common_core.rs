fn get(path: &str) -> Option<ApiRequest> {
    Some(ApiRequest {
        method: "GET",
        path: path.into(),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn list_with_flags(args: &[String], path: &str, supports_all: bool) -> Option<ApiRequest> {
    let mut query = Vec::new();
    if supports_all && args.iter().any(|arg| arg == "--all" || arg == "-a") {
        query.push(("all", "true".to_string()));
    }
    let mut filters = serde_json::Map::new();
    for pair in args.windows(2) {
        if pair[0] == "--filter" || pair[0] == "-f" {
            let (key, value) = pair[1]
                .split_once('=')
                .unwrap_or(("name", pair[1].as_str()));
            let values = filters
                .entry(key.to_owned())
                .or_insert_with(|| serde_json::json!([]));
            values.as_array_mut()?.push(serde_json::json!(value));
        }
    }
    if !filters.is_empty() {
        query.push(("filters", serde_json::to_string(&filters).ok()?));
    }
    let suffix = query
        .into_iter()
        .map(|(key, value)| format!("{key}={}", encode_query(&value)))
        .collect::<Vec<_>>()
        .join("&");
    Some(ApiRequest {
        method: "GET",
        path: if suffix.is_empty() {
            path.into()
        } else {
            format!("{path}?{suffix}")
        },
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn id_op(
    args: &[String],
    method: &'static str,
    suffix: &str,
    shape: ResponseShape,
) -> Option<ApiRequest> {
    Some(ApiRequest {
        method,
        path: format!("/containers/{}{}", args.get(2)?, suffix),
        body: None,
        response_shape: shape,
        start_created: false,
    })
}

fn container_logs(args: &[String]) -> Option<ApiRequest> {
    let id = positional_arg(args, 2, &["--tail", "--since", "--until"])?;
    let mut query = vec![
        ("stdout", "true".to_string()),
        ("stderr", "true".to_string()),
    ];
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--follow" => query.push(("follow", "true".into())),
            "--timestamps" => query.push(("timestamps", "true".into())),
            "--tail" => query.push(("tail", pair[1].clone())),
            "--since" => query.push(("since", pair[1].clone())),
            "--until" => query.push(("until", pair[1].clone())),
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "GET",
        path: format!(
            "/containers/{}/logs?{}",
            encode_query(&id),
            query_string(query)
        ),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn system_events(args: &[String]) -> Option<ApiRequest> {
    let mut query = Vec::new();
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--since" => query.push(("since", pair[1].clone())),
            "--until" => query.push(("until", pair[1].clone())),
            "--filter" | "-f" => query.push(("filters", pair[1].clone())),
            _ => {}
        }
    }
    let suffix = if query.is_empty() {
        String::new()
    } else {
        format!(
            "?{}",
            query
                .iter()
                .map(|(k, v)| format!("{k}={}", encode_query(v)))
                .collect::<Vec<_>>()
                .join("&")
        )
    };
    Some(ApiRequest {
        method: "GET",
        path: format!("/events{suffix}"),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn registry_login(args: &[String]) -> Option<ApiRequest> {
    let mut body = serde_json::json!({});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--username" | "-u" => body["username"] = serde_json::json!(pair[1]),
            "--password" | "-p" => body["password"] = serde_json::json!(pair[1]),
            value if !value.starts_with('-') && pair[1].starts_with('-') => {
                body["serveraddress"] = serde_json::json!(value)
            }
            _ => {}
        }
    }
    if let Some(registry) = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with('-') && *arg != "login")
    {
        body["serveraddress"] = serde_json::json!(registry);
    }
    Some(ApiRequest {
        method: "POST",
        path: "/auth".into(),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn container_remove(args: &[String]) -> Option<ApiRequest> {
    let id = positional_arg(args, 2, &[])?;
    let path = format!(
        "/containers/{}?force={}&v={}&link={}",
        encode_query(&id),
        args.iter().any(|v| v == "--force"),
        args.iter().any(|v| v == "--volumes" || v == "-v"),
        args.iter().any(|v| v == "--link")
    );
    Some(ApiRequest {
        method: "DELETE",
        path,
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn container_stats(args: &[String]) -> Option<ApiRequest> {
    let id = positional_arg(args, 2, &["--format"])?;
    let stream = if args.iter().any(|arg| arg == "--no-stream") {
        "false"
    } else {
        "true"
    };
    Some(ApiRequest {
        method: "GET",
        path: format!("/containers/{id}/stats?stream={stream}"),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn container_rename(args: &[String]) -> Option<ApiRequest> {
    let id = args.get(2)?;
    let name = args.get(3)?;
    Some(ApiRequest {
        method: "POST",
        path: format!("/containers/{id}/rename?name={}", encode_query(name)),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}


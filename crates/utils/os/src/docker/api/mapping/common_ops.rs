fn container_update(args: &[String]) -> Option<ApiRequest> {
    let id = positional_arg(args, 2, &["--memory", "--cpus", "--restart"])?;
    let mut body = serde_json::json!({});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--memory" => body["Memory"] = serde_json::json!(parse_bytes(&pair[1])),
            "--cpus" => {
                body["NanoCpus"] = serde_json::json!(
                    (pair[1].parse::<f64>().unwrap_or(0.0) * 1_000_000_000.0) as i64
                )
            }
            "--restart" => body["RestartPolicy"] = restart_policy(&pair[1]),
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: format!("/containers/{}/update", encode_query(&id)),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn network_membership(args: &[String]) -> Option<ApiRequest> {
    let network = args.get(2)?;
    let container = args.get(3)?;
    let action = args.get(1)?;
    let body = if action == "disconnect" {
        serde_json::json!({"Container": container, "Force": false})
    } else {
        serde_json::json!({"Container": container, "EndpointConfig": {}})
    };
    Some(ApiRequest {
        method: "POST",
        path: format!("/networks/{}/{}", encode_query(network), action),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn image_tag(args: &[String]) -> Option<ApiRequest> {
    let source = args.get(2)?;
    let target = args.get(3)?;
    let (repo, tag) = target.split_once(':').unwrap_or((target, "latest"));
    Some(ApiRequest {
        method: "POST",
        path: format!(
            "/images/{}/tag?repo={}&tag={}",
            encode_query(source),
            encode_query(repo),
            encode_query(tag)
        ),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn prune(path: &str, args: &[String]) -> Option<ApiRequest> {
    let filters = args
        .windows(2)
        .filter(|&pair| pair[0] == "--filter" || pair[0] == "-f").map(|pair| pair[1].clone())
        .collect::<Vec<_>>();
    let suffix = if filters.is_empty() {
        String::new()
    } else {
        format!(
            "?filters={}",
            encode_query(&format!("{{\"until\":[\"{}\"]}}", filters.join("\",\"")))
        )
    };
    Some(ApiRequest {
        method: "POST",
        path: format!("{path}{suffix}"),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn query_string(query: Vec<(&str, String)>) -> String {
    query
        .into_iter()
        .map(|(key, value)| format!("{key}={}", encode_query(&value)))
        .collect::<Vec<_>>()
        .join("&")
}


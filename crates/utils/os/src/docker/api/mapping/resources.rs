fn volume_create(args: &[String]) -> Option<ApiRequest> {
    let name = positional_arg(args, 2, &["--driver", "--label", "--opt"])?;
    let mut body =
        serde_json::json!({"Name": name, "Driver": "local", "Labels": {}, "DriverOpts": {}});
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--driver" => {
                body["Driver"] = serde_json::json!(args.get(i + 1)?);
                i += 2;
            }
            "--label" => {
                let (k, v) = args.get(i + 1)?.split_once('=')?;
                body["Labels"][k] = serde_json::json!(v);
                i += 2;
            }
            "--opt" => {
                let (k, v) = args.get(i + 1)?.split_once('=')?;
                body["DriverOpts"][k] = serde_json::json!(v);
                i += 2;
            }
            _ => i += 1,
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: "/volumes/create".into(),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn network_create(args: &[String]) -> Option<ApiRequest> {
    let name = positional_arg(
        args,
        2,
        &[
            "--driver",
            "--subnet",
            "--gateway",
            "--label",
            "--opt",
            "--scope",
        ],
    )?;
    let mut body = serde_json::json!({"Name": name, "Driver": "bridge", "CheckDuplicate": true, "Internal": false, "Attachable": false, "Ingress": false, "EnableIPv6": false, "Labels": {}, "Options": {}, "IPAM": {"Driver": "default", "Config": []}});
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--driver" => {
                body["Driver"] = serde_json::json!(args.get(i + 1)?);
                i += 2;
            }
            "--subnet" => {
                body["IPAM"]["Config"]
                    .as_array_mut()?
                    .push(serde_json::json!({"Subnet": args.get(i+1)?}));
                i += 2;
            }
            "--gateway" => {
                let config = body["IPAM"]["Config"].as_array_mut()?;
                if config.is_empty() {
                    config.push(serde_json::json!({}));
                }
                config[0]["Gateway"] = serde_json::json!(args.get(i + 1)?);
                i += 2;
            }
            "--label" => {
                let (k, v) = args.get(i + 1)?.split_once('=')?;
                body["Labels"][k] = serde_json::json!(v);
                i += 2;
            }
            "--opt" => {
                let (k, v) = args.get(i + 1)?.split_once('=')?;
                body["Options"][k] = serde_json::json!(v);
                i += 2;
            }
            "--attachable" => {
                body["Attachable"] = serde_json::json!(true);
                i += 1;
            }
            "--internal" => {
                body["Internal"] = serde_json::json!(true);
                i += 1;
            }
            "--ipv6" => {
                body["EnableIPv6"] = serde_json::json!(true);
                i += 1;
            }
            _ => i += 1,
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: "/networks/create".into(),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn image_pull(args: &[String]) -> Option<ApiRequest> {
    let image = positional_arg(args, 2, &["--platform"])?;
    let mut path = format!("/images/create?fromImage={}", encode_query(&image));
    for pair in args.windows(2) {
        if pair[0] == "--platform" {
            path.push_str(&format!("&platform={}", encode_query(&pair[1])));
        }
    }
    Some(ApiRequest {
        method: "POST",
        path,
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn image_push(args: &[String]) -> Option<ApiRequest> {
    let image = positional_arg(args, 2, &[])?;
    Some(ApiRequest {
        method: "POST",
        path: format!("/images/{}/push", encode_query(&image)),
        body: None,
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}


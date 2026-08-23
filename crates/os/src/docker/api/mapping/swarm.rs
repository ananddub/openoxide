fn swarm_init(args: &[String]) -> Option<ApiRequest> {
    let mut body = serde_json::json!({"ListenAddr": "0.0.0.0:2377", "ForceNewCluster": args.iter().any(|v| v == "--force-new-cluster")});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--advertise-addr" => body["AdvertiseAddr"] = serde_json::json!(pair[1]),
            "--listen-addr" => body["ListenAddr"] = serde_json::json!(pair[1]),
            "--data-path-port" => {
                body["DataPathPort"] = serde_json::json!(pair[1].parse::<u16>().ok()?)
            }
            "--default-addr-pool" => body["DefaultAddrPool"] = serde_json::json!([pair[1].clone()]),
            "--default-addr-pool-mask-length" => {
                body["SubnetSize"] = serde_json::json!(pair[1].parse::<u8>().ok()?)
            }
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: "/swarm/init".into(),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn swarm_join(args: &[String]) -> Option<ApiRequest> {
    let remote = positional_arg(
        args,
        2,
        &[
            "--token",
            "--advertise-addr",
            "--listen-addr",
            "--data-path-port",
        ],
    )?;
    let mut body = serde_json::json!({"ListenAddr": "0.0.0.0:2377", "RemoteAddrs": [remote]});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--token" => body["JoinToken"] = serde_json::json!(pair[1]),
            "--advertise-addr" => body["AdvertiseAddr"] = serde_json::json!(pair[1]),
            "--listen-addr" => body["ListenAddr"] = serde_json::json!(pair[1]),
            "--data-path-port" => {
                body["DataPathPort"] = serde_json::json!(pair[1].parse::<u16>().ok()?)
            }
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: "/swarm/join".into(),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn swarm_unlock(args: &[String]) -> Option<ApiRequest> {
    let key = args.iter().skip(2).find(|arg| !arg.starts_with('-'))?;
    Some(ApiRequest {
        method: "POST",
        path: "/swarm/unlock".into(),
        body: Some(serde_json::json!({"UnlockKey": key})),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}

fn swarm_update(args: &[String]) -> Option<ApiRequest> {
    let mut body = serde_json::json!({});
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--autolock" => body["AutolockManagers"] = serde_json::json!(pair[1] == "true"),
            "--task-history-limit" => {
                body["TaskHistoryRetentionLimit"] = serde_json::json!(pair[1].parse::<u64>().ok()?)
            }
            "--snapshot-interval" => {
                body["SnapshotInterval"] = serde_json::json!(pair[1].parse::<u64>().ok()?)
            }
            "--dispatcher-heartbeat" => {
                body["DispatcherHeartbeatPeriod"] = serde_json::json!(parse_duration_ns(&pair[1]));
            }
            _ => {}
        }
    }
    Some(ApiRequest {
        method: "POST",
        path: "/swarm/update?version=0".into(),
        body: Some(body),
        response_shape: ResponseShape::Raw,
        start_created: false,
    })
}


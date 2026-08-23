fn container_create(args: &[String]) -> Option<ApiRequest> {
    let mut config = serde_json::json!({"Env": [], "Labels": {}, "Cmd": [], "ExposedPorts": {}});
    let mut host = serde_json::json!({"Binds": [], "Mounts": [], "PortBindings": {}, "NetworkMode": "default", "CapAdd": [], "CapDrop": [], "Dns": [], "SecurityOpt": [], "Devices": []});
    let mut env_files = Vec::new();
    let mut name = None;
    let mut platform = None;
    let mut network = "default".to_string();
    let mut aliases = Vec::new();
    let mut i = 2;
    let mut image = None;
    while i < args.len() {
        let arg = args[i].as_str();
        let value = args.get(i + 1).map(String::as_str);
        match arg {
            "--name" => {
                name = Some(value?.to_string());
                i += 2;
            }
            "--hostname" => {
                config["Hostname"] = serde_json::json!(value?);
                i += 2;
            }
            "--workdir" => {
                config["WorkingDir"] = serde_json::json!(value?);
                i += 2;
            }
            "--user" => {
                config["User"] = serde_json::json!(value?);
                i += 2;
            }
            "--entrypoint" => {
                config["Entrypoint"] =
                    serde_json::json!(value?.split_whitespace().collect::<Vec<_>>());
                i += 2;
            }
            "--env" => {
                config["Env"]
                    .as_array_mut()?
                    .push(serde_json::json!(value?));
                i += 2;
            }
            "--env-file" => {
                env_files.push(value?.to_string());
                i += 2;
            }
            "--platform" => {
                platform = Some(value?.to_string());
                i += 2;
            }
            "--label" => {
                let pair = value?;
                let (k, v) = pair.split_once('=')?;
                config["Labels"][k] = serde_json::json!(v);
                i += 2;
            }
            "--publish" => {
                let port = value?;
                let parts = port.split(':').collect::<Vec<_>>();
                let (host_ip, host_port, container_port, protocol) = match parts.as_slice() {
                    [container] => ("", "", *container, "tcp"),
                    [host, container] => ("", *host, *container, "tcp"),
                    [ip, host, container] if ip.parse::<std::net::IpAddr>().is_ok() => {
                        (*ip, *host, *container, "tcp")
                    }
                    _ => ("", "", port, "tcp"),
                };
                let (container_port, protocol) = container_port
                    .split_once('/')
                    .map(|(p, proto)| (p, proto))
                    .unwrap_or((container_port, protocol));
                let key = format!("{container_port}/{protocol}");
                let binding = if host_ip.is_empty() {
                    serde_json::json!({"HostPort": host_port})
                } else {
                    serde_json::json!({"HostIp": host_ip, "HostPort": host_port})
                };
                host["PortBindings"][key.clone()] = serde_json::json!([binding]);
                config["ExposedPorts"][key] = serde_json::json!({});
                i += 2;
            }
            "--volume" | "-v" => {
                host["Binds"]
                    .as_array_mut()?
                    .push(serde_json::json!(value?));
                i += 2;
            }
            "--mount" => {
                host["Mounts"].as_array_mut()?.push(parse_mount(value?)?);
                i += 2;
            }
            "--cap-add" => {
                host["CapAdd"]
                    .as_array_mut()?
                    .push(serde_json::json!(value?));
                i += 2;
            }
            "--cap-drop" => {
                host["CapDrop"]
                    .as_array_mut()?
                    .push(serde_json::json!(value?));
                i += 2;
            }
            "--dns" => {
                host["Dns"].as_array_mut()?.push(serde_json::json!(value?));
                i += 2;
            }
            "--security-opt" => {
                host["SecurityOpt"]
                    .as_array_mut()?
                    .push(serde_json::json!(value?));
                i += 2;
            }
            "--device" => {
                let dev = value?;
                let (src, dst, mode) = parse_device(dev);
                host["Devices"].as_array_mut()?.push(serde_json::json!({"PathOnHost": src, "PathInContainer": dst, "CgroupPermissions": mode}));
                i += 2;
            }
            "--add-host" => {
                let pair = value?;
                let (k, v) = pair.split_once(':')?;
                if !host["ExtraHosts"].is_array() {
                    host["ExtraHosts"] = serde_json::json!([]);
                }
                host["ExtraHosts"]
                    .as_array_mut()?
                    .push(serde_json::json!(format!("{k}:{v}")));
                i += 2;
            }
            "--shm-size" => {
                host["ShmSize"] = serde_json::json!(parse_bytes(value?));
                i += 2;
            }
            "--restart" => {
                host["RestartPolicy"] = restart_policy(value?);
                i += 2;
            }
            "--init" => {
                config["Init"] = serde_json::json!(true);
                i += 1;
            }
            "--rm" => {
                host["AutoRemove"] = serde_json::json!(true);
                i += 1;
            }
            "--network-alias" => {
                aliases.push(value?.to_string());
                i += 2;
            }
            "--network" => {
                network = value?.to_string();
                host["NetworkMode"] = serde_json::json!(network.clone());
                i += 2;
            }
            "--memory" => {
                host["Memory"] = serde_json::json!(parse_bytes(value?));
                i += 2;
            }
            "--cpus" => {
                host["NanoCpus"] = serde_json::json!(
                    (value?.parse::<f64>().unwrap_or(0.0) * 1_000_000_000.0) as i64
                );
                i += 2;
            }
            "--privileged" => {
                host["Privileged"] = serde_json::json!(true);
                i += 1;
            }
            "--tty" | "-t" => {
                config["Tty"] = serde_json::json!(true);
                i += 1;
            }
            "--interactive" | "-i" => {
                config["OpenStdin"] = serde_json::json!(true);
                i += 1;
            }
            value if !value.starts_with('-') && image.is_none() => {
                image = Some(value.to_string());
                i += 1;
            }
            value if !value.starts_with('-') => {
                config["Cmd"].as_array_mut()?.push(serde_json::json!(value));
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    config["Image"] = serde_json::json!(image?);
    config["HostConfig"] = host;
    if !aliases.is_empty() {
        config["NetworkingConfig"]["EndpointsConfig"][&network]["Aliases"] =
            serde_json::json!(aliases);
    }
    if !env_files.is_empty() {
        config["__openoxide_env_files"] = serde_json::json!(env_files);
    }
    let mut path = name
        .map(|n| format!("/containers/create?name={}", encode_query(&n)))
        .unwrap_or_else(|| "/containers/create".into());
    if let Some(platform) = platform {
        path.push_str(if path.contains('?') {
            "&platform="
        } else {
            "?platform="
        });
        path.push_str(&encode_query(&platform));
    }
    Some(ApiRequest {
        method: "POST",
        path,
        body: Some(config),
        response_shape: ResponseShape::CreatedId,
        start_created: false,
    })
}

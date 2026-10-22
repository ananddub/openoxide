pub fn map_cli_args(args: &[String]) -> Option<ApiRequest> {
    let command = args.first()?.as_str();
    let resource = args.get(1).map(String::as_str);
    match (command, resource) {
        ("container", Some("ls" | "ps")) => list_with_flags(args, "/containers/json", true),
        ("image", Some("ls")) | ("images", None) => list_with_flags(args, "/images/json", true),
        ("volume", Some("ls")) => list_with_flags(args, "/volumes", false),
        ("volume", Some("create")) => volume_create(args),
        ("network", Some("ls")) => list_with_flags(args, "/networks", false),
        ("network", Some("create")) => network_create(args),
        ("secret", Some("ls")) => list_with_flags(args, "/secrets", false),
        ("config", Some("ls")) => list_with_flags(args, "/configs", false),
        ("service", Some("ls")) => list_with_flags(args, "/services", false),
        ("node", Some("ls")) => list_with_flags(args, "/nodes", false),
        ("image", Some("pull")) => image_pull(args),
        ("image", Some("push")) => image_push(args),
        ("service", Some("create")) => service_create(args),
        ("service", Some("update")) => service_update(args),
        ("service", Some("scale")) => service_scale(args),
        ("service", Some("rollback")) => resource_action(args, "/services", "rollback"),
        ("service", Some("logs" | "ps")) => service_subcommand(args),
        ("node", Some("promote" | "demote")) => node_role(args),
        ("node", Some("rm" | "remove")) => {
            let id = args.get(2)?;
            Some(ApiRequest {
                method: "DELETE",
                path: format!(
                    "/nodes/{id}?force={}",
                    args.iter().any(|arg| arg == "--force")
                ),
                body: None,
                response_shape: ResponseShape::Raw,
                start_created: false,
            })
        }
        ("node", Some("ps")) => node_tasks(args),
        ("node", Some("update")) => node_update(args),
        ("swarm", Some("init")) => swarm_init(args),
        ("swarm", Some("join")) => swarm_join(args),
        ("swarm", Some("leave")) => Some(ApiRequest {
            method: "POST",
            path: format!("/swarm/leave?force={}", args.iter().any(|v| v == "--force")),
            body: None,
            response_shape: ResponseShape::Raw,
            start_created: false,
        }),
        ("swarm", Some("unlock")) => swarm_unlock(args),
        ("swarm", Some("ca")) => Some(ApiRequest {
            method: "POST",
            path: "/swarm/ca".into(),
            body: Some(serde_json::json!({})),
            response_shape: ResponseShape::Raw,
            start_created: false,
        }),
        ("swarm", Some("update")) => swarm_update(args),
        ("system", Some("info")) | ("info", None) => get("/info"),
        ("version", None) => get("/version"),
        ("swarm", Some("inspect")) => get("/swarm"),
        ("system", Some("df")) => get("/system/df"),
        ("system", Some("events")) => system_events(args),
        ("system", Some("prune")) => prune("/system/prune", args),
        ("login", _) | ("system", Some("login")) => registry_login(args),
        ("logout", None) | ("system", Some("logout")) => None,
        (
            "container",
            Some(action @ ("start" | "stop" | "restart" | "kill" | "pause" | "unpause")),
        ) => id_op(args, "POST", &format!("/{action}"), ResponseShape::Raw),
        ("container", Some("wait")) => id_op(args, "POST", "/wait", ResponseShape::Raw),
        ("container", Some("top")) => id_op(args, "GET", "/top", ResponseShape::Raw),
        ("container", Some("port")) => id_op(args, "GET", "/port", ResponseShape::Raw),
        ("container", Some("logs")) => container_logs(args),
        ("container", Some("stats")) => container_stats(args),
        ("container", Some("rename")) => container_rename(args),
        ("container", Some("update")) => container_update(args),
        ("network", Some("connect" | "disconnect")) => network_membership(args),
        ("container", Some("remove" | "rm")) => container_remove(args),
        ("container", Some("inspect")) => id_op(args, "GET", "/json", ResponseShape::Array),
        ("container", Some("create")) => container_create(args),
        ("container", Some("run")) => container_create(args).map(|mut request| {
            request.start_created = true;
            request
        }),
        ("image", Some("inspect")) => {
            id_op(args, "GET", "/json", ResponseShape::Array).map(|mut r| {
                r.path = r.path.replace("/containers/", "/images/");
                r
            })
        }
        ("image", Some("remove" | "rm")) => {
            let suffix = format!("?force={}", args.iter().any(|v| v == "--force"));
            id_op(args, "DELETE", &suffix, ResponseShape::Raw).map(|mut r| {
                r.path = r.path.replace("/containers/", "/images/");
                r
            })
        }
        ("image", Some("history")) => {
            id_op(args, "GET", "/history", ResponseShape::Raw).map(|mut r| {
                r.path = r.path.replace("/containers/", "/images/");
                r
            })
        }
        ("image", Some("tag")) => image_tag(args),
        ("volume", Some("prune")) => prune("/volumes/prune", args),
        ("network", Some("prune")) => prune("/networks/prune", args),
        ("container", Some("prune")) => prune("/containers/prune", args),
        ("image", Some("prune")) => prune("/images/prune", args),
        ("volume", Some("inspect" | "remove" | "rm")) => id_op(
            args,
            if resource == Some("inspect") {
                "GET"
            } else {
                "DELETE"
            },
            "",
            ResponseShape::Raw,
        )
        .map(|mut r| {
            r.path = r.path.replace("/containers/", "/volumes/");
            r
        }),
        ("network", Some("inspect" | "remove" | "rm")) => id_op(
            args,
            if resource == Some("inspect") {
                "GET"
            } else {
                "DELETE"
            },
            "",
            ResponseShape::Raw,
        )
        .map(|mut r| {
            r.path = r.path.replace("/containers/", "/networks/");
            r
        }),
        ("secret", Some("inspect" | "remove" | "rm")) => id_op(
            args,
            if resource == Some("inspect") {
                "GET"
            } else {
                "DELETE"
            },
            "",
            ResponseShape::Raw,
        )
        .map(|mut r| {
            r.path = r.path.replace("/containers/", "/secrets/");
            r
        }),
        ("config", Some("inspect" | "remove" | "rm")) => id_op(
            args,
            if resource == Some("inspect") {
                "GET"
            } else {
                "DELETE"
            },
            "",
            ResponseShape::Raw,
        )
        .map(|mut r| {
            r.path = r.path.replace("/containers/", "/configs/");
            r
        }),
        ("service", Some("inspect" | "remove" | "rm")) => id_op(
            args,
            if resource == Some("inspect") {
                "GET"
            } else {
                "DELETE"
            },
            "",
            ResponseShape::Array,
        )
        .map(|mut r| {
            r.path = r.path.replace("/containers/", "/services/");
            r
        }),
        ("node", Some("inspect")) => id_op(
            args,
            if resource == Some("inspect") {
                "GET"
            } else {
                "DELETE"
            },
            "",
            ResponseShape::Raw,
        )
        .map(|mut r| {
            r.path = r.path.replace("/containers/", "/nodes/");
            r
        }),
        _ => None,
    }
}


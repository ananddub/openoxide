fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=db/migrations");
    println!("cargo:rerun-if-changed=proto/openoxide/api/v1");

    let protoc_path = protoc_bin_vendored::protoc_bin_path()?;
    unsafe {
        std::env::set_var("PROTOC", protoc_path);
    }

    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    tonic_build::configure().compile_protos_with_config(
        config,
        &["proto/agent.proto"],
        &["proto"],
    )?;

    let mut buffa_config = connectrpc_build::CodeGenConfig::default();
    buffa_config.lazy_views = true;
    buffa_config.preserve_unknown_fields = false;
    buffa_config.generate_json = true;
    buffa_config.emit_register_fn = true;
    buffa_config
        .bytes_fields
        .push((".".into(), buffa_codegen::BytesRepr::Bytes));

    connectrpc_build::Config::new()
        .buffa_config(buffa_config)
        .files(&[
            "proto/openoxide/api/v1/common.proto",
            "proto/openoxide/api/v1/public.proto",
            "proto/openoxide/api/v1/auth.proto",
            "proto/openoxide/api/v1/organization.proto",
            "proto/openoxide/api/v1/project.proto",
            "proto/openoxide/api/v1/environment.proto",
            "proto/openoxide/api/v1/application.proto",
            "proto/openoxide/api/v1/compose.proto",
            "proto/openoxide/api/v1/deployment.proto",
            "proto/openoxide/api/v1/preview.proto",
            "proto/openoxide/api/v1/database.proto",
            "proto/openoxide/api/v1/server.proto",
            "proto/openoxide/api/v1/server_management.proto",
            "proto/openoxide/api/v1/server_controller.proto",
            "proto/openoxide/api/v1/remote_server.proto",
            "proto/openoxide/api/v1/server_private_network.proto",
            "proto/openoxide/api/v1/git_provider.proto",
            "proto/openoxide/api/v1/dns.proto",
            "proto/openoxide/api/v1/vault.proto",
            "proto/openoxide/api/v1/certificate.proto",
            "proto/openoxide/api/v1/domain.proto",
            "proto/openoxide/api/v1/backup.proto",
            "proto/openoxide/api/v1/schedule.proto",
            "proto/openoxide/api/v1/destination.proto",
            "proto/openoxide/api/v1/monitoring.proto",
            "proto/openoxide/api/v1/registry.proto",
            "proto/openoxide/api/v1/notification.proto",
            "proto/openoxide/api/v1/permission.proto",
            "proto/openoxide/api/v1/audit.proto",
            "proto/openoxide/api/v1/ai.proto",
            "proto/openoxide/api/v1/swarm.proto",
            "proto/openoxide/api/v1/traefik.proto",
            "proto/openoxide/api/v1/ssh_key.proto",
            "proto/openoxide/api/v1/terminal.proto",
            "proto/openoxide/api/v1/global.proto",
            "proto/openoxide/api/v1/system.proto",
            "proto/openoxide/api/v1/overview.proto",
            "proto/openoxide/api/v1/alert.proto",
            "proto/openoxide/api/v1/alert_rule.proto",
            "proto/openoxide/api/v1/tag.proto",
            "proto/openoxide/api/v1/webhook.proto",
            "proto/openoxide/api/v1/settings.proto",
            "proto/openoxide/api/v1/networking_operations.proto",
            "proto/openoxide/api/v1/database_network.proto",
            "proto/openoxide/api/v1/incoming_webhook.proto",
            "proto/openoxide/api/v1/realtime.proto",
        ])
        .includes(&["proto"])
        .include_file("_connectrpc.rs")
        .file_per_package(true)
        .emit_descriptor_set("openoxide.fds.bin")
        .compile()?;

    generate_ai_catalog()?;

    Ok(())
}

/// Generate the AI assistant's action catalog from the descriptor set, so adding an RPC adds an assistant action with no manual catalog editing.
fn generate_ai_catalog() -> Result<(), Box<dyn std::error::Error>> {
    use prost::Message;
    use prost_types::FileDescriptorSet;
    use prost_types::field_descriptor_proto::Type as FieldType;

    let out_dir = std::env::var("OUT_DIR")?;
    // connectrpc writes the fds relative to OUT_DIR.
    let fds_path = std::path::Path::new(&out_dir).join("openoxide.fds.bin");
    let bytes = match std::fs::read(&fds_path) {
        Ok(b) => b,
        Err(_) => {
            // Fallback: some versions write to the crate root target.
            std::fs::read("openoxide.fds.bin").unwrap_or_default()
        }
    };
    if bytes.is_empty() {
        // No descriptor available; emit an empty catalog so the module still compiles.
        std::fs::write(
            std::path::Path::new(&out_dir).join("ai_catalog_generated.rs"),
            "pub static GENERATED_ACTIONS: &[GeneratedAction] = &[];\n",
        )?;
        return Ok(());
    }
    let fds = FileDescriptorSet::decode(bytes.as_slice())?;

    // The connectrpc-emitted descriptor strips SourceCodeInfo (proto comments),
    // so run a dedicated protoc pass that keeps it, and build a map of
    // `connect_path -> leading // comment on the rpc`. These comments become
    // the assistant's action descriptions (authored in the .proto, auto-picked
    // up here — no separate manual table to maintain).
    let rpc_comments = extract_rpc_comments().unwrap_or_default();

    // Index all messages by fully-qualified name for field lookups.
    let mut messages: std::collections::HashMap<String, prost_types::DescriptorProto> =
        std::collections::HashMap::new();
    // Index enums by fully-qualified name so param hints list valid values instead of the model guessing.
    let mut enums: std::collections::HashMap<String, prost_types::EnumDescriptorProto> =
        std::collections::HashMap::new();
    fn index_enums(
        pkg_path: &str,
        msg: &prost_types::DescriptorProto,
        enums: &mut std::collections::HashMap<String, prost_types::EnumDescriptorProto>,
    ) {
        let base = format!("{}.{}", pkg_path, msg.name());
        for en in &msg.enum_type {
            enums.insert(format!("{}.{}", base, en.name()), en.clone());
        }
        for nested in &msg.nested_type {
            index_enums(&base, nested, enums);
        }
    }
    for file in &fds.file {
        let pkg = file.package();
        for en in &file.enum_type {
            enums.insert(format!(".{}.{}", pkg, en.name()), en.clone());
        }
        for msg in &file.message_type {
            let fq = format!(".{}.{}", pkg, msg.name());
            messages.insert(fq, msg.clone());
            index_enums(&format!(".{}", pkg), msg, &mut enums);
        }
    }

    // Human-readable param type hint; for enums list the valid non-zero names so the model sends an accepted literal.
    let field_hint = |f: &prost_types::FieldDescriptorProto| -> String {
        let scalar = match f.r#type() {
            FieldType::Int64
            | FieldType::Int32
            | FieldType::Uint64
            | FieldType::Uint32
            | FieldType::Sint32
            | FieldType::Sint64
            | FieldType::Fixed32
            | FieldType::Fixed64
            | FieldType::Sfixed32
            | FieldType::Sfixed64
            | FieldType::Double
            | FieldType::Float => "number",
            FieldType::Bool => "boolean",
            FieldType::String => "string",
            FieldType::Message | FieldType::Group => "object",
            FieldType::Bytes => "string",
            FieldType::Enum => "enum",
        };
        if scalar == "enum" {
            if let Some(en) = enums.get(f.type_name()) {
                let values: Vec<&str> = en
                    .value
                    .iter()
                    .filter(|v| v.number() != 0)
                    .map(|v| v.name())
                    .collect();
                if !values.is_empty() {
                    return format!("enum({})", values.join("|"));
                }
            }
            return "string".to_string();
        }
        scalar.to_string()
    };

    let mut out = String::from(
        "// @generated by build.rs from the proto descriptor set. Do not edit.\n\
         pub struct GeneratedAction {\n    pub name: &'static str,\n    pub connect_path: &'static str,\n    pub description: &'static str,\n    pub params: &'static str,\n}\n\npub static GENERATED_ACTIONS: &[GeneratedAction] = &[\n",
    );

    for file in &fds.file {
        let pkg = file.package();
        for svc in &file.service {
            for method in &svc.method {
                // Skip streaming methods — the invoke tool is unary-only.
                if method.client_streaming() || method.server_streaming() {
                    continue;
                }
                let connect_path = format!("{}.{}/{}", pkg, svc.name(), method.name());
                // snake_case action name: Service + Method, trimmed of "Service".
                let svc_short = svc.name().trim_end_matches("Service");
                let action = format!("{}_{}", to_snake(svc_short), to_snake(method.name()));
                // Build a param hint from the input message's fields, marking
                // required (name!) vs optional (name?) so the model knows what
                // it must supply. proto3 `optional` or a repeated field is not
                // required; everything else is treated as required.
                let params = messages
                    .get(method.input_type())
                    .map(|m| {
                        let parts: Vec<String> = m
                            .field
                            .iter()
                            .map(|f| {
                                let optional = f.proto3_optional()
                                    || f.label()
                                        == prost_types::field_descriptor_proto::Label::Repeated;
                                let marker = if optional { "?" } else { "!" };
                                format!("{}{}:{}", f.name(), marker, field_hint(f))
                            })
                            .collect();
                        format!("{{ {} }}", parts.join(", "))
                    })
                    .unwrap_or_else(|| "{}".to_string());
                // The proto RPC comment is the opt-in signal for AI tooling:
                // only actions a human deliberately documented with a leading
                // `//` are exposed to the assistant. RPCs with no comment
                // (and anything explicitly marked internal) are intentionally
                // NOT registered as tools — e.g. login, logout, refresh-token,
                // 2FA and other credential/session endpoints the AI must never
                // drive on the user's behalf. Mark such an rpc by starting its
                // comment with `@internal` (or `internal:`) to keep the docs
                // while excluding it from the catalog.
                let desc = match rpc_comments
                    .get(&connect_path)
                    .map(|c| c.trim())
                    .filter(|c| !c.is_empty())
                {
                    Some(c) => c.to_string(),
                    None => continue,
                };
                let lower = desc.to_ascii_lowercase();
                if lower.starts_with("@internal") || lower.starts_with("internal:") {
                    continue;
                }
                out.push_str(&format!(
                    "    GeneratedAction {{ name: {:?}, connect_path: {:?}, description: {:?}, params: {:?} }},\n",
                    action, connect_path, desc, params
                ));
            }
        }
    }
    out.push_str("];\n");

    std::fs::write(
        std::path::Path::new(&out_dir).join("ai_catalog_generated.rs"),
        out,
    )?;
    Ok(())
}

/// Run a dedicated protoc pass that retains `SourceCodeInfo`, then map each
/// RPC's leading `//` comment to its connect path (`pkg.Service/Method`).
/// Returns an empty map on any failure (descriptions then fall back to auto).
fn extract_rpc_comments()
-> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
    use prost::Message;
    use prost_types::FileDescriptorSet;

    let out_dir = std::env::var("OUT_DIR")?;
    let out_path = std::path::Path::new(&out_dir).join("openoxide_comments.fds.bin");
    let proto_dir = "proto/openoxide/api/v1";

    // Collect every .proto under the api dir.
    let mut files: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(proto_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("proto") {
            files.push(path.to_string_lossy().into_owned());
        }
    }

    let protoc = std::env::var("PROTOC").map(std::path::PathBuf::from).or_else(
        |_| protoc_bin_vendored::protoc_bin_path(),
    )?;
    let mut cmd = std::process::Command::new(protoc);
    cmd.arg("--include_imports")
        .arg("--include_source_info")
        .arg(format!("--descriptor_set_out={}", out_path.display()))
        .arg("--proto_path=proto");
    for f in &files {
        cmd.arg(f);
    }
    let status = cmd.status()?;
    if !status.success() {
        return Ok(Default::default());
    }

    let bytes = std::fs::read(&out_path)?;
    let fds = FileDescriptorSet::decode(bytes.as_slice())?;

    let mut map = std::collections::HashMap::new();
    // Field numbers in the proto reflection schema:
    //   FileDescriptorProto.service = 6
    //   ServiceDescriptorProto.method = 2
    const FILE_SERVICE: i32 = 6;
    const SERVICE_METHOD: i32 = 2;
    for file in &fds.file {
        let pkg = file.package();
        let Some(sci) = &file.source_code_info else {
            continue;
        };
        for loc in &sci.location {
            // A method location path is [6, svc_idx, 2, method_idx].
            let p = &loc.path;
            if p.len() == 4 && p[0] == FILE_SERVICE && p[2] == SERVICE_METHOD {
                let svc_idx = p[1] as usize;
                let method_idx = p[3] as usize;
                let Some(svc) = file.service.get(svc_idx) else {
                    continue;
                };
                let Some(method) = svc.method.get(method_idx) else {
                    continue;
                };
                let comment = loc
                    .leading_comments
                    .as_deref()
                    .unwrap_or("")
                    .lines()
                    .map(|l| l.trim())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !comment.is_empty() {
                    let path = format!("{}.{}/{}", pkg, svc.name(), method.name());
                    map.insert(path, comment);
                }
            }
        }
    }
    Ok(map)
}

fn to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

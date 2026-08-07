use std::{fs, path::Path};

const RAW_WIREGUARD_CALLS: [&str; 4] = [
    ".run(\"wg\",",
    ".run(\"wg-quick\",",
    ".run_with_stdin(\"wg\",",
    ".run_with_stdin(\"wg-quick\",",
];

#[test]
fn wireguard_commands_cannot_bypass_the_typed_cli() {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let typed_cli_root = source_root.join("utils/os/wireguard/cli");
    let mut violations = Vec::new();

    inspect_rust_sources(&source_root, &typed_cli_root, &mut violations);

    assert!(
        violations.is_empty(),
        "WireGuard commands must use OsCli::wireguard() typed builders; raw calls found in:\n{}",
        violations.join("\n")
    );
}

fn inspect_rust_sources(directory: &Path, typed_cli_root: &Path, violations: &mut Vec<String>) {
    let entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to inspect {}: {error}", directory.display()));

    for entry in entries {
        let path = entry.expect("failed to inspect source entry").path();
        if path.starts_with(typed_cli_root) {
            continue;
        }
        if path.is_dir() {
            inspect_rust_sources(&path, typed_cli_root, violations);
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        if path.file_name().and_then(|value| value.to_str()) == Some("architecture_tests.rs") {
            continue;
        }

        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let compact: String = source
            .chars()
            .filter(|value| !value.is_whitespace())
            .collect();
        if RAW_WIREGUARD_CALLS
            .iter()
            .any(|pattern| compact.contains(pattern))
        {
            violations.push(path.display().to_string());
        }
    }
}

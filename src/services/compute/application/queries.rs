use uuid::Uuid;

pub(super) fn generate_app_name(name: &str) -> String {
    let raw = name
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-");
    let clean = raw.trim_matches('-');
    let suffix = Uuid::new_v4()
        .to_string()
        .split('-')
        .next()
        .unwrap()
        .to_string();
    format!("{}-{}", clean, suffix)
}

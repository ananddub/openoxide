//! Server-side access to the OpenOxide template registry (`GET /meta.json`, `GET /blueprints/{id}/{file}`) so the assistant installs by real id instead of hallucinating compose YAML.

const REGISTRY_BASE: &str = "https://openoxide-templates-worker.duanand6.workers.dev";

/// One catalog row from the registry's `meta.json`.
#[derive(serde::Deserialize)]
pub struct TemplateMeta {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// The resolved files needed to install a template.
pub struct TemplateFiles {
    pub compose: String,
    /// template.toml (env/domains/mounts); empty if the template has none.
    pub toml: String,
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .build()
        .map_err(|e| format!("http client: {e}"))
}

/// Fetch the template catalog (id, name, description, tags).
pub async fn list() -> Result<Vec<TemplateMeta>, String> {
    let resp = client()?
        .get(format!("{REGISTRY_BASE}/meta.json"))
        .send()
        .await
        .map_err(|e| format!("template registry unreachable: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("template registry returned {}", resp.status()));
    }
    resp.json::<Vec<TemplateMeta>>()
        .await
        .map_err(|e| format!("invalid template catalog: {e}"))
}

/// Fetch the compose + template.toml for a template id (toml optional).
pub async fn files(template_id: &str) -> Result<TemplateFiles, String> {
    let base = format!(
        "{REGISTRY_BASE}/blueprints/{}",
        urlencoding::encode(template_id)
    );
    let http = client()?;

    let compose_resp = http
        .get(format!("{base}/docker-compose.yml"))
        .send()
        .await
        .map_err(|e| format!("template fetch failed: {e}"))?;
    if !compose_resp.status().is_success() {
        return Err(format!(
            "no template '{template_id}' (registry returned {})",
            compose_resp.status()
        ));
    }
    let compose = compose_resp
        .text()
        .await
        .map_err(|e| format!("template read failed: {e}"))?;

    // template.toml is optional.
    let toml = match http.get(format!("{base}/template.toml")).send().await {
        Ok(r) if r.status().is_success() => r.text().await.unwrap_or_default(),
        _ => String::new(),
    };

    Ok(TemplateFiles { compose, toml })
}

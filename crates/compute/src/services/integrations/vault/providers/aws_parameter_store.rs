use aws_credential_types::Credentials;
use aws_sdk_ssm::Client;
use serde_json::Value;

use crate::services::integrations::vault::{VaultSecretListDto, VaultTestResultDto};

/// AWS Systems Manager Parameter Store (mirrors the standard
/// `aws-parameter-store` vault type). Config JSON:
/// `{"region": "...", "accessKeyId": "...", "secretAccessKey": "...",
///  "endpoint": "optional", "parameterPath": "/optional/prefix"}`.
pub(crate) fn client(config: Option<&str>) -> Result<Client, String> {
    let value: Value = config
        .and_then(|raw| serde_json::from_str(raw).ok())
        .ok_or_else(|| "AWS Parameter Store config is required".to_string())?;
    let region = value["region"].as_str().unwrap_or("").trim();
    let access_key = value["accessKeyId"].as_str().unwrap_or("").trim();
    let secret_key = value["secretAccessKey"].as_str().unwrap_or("").trim();
    if region.is_empty() || access_key.is_empty() || secret_key.is_empty() {
        return Err("AWS region, accessKeyId and secretAccessKey are required".into());
    }
    let credentials = Credentials::new(access_key, secret_key, None, None, "rustploy-vault");
    let mut builder = aws_sdk_ssm::Config::builder()
        .region(aws_sdk_ssm::config::Region::new(region.to_string()))
        .credentials_provider(credentials)
        .behavior_version_latest();
    if let Some(endpoint) = value["endpoint"].as_str().filter(|v| !v.trim().is_empty()) {
        builder = builder.endpoint_url(endpoint.trim());
    }
    Ok(Client::from_conf(builder.build()))
}

fn parameter_path(config: Option<&str>) -> String {
    config
        .and_then(|raw| serde_json::from_str::<Value>(raw).ok())
        .and_then(|v| v["parameterPath"].as_str().map(|s| s.trim().to_string()))
        .unwrap_or_default()
}

pub(crate) async fn test(config: Option<&str>) -> Result<VaultTestResultDto, String> {
    let path = parameter_path(config);
    let root = if path.is_empty() {
        "/".to_string()
    } else {
        path.clone()
    };
    client(config)?
        .get_parameters_by_path()
        .path(root)
        .recursive(true)
        .max_results(1)
        .send()
        .await
        .map_err(|e| format!("AWS Parameter Store authentication failed: {e}"))?;
    Ok(VaultTestResultDto {
        success: true,
        message: "AWS Parameter Store credentials verified successfully".into(),
    })
}

pub(crate) async fn list(config: Option<&str>) -> Result<VaultSecretListDto, String> {
    let path = parameter_path(config);
    let client = client(config)?;
    let mut names = Vec::new();
    let mut next_token: Option<String> = None;
    loop {
        let mut request = client
            .get_parameters_by_path()
            .path(if path.is_empty() {
                "/".to_string()
            } else {
                path.clone()
            })
            .recursive(true)
            .max_results(10);
        if let Some(token) = next_token.take() {
            request = request.next_token(token);
        }
        let response = request
            .send()
            .await
            .map_err(|e| format!("AWS Parameter Store list failed: {e}"))?;
        for parameter in response.parameters() {
            names.push(parameter.name().unwrap_or_default().to_string());
        }
        next_token = response.next_token().map(str::to_string);
        if next_token.is_none() {
            break;
        }
    }
    Ok(VaultSecretListDto { secrets: names })
}

pub(crate) async fn fetch(config: Option<&str>, reference: &str) -> Result<String, String> {
    let response = client(config)?
        .get_parameter()
        .name(reference)
        .with_decryption(true)
        .send()
        .await
        .map_err(|e| format!("AWS Parameter Store failed to read '{reference}': {e}"))?;
    response
        .parameter()
        .and_then(|p| p.value())
        .map(str::to_string)
        .ok_or_else(|| format!("AWS parameter '{reference}' has no value"))
}

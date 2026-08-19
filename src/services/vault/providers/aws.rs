use aws_credential_types::Credentials;
use aws_sdk_secretsmanager::Client;
use serde_json::Value;

use crate::api::dto::vault::{VaultSecretListDto, VaultTestResultDto};

pub(crate) fn client(config: Option<&str>) -> Result<Client, String> {
    let value: Value = config
        .and_then(|raw| serde_json::from_str(raw).ok())
        .ok_or_else(|| "AWS Secrets Manager config is required".to_string())?;
    let region = value["region"].as_str().unwrap_or("").trim();
    let access_key = value["accessKeyId"].as_str().unwrap_or("").trim();
    let secret_key = value["secretAccessKey"].as_str().unwrap_or("").trim();
    if region.is_empty() || access_key.is_empty() || secret_key.is_empty() {
        return Err("AWS region, accessKeyId and secretAccessKey are required".into());
    }
    let credentials = Credentials::new(access_key, secret_key, None, None, "rustploy-vault");
    let mut builder = aws_sdk_secretsmanager::Config::builder()
        .region(aws_sdk_secretsmanager::config::Region::new(
            region.to_string(),
        ))
        .credentials_provider(credentials)
        .behavior_version_latest();
    if let Some(endpoint) = value["endpoint"].as_str().filter(|v| !v.trim().is_empty()) {
        builder = builder.endpoint_url(endpoint.trim());
    }
    Ok(Client::from_conf(builder.build()))
}

pub(crate) fn parse_reference(reference: &str) -> Result<(&str, Option<&str>), String> {
    if reference.starts_with("arn:") {
        return Err("AWS Secrets Manager references must use the secret name, not an ARN".into());
    }
    Ok(reference
        .rsplit_once(':')
        .map_or((reference, None), |(name, field)| (name, Some(field))))
}

pub(crate) async fn test(config: Option<&str>) -> Result<VaultTestResultDto, String> {
    client(config)?
        .list_secrets()
        .max_results(1)
        .send()
        .await
        .map_err(|e| format!("AWS Secrets Manager authentication failed: {e}"))?;
    Ok(VaultTestResultDto {
        success: true,
        message: "AWS Secrets Manager verified successfully".into(),
    })
}

pub(crate) async fn list(config: Option<&str>) -> Result<VaultSecretListDto, String> {
    let client = client(config)?;
    let mut secrets = Vec::new();
    let mut token: Option<String> = None;
    loop {
        let mut request = client.list_secrets().max_results(100);
        if let Some(value) = token.as_deref() {
            request = request.next_token(value);
        }
        let response = request
            .send()
            .await
            .map_err(|e| format!("AWS Secrets Manager list failed: {e}"))?;
        secrets.extend(
            response
                .secret_list()
                .iter()
                .filter_map(|item| item.name().map(str::to_string)),
        );
        token = response.next_token().map(str::to_string);
        if token.is_none() || secrets.len() >= 500 {
            break;
        }
    }
    secrets.truncate(500);
    Ok(VaultSecretListDto { secrets })
}

pub(crate) async fn fetch(config: Option<&str>, reference: &str) -> Result<String, String> {
    let (secret_id, field) = parse_reference(reference)?;
    let response = client(config)?
        .get_secret_value()
        .secret_id(secret_id)
        .send()
        .await
        .map_err(|e| format!("AWS Secrets Manager failed to read '{secret_id}': {e}"))?;
    let value = response.secret_string().ok_or_else(|| {
        format!("AWS secret '{secret_id}' is binary; binary secrets are not supported")
    })?;
    let Some(field) = field else {
        return Ok(value.to_string());
    };
    let json: Value = serde_json::from_str(value)
        .map_err(|_| format!("AWS secret '{secret_id}' is not JSON, cannot extract '{field}'"))?;
    let selected = &json[field];
    if selected.is_null() {
        return Err(format!(
            "AWS field '{field}' not found in secret '{secret_id}'"
        ));
    }
    Ok(selected
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| selected.to_string()))
}

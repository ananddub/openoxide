use std::{collections::BTreeMap, net::IpAddr, sync::Arc};

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    api::dto::application::middleware::{
        ApplicationMiddlewareResponseDto, MiddlewareHeaderDto, UpsertApplicationMiddlewareDto,
    },
    db::models::application_middlewares::ApplicationMiddleware,
    repository::{ApplicationMiddlewareRepository, ApplicationRepository},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoredMiddlewareConfig {
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    pub average: Option<i64>,
    pub burst: Option<i64>,
    #[serde(default)]
    pub source_ranges: Vec<String>,
}

pub struct ApplicationMiddlewareService {
    applications: Arc<ApplicationRepository>,
    middlewares: Arc<ApplicationMiddlewareRepository>,
}

#[singleton]
impl ApplicationMiddlewareService {
    fn new(
        applications: Arc<ApplicationRepository>,
        middlewares: Arc<ApplicationMiddlewareRepository>,
    ) -> Self {
        Self {
            applications,
            middlewares,
        }
    }

    async fn ensure_application(&self, id: i64) -> sqlx::Result<()> {
        self.applications
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }

    pub async fn list(
        &self,
        application_id: i64,
    ) -> sqlx::Result<Vec<ApplicationMiddlewareResponseDto>> {
        self.ensure_application(application_id).await?;
        let items = self.middlewares.list_by_application(application_id).await?;
        items.into_iter().map(response).collect()
    }

    pub async fn create(
        &self,
        application_id: i64,
        input: UpsertApplicationMiddlewareDto,
    ) -> sqlx::Result<ApplicationMiddlewareResponseDto> {
        self.ensure_application(application_id).await?;
        let (name, middleware_type, enabled, config) = normalize(input)?;
        let item = self
            .middlewares
            .create(application_id, &name, &middleware_type, enabled, &config)
            .await?;
        response(item)
    }

    pub async fn update(
        &self,
        application_id: i64,
        id: i64,
        input: UpsertApplicationMiddlewareDto,
    ) -> sqlx::Result<ApplicationMiddlewareResponseDto> {
        self.ensure_application(application_id).await?;
        let (name, middleware_type, enabled, config) = normalize(input)?;
        let item = self
            .middlewares
            .update(
                id,
                application_id,
                &name,
                &middleware_type,
                enabled,
                &config,
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        response(item)
    }

    pub async fn delete(&self, application_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_application(application_id).await?;
        self.middlewares.delete(id, application_id).await
    }
}

fn normalize(input: UpsertApplicationMiddlewareDto) -> sqlx::Result<(String, String, i64, String)> {
    let name = input.name.trim().to_ascii_lowercase().replace(' ', "-");
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(sqlx::Error::Protocol(
            "middleware name may contain only letters, numbers and hyphens".into(),
        ));
    }
    let middleware_type = input.middleware_type;
    let mut config = StoredMiddlewareConfig::default();
    match middleware_type {
        crate::api::dto::application::middleware::ApplicationMiddlewareType::Compress => {}
        crate::api::dto::application::middleware::ApplicationMiddlewareType::Headers => {
            let headers = input.headers.unwrap_or_default();
            if headers.is_empty() {
                return Err(sqlx::Error::Protocol(
                    "HEADERS middleware requires at least one header".into(),
                ));
            }
            for header in headers {
                config
                    .headers
                    .insert(header.name.trim().to_string(), header.value);
            }
        }
        crate::api::dto::application::middleware::ApplicationMiddlewareType::RateLimit => {
            let average = input.average.filter(|v| *v > 0).ok_or_else(|| {
                sqlx::Error::Protocol("RATE_LIMIT average must be positive".into())
            })?;
            let burst = input
                .burst
                .filter(|v| *v > 0)
                .ok_or_else(|| sqlx::Error::Protocol("RATE_LIMIT burst must be positive".into()))?;
            config.average = Some(average);
            config.burst = Some(burst);
        }
        crate::api::dto::application::middleware::ApplicationMiddlewareType::IpAllowlist => {
            let ranges = input.source_ranges.unwrap_or_default();
            if ranges.is_empty() {
                return Err(sqlx::Error::Protocol(
                    "IP_ALLOWLIST requires source_ranges".into(),
                ));
            }
            for range in &ranges {
                validate_cidr(range)?;
            }
            config.source_ranges = ranges;
        }
    }
    let config =
        serde_json::to_string(&config).map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
    Ok((
        name,
        middleware_type.as_str().to_owned(),
        i64::from(input.enabled),
        config,
    ))
}

fn validate_cidr(value: &str) -> sqlx::Result<()> {
    let (address, prefix) = value
        .split_once('/')
        .ok_or_else(|| sqlx::Error::Protocol(format!("invalid CIDR: {value}")))?;
    let address: IpAddr = address
        .parse()
        .map_err(|_| sqlx::Error::Protocol(format!("invalid CIDR: {value}")))?;
    let prefix: u8 = prefix
        .parse()
        .map_err(|_| sqlx::Error::Protocol(format!("invalid CIDR: {value}")))?;
    let max = if address.is_ipv4() { 32 } else { 128 };
    if prefix > max {
        return Err(sqlx::Error::Protocol(format!("invalid CIDR: {value}")));
    }
    Ok(())
}

fn response(item: ApplicationMiddleware) -> sqlx::Result<ApplicationMiddlewareResponseDto> {
    let config: StoredMiddlewareConfig = serde_json::from_str(&item.config).map_err(|error| {
        sqlx::Error::Protocol(format!("invalid stored middleware config: {error}"))
    })?;
    Ok(ApplicationMiddlewareResponseDto {
        id: item.id,
        application_id: item.application_id,
        name: item.name,
        middleware_type: item.middleware_type,
        enabled: item.enabled != 0,
        headers: config
            .headers
            .into_iter()
            .map(|(name, value)| MiddlewareHeaderDto { name, value })
            .collect(),
        average: config.average,
        burst: config.burst,
        source_ranges: config.source_ranges,
        created_at: item.created_at,
        updated_at: item.updated_at,
    })
}

use super::{
    DnsProviderType, DnsRecordDto, DnsRecordTargetDto, DnsService, DnsServiceError,
    DnsTestResultDto, DnsZoneDto, UpdateDnsRecordDto, UpsertDnsRecordDto,
};

impl DnsService {
    pub async fn test_connection(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.test_cloudflare_token(&provider.credentials_json).await
            }
            DnsProviderType::Route53 => self.test_route53(&provider.credentials_json).await,
            other => {
                self.bridge_test_credentials(other, &provider.credentials_json)
                    .await
            }
        }
    }

    pub async fn test_credentials(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        match provider_type {
            DnsProviderType::Cloudflare => self.test_cloudflare_token(credentials_json).await,
            DnsProviderType::Route53 => self.test_route53(credentials_json).await,
            other => self.bridge_test_credentials(other, credentials_json).await,
        }
    }

    pub async fn list_zones(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.list_cloudflare_zones(&provider.credentials_json).await
            }
            DnsProviderType::Route53 => self.list_route53_zones(&provider.credentials_json).await,
            other => {
                self.bridge_list_zones(other, &provider.credentials_json)
                    .await
            }
        }
    }

    pub async fn list_records(
        &self,
        id: i64,
        organization_id: i64,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.list_cloudflare_records(&provider.credentials_json, zone_id)
                    .await
            }
            DnsProviderType::Route53 => {
                self.list_route53_records(&provider.credentials_json, zone_id)
                    .await
            }
            other => {
                self.bridge_list_records(other, &provider.credentials_json, zone_id)
                    .await
            }
        }
    }

    pub async fn upsert_record(
        &self,
        id: i64,
        organization_id: i64,
        body: UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.upsert_cloudflare_record(&provider.credentials_json, &body)
                    .await
            }
            DnsProviderType::Route53 => {
                self.upsert_route53_record(&provider.credentials_json, &body)
                    .await
            }
            other => {
                self.bridge_upsert_record(other, &provider.credentials_json, &body)
                    .await
            }
        }
    }

    pub async fn update_record(
        &self,
        id: i64,
        organization_id: i64,
        body: UpdateDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                let record_id = if body.target.id.trim().is_empty() {
                    let rows = self
                        .list_cloudflare_records(&provider.credentials_json, &body.target.zone_id)
                        .await?;
                    Self::pick_row(&rows, &body.target)?
                } else {
                    body.target.id.clone()
                };
                self.update_cloudflare_record(&provider.credentials_json, &record_id, &body.next)
                    .await
            }
            DnsProviderType::Route53 => {
                self.update_route53_record(&provider.credentials_json, &body.target, &body.next)
                    .await
            }
            other => {
                self.bridge_update_record(other, &provider.credentials_json, &body)
                    .await
            }
        }
    }

    pub async fn delete_record(
        &self,
        id: i64,
        organization_id: i64,
        target: DnsRecordTargetDto,
    ) -> Result<bool, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                // The delete endpoint takes a record id, never a name.
                let record_id = if target.id.trim().is_empty() {
                    let rows = self
                        .list_cloudflare_records(&provider.credentials_json, &target.zone_id)
                        .await?;
                    Self::pick_row(&rows, &target)?
                } else {
                    target.id.clone()
                };
                self.delete_cloudflare_record(
                    &provider.credentials_json,
                    &target.zone_id,
                    &record_id,
                )
                .await
            }
            DnsProviderType::Route53 => {
                self.delete_route53_record(&provider.credentials_json, &target)
                    .await
            }
            other => {
                self.bridge_delete_record(other, &provider.credentials_json, &target)
                    .await
            }
        }
    }

    /// Pick the listed row a target points at. Rows come back from the provider's own
    /// list call, so a miss means the row is gone rather than a bad request.
    fn pick_row(
        rows: &[DnsRecordDto],
        target: &DnsRecordTargetDto,
    ) -> Result<String, DnsServiceError> {
        let matches: Vec<&DnsRecordDto> =
            rows.iter().filter(|row| row_matches(row, target)).collect();
        match matches.as_slice() {
            [] => Err(DnsServiceError::ProviderError(format!(
                "record '{}' is no longer listed by this provider — refresh the record list",
                target.name
            ))),
            [one] => Ok(one.id.clone()),
            [several @ ..] => Err(DnsServiceError::ProviderError(format!(
                "{} records match '{}' — delete them one at a time",
                several.len(),
                target.name
            ))),
        }
    }
}

/// Compare a UI target against a listed row. Empty target fields widen the match,
/// which is what a row that only knows its name needs.
fn row_matches(row: &DnsRecordDto, target: &DnsRecordTargetDto) -> bool {
    if row.id == target.id && !target.id.is_empty() {
        return true;
    }
    row.name == target.name
        && (target.record_type.is_empty()
            || row.record_type.eq_ignore_ascii_case(&target.record_type))
        && (target.content.is_empty() || row.content == target.content)
}

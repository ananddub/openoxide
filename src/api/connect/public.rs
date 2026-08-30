use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};
use sqlx::SqlitePool;

use super::generated::openoxide::api::v1::{
    GenerateDomainRequest, GenerateDomainResponse, ListGitBranchesRequest, ListGitBranchesResponse,
    PublicService, PublicServiceExt,
};
use crate::services::identity::permission::{PermissionOperation, PermissionResource, Server, CanRead};
use crate::utils::git::GitCli;

pub struct PublicConnectService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl PublicConnectService {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl PublicService for PublicConnectService {
    async fn list_git_branches<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListGitBranchesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListGitBranchesResponse> + Send + use<'a>> {
        let auth = Self::extract_auth(&ctx).await?;
        auth.require_permission(Server::NAME, CanRead::NAME).await.map_err(|e| e.into_connect_error())?;

        let repository_url = request.repository_url.trim();
        if repository_url.is_empty() {
            return Err(ConnectError::invalid_argument(
                "repository_url cannot be empty",
            ));
        }

        let branches = GitCli::new_local()
            .queries()
            .remote_branches(repository_url, None)
            .await
            .map_err(|error| {
                tracing::warn!(repository_url, %error, "could not list git remote branches");
                ConnectError::invalid_argument("could not list git remote branches")
            })?
            .into_iter()
            .map(|branch| branch.name)
            .collect();

        Response::ok(ListGitBranchesResponse {
            branches,
            ..Default::default()
        })
    }

    async fn generate_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GenerateDomainRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GenerateDomainResponse> + Send + use<'a>> {
        let _auth = Self::extract_auth(&ctx).await?;

        let name = slugify(request.name);
        if name.is_empty() {
            return Err(ConnectError::invalid_argument("name cannot be empty"));
        }

        let server_ip =
            sqlx::query_scalar::<_, Option<String>>("SELECT server_ip FROM settings LIMIT 1")
                .fetch_optional(self.db.as_ref())
                .await
                .map_err(|error| {
                    tracing::error!(%error, "could not load server IP for generated domain");
                    ConnectError::new(ErrorCode::Internal, "could not generate domain")
                })?
                .flatten();

        let domain = match server_ip.as_deref() {
            Some(ip) if !ip.is_empty() => format!("{name}.{}.sslip.io", ip.replace('.', "-")),
            _ => format!("{name}.localhost"),
        };

        Response::ok(GenerateDomainResponse {
            domain,
            ..Default::default()
        })
    }
}

fn slugify(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .fold((String::new(), false), |(mut out, separated), ch| {
            if ch.is_ascii_alphanumeric() {
                out.push(ch);
                (out, false)
            } else if !out.is_empty() && !separated {
                out.push('-');
                (out, true)
            } else {
                (out, separated)
            }
        })
        .0
        .trim_end_matches('-')
        .to_owned()
}

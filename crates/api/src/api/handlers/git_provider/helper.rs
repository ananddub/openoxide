use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    GitProvider as GitProviderProto, GitProviderCollaboratorPermission,
    GitProviderConfig as GitProviderConfigProto, GitProviderReference, GitProviderRepository,
    GitProviderType as ProtoGitProviderType, GitProviderWebhookInfo,
};
use crate::services::integrations::git::{GitProviderConfigView, GitProviderView};
use crate::utils::provider::GitProviderType;
use crate::utils::provider::discovery::{
    CollaboratorPermission, GitReferenceInfo, ProviderWebhookInfo, RepositoryInfo,
};

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("git provider not found"),
        sqlx::Error::Protocol(msg) if msg.contains("assigned") => {
            ConnectError::new(ErrorCode::AlreadyExists, msg.clone())
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "git provider operation failed");
            ConnectError::new(ErrorCode::Internal, "git provider operation failed")
        }
    }
}

pub fn provider_error(err: String) -> ConnectError {
    let normalized = err.to_ascii_lowercase();
    if normalized.contains("not found") {
        ConnectError::not_found(err)
    } else if normalized.contains("required")
        || normalized.contains("invalid")
        || normalized.contains("unsupported")
        || normalized.contains("must end with")
        || normalized.contains("does not support")
    {
        ConnectError::invalid_argument(err)
    } else {
        ConnectError::new(ErrorCode::Unavailable, err)
    }
}

pub fn provider_type_to_proto(value: GitProviderType) -> ProtoGitProviderType {
    match value {
        GitProviderType::Github => ProtoGitProviderType::Github,
        GitProviderType::Gitlab => ProtoGitProviderType::Gitlab,
        GitProviderType::Gitea => ProtoGitProviderType::Gitea,
        GitProviderType::Bitbucket => ProtoGitProviderType::Bitbucket,
    }
}

/// Maps the stored provider-type string (`GITHUB`, `GITLAB`, ...) onto the
/// proto enum; unknown or missing values collapse to `UNSPECIFIED`.
pub fn provider_type_str_to_proto(value: &str) -> ProtoGitProviderType {
    value
        .parse::<GitProviderType>()
        .ok()
        .map(provider_type_to_proto)
        .unwrap_or(ProtoGitProviderType::Unspecified)
}

pub fn config_to_proto(view: GitProviderConfigView) -> GitProviderConfigProto {
    GitProviderConfigProto {
        url: view.url,
        internal_url: view.internal_url,
        app_name: view.app_name,
        app_id: view.app_id,
        client_id: view.client_id,
        installation_id: view.installation_id,
        application_id: view.application_id,
        redirect_uri: view.redirect_uri,
        group_name: view.group_name,
        scopes: view.scopes,
        username: view.username,
        email: view.email,
        workspace: view.workspace,
        has_client_secret: view.has_client_secret,
        has_private_key: view.has_private_key,
        has_access_token: view.has_access_token,
        has_app_password: view.has_app_password,
        has_api_token: view.has_api_token,
    }
}

pub fn provider_to_proto(view: GitProviderView) -> GitProviderProto {
    GitProviderProto {
        id: view.id,
        name: view.name,
        provider_type: ::buffa::EnumValue::Known(provider_type_str_to_proto(&view.provider_type)),
        shared: view.shared,
        configured: view.configured,
        webhook_configured: view.webhook_configured,
        created_at: view.created_at,
        updated_at: view.updated_at,
        config: ::buffa::MessageField::some(config_to_proto(view.config)),
    }
}

pub fn repository_to_proto(info: RepositoryInfo) -> GitProviderRepository {
    GitProviderRepository {
        owner: info.owner,
        name: info.name,
        full_name: info.full_name,
        private: info.private,
        default_branch: info.default_branch,
        clone_url: info.clone_url,
    }
}

pub fn reference_to_proto(info: GitReferenceInfo) -> GitProviderReference {
    GitProviderReference {
        name: info.name,
        commit: info.commit,
    }
}

pub fn webhook_info_to_proto(info: ProviderWebhookInfo) -> GitProviderWebhookInfo {
    GitProviderWebhookInfo {
        id: info.id,
        url: info.url,
        active: info.active,
    }
}

pub fn permission_to_proto(
    permission: CollaboratorPermission,
) -> GitProviderCollaboratorPermission {
    GitProviderCollaboratorPermission {
        username: permission.username,
        level: permission.level,
        can_write: permission.can_write,
        can_admin: permission.can_admin,
    }
}

use auto_di::singleton;
use socketioxide::{SocketIo, layer::SocketIoLayer};

pub struct Socket {
    pub io: SocketIo,
    pub layer: SocketIoLayer,
}

#[singleton]
pub async fn socket_init() -> Socket {
    let _ = auto_socket::set_authenticator(|token| async move {
        let auth = auto_di::resolve::<crate::services::auth::AuthService>()
            .await
            .map_err(|error| error.to_string())?;
        let claims = auth
            .validate_access_token(&token)
            .await
            .map_err(|error| error.to_string())?;
        let permissions = auto_di::resolve::<crate::services::permission::PermissionService>()
            .await
            .map_err(|error| error.to_string())?;
        let organization_id = permissions
            .resolve_organization(claims.user.user_id)
            .await
            .map_err(|error| error.to_string())?;
        Ok(auto_socket::LiveIdentity {
            user_id: claims.user.user_id,
            organization_id,
        })
    });
    let _ = auto_socket::set_authorizer(|identity, resource, operation| async move {
        let Some(organization_id) = identity.organization_id else {
            return Ok(false);
        };
        let permissions = auto_di::resolve::<crate::services::permission::PermissionService>()
            .await
            .map_err(|error| error.to_string())?;
        let resource = match resource.rsplit("::").next().unwrap_or(resource) {
            "Project" => "project",
            "Server" => "server",
            "Application" => "app",
            "Database" => "database",
            "Environment" => "env",
            "Organization" => "org",
            "Users" => "users",
            "Groups" => "group",
            "Members" => "member",
            "Invitation" => "invitation",
            "Deployment" => "deployment",
            "Backup" => "backup",
            "VolumeBackup" => "volume_backup",
            "Schedule" => "schedule",
            "Notification" => "notification",
            "Certificate" => "certificate",
            "Registry" => "registry",
            "SshKey" => "ssh_key",
            "Logs" => "logs",
            "Monitoring" => "monitoring",
            "AuditLog" => "audit_log",
            "Workspace" => "workspace",
            "Traefik" => "traefik",
            "Alert" => "alert",
            _ => return Ok(false),
        };
        let operation = match operation.rsplit("::").next().unwrap_or(operation) {
            "CanRead" => "read",
            "CanCreate" => "create",
            "CanUpdate" => "update",
            "CanDelete" => "delete",
            "CanDeploy" => "deploy",
            "CanMonitor" => "monitor",
            "CanWrite" => "write",
            "CanCancel" => "cancel",
            "CanRestore" => "restore",
            _ => return Ok(false),
        };
        permissions
            .check_permission(
                identity.user_id,
                organization_id,
                crate::services::permission::PolicyAction::new(resource, operation),
            )
            .await
            .map_err(|error| error.to_string())
    });
    let (layer, io) = SocketIo::new_layer();
    auto_socket::register_global(&io)
        .await
        .expect("failed to register socket handlers");
    Socket { io, layer }
}

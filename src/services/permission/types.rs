use super::macros::{define_operations, define_resources};
use os::string_enum;

/// Canonical permission resource used by both middleware and the permission service.
pub trait PermissionResource: Send + Sync + 'static {
    const NAME: &'static str;
}

define_resources! {
    Project => "project",
    Server => "server",
    Application => "app",
    Database => "database",
    Environment => "env",
    Organization => "org",
    Users => "users",
    Groups => "group",
    Members => "member",
    Invitation => "invitation",
    Deployment => "deployment",
    Backup => "backup",
    VolumeBackup => "volume_backup",
    Schedule => "schedule",
    Notification => "notification",
    Certificate => "certificate",
    Registry => "registry",
    SshKey => "ssh_key",
    Logs => "logs",
    Monitoring => "monitoring",
    AuditLog => "audit_log",
    Workspace => "workspace",
    Traefik => "traefik",
    Alert => "alert",
}

/// Canonical permission operation used by both middleware and the permission service.
pub trait PermissionOperation: Send + Sync + 'static {
    const NAME: &'static str;
}

define_operations! {
    CanRead => "read",
    CanCreate => "create",
    CanUpdate => "update",
    CanDelete => "delete",
    CanDeploy => "deploy",
    CanMonitor => "monitor",
    CanWrite => "write",
    CanCancel => "cancel",
    CanRestore => "restore",
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum UserRole {
        default = Member;
        Owner => "OWNER",
        Admin => "ADMIN",
        Member => "MEMBER",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PolicyEffect {
        default = Grant;
        Grant => "GRANT",
        Deny => "DENY",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ResourceType {
        default = Project;
        Project => "PROJECT",
        Server => "SERVER",
        Environment => "ENVIRONMENT",
        Service => "SERVICE",
        GitProvider => "GIT_PROVIDER",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyAction {
    resource: &'static str,
    operation: &'static str,
}

impl PolicyAction {
    pub const fn new(resource: &'static str, operation: &'static str) -> Self {
        Self {
            resource,
            operation,
        }
    }

    pub fn as_str(self) -> String {
        format!("{}:{}", self.resource, self.operation)
    }
}

use std::collections::HashMap;

use crate::api::connect::generated::openoxide::api::v1::WatchedTable;

/// Static mapping between SQLite table names and the wire enum. Both sides
/// are compile-time constants, so an unmapped table name means the schema
/// gained a table the proto enum does not know about yet — such changes are
/// downgraded to "all tables" notifications instead of being dropped.
pub struct TableRegistry {
    by_name: HashMap<&'static str, WatchedTable>,
}

impl TableRegistry {
    pub fn new() -> Self {
        let entries = [
            ("ai_generations", WatchedTable::AiGenerations),
            ("ai_settings", WatchedTable::AiSettings),
            ("alert_events", WatchedTable::AlertEvents),
            ("alert_rules", WatchedTable::AlertRules),
            ("application_middlewares", WatchedTable::ApplicationMiddlewares),
            ("application_migrations", WatchedTable::ApplicationMigrations),
            ("audit_logs", WatchedTable::AuditLogs),
            ("background_policies", WatchedTable::BackgroundPolicies),
            ("backup_executions", WatchedTable::BackupExecutions),
            ("backups", WatchedTable::Backups),
            ("certificate_renewals", WatchedTable::CertificateRenewals),
            ("certificates", WatchedTable::Certificates),
            ("database_networks", WatchedTable::DatabaseNetworks),
            ("deployments", WatchedTable::Deployments),
            ("destinations", WatchedTable::Destinations),
            ("dns_providers", WatchedTable::DnsProviders),
            ("domains", WatchedTable::Domains),
            ("environments", WatchedTable::Environments),
            ("git_providers", WatchedTable::GitProviders),
            ("group_policy", WatchedTable::GroupPolicy),
            ("groups", WatchedTable::Groups),
            ("health_reports", WatchedTable::HealthReports),
            ("monitoring_agents", WatchedTable::MonitoringAgents),
            (
                "monitoring_maintenance_windows",
                WatchedTable::MonitoringMaintenanceWindows,
            ),
            ("monitoring_policy", WatchedTable::MonitoringPolicy),
            ("mounts", WatchedTable::Mounts),
            ("migration_objects", WatchedTable::MigrationObjects),
            (
                "notification_delivery_attempts",
                WatchedTable::NotificationDeliveryAttempts,
            ),
            (
                "notification_resource_bindings",
                WatchedTable::NotificationResourceBindings,
            ),
            ("notifications", WatchedTable::Notifications),
            ("organization", WatchedTable::Organization),
            ("organization_invites", WatchedTable::OrganizationInvites),
            ("organization_members", WatchedTable::OrganizationMembers),
            ("patches", WatchedTable::Patches),
            ("policy", WatchedTable::Policy),
            ("ports", WatchedTable::Ports),
            ("preview_deployments", WatchedTable::PreviewDeployments),
            ("projects", WatchedTable::Projects),
            ("project_tags", WatchedTable::ProjectTags),
            ("redirects", WatchedTable::Redirects),
            ("registries", WatchedTable::Registries),
            ("resource_access", WatchedTable::ResourceAccess),
            ("rollbacks", WatchedTable::Rollbacks),
            ("schedule_executions", WatchedTable::ScheduleExecutions),
            (
                "schedule_runtime_policies",
                WatchedTable::ScheduleRuntimePolicies,
            ),
            ("schedules", WatchedTable::Schedules),
            ("security", WatchedTable::Security),
            (
                "server_cleanup_executions",
                WatchedTable::ServerCleanupExecutions,
            ),
            ("server_management", WatchedTable::ServerManagement),
            ("server_migrations", WatchedTable::ServerMigrations),
            (
                "server_private_networks",
                WatchedTable::ServerPrivateNetworks,
            ),
            ("servers", WatchedTable::Servers),
            ("service_application", WatchedTable::ServiceApplication),
            ("service_compose", WatchedTable::ServiceCompose),
            ("service_database", WatchedTable::ServiceDatabase),
            ("service_git", WatchedTable::ServiceGit),
            ("services", WatchedTable::Services),
            ("settings", WatchedTable::Settings),
            ("ssh_keys", WatchedTable::SshKeys),
            ("tags", WatchedTable::Tags),
            ("user_policy", WatchedTable::UserPolicy),
            ("users", WatchedTable::Users),
            ("vault_providers", WatchedTable::VaultProviders),
            ("volume_backups", WatchedTable::VolumeBackups),
        ];
        Self {
            by_name: entries.into_iter().collect(),
        }
    }

    /// Resolves a table name to its wire enum, or `None` when the table is
    /// not watchable (internal churn tables) or unknown to this build.
    pub fn resolve(&self, table: &str) -> Option<WatchedTable> {
        self.by_name.get(table).copied()
    }

    /// Every watchable table — used for resync events.
    pub fn all(&self) -> Vec<WatchedTable> {
        self.by_name.values().copied().collect()
    }
}

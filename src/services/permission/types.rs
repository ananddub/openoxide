use crate::string_enum;

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

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PolicyAction {
        default = ProjectRead;

        ProjectRead => "project:read",
        ProjectCreate => "project:create",
        ProjectUpdate => "project:update",
        ProjectDelete => "project:delete",

        ServerRead => "server:read",
        ServerCreate => "server:create",
        ServerUpdate => "server:update",
        ServerDelete => "server:delete",

        AppRead => "app:read",
        AppCreate => "app:create",
        AppUpdate => "app:update",
        AppDelete => "app:delete",
        AppDeploy => "app:deploy",

        DatabaseRead => "database:read",
        DatabaseCreate => "database:create",
        DatabaseUpdate => "database:update",
        DatabaseDelete => "database:delete",

        EnvRead => "env:read",
        EnvWrite => "env:write",

        OrgRead => "org:read",
        OrgWrite => "org:write",

        UsersRead => "users:read",
        UsersWrite => "users:write",

        ServerMonitor => "server:monitor",
        AppMonitor => "app:monitor",
        TraefikRead => "traefik:read",
        TraefikWrite => "traefik:write",
        AlertWrite => "alert:write",
    }
}

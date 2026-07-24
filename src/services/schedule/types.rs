use crate::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ShellType {
        default = Bash;
        Bash => "BASH",
        Sh => "SH",
    }
}

impl ShellType {
    pub fn executable(self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Sh => "sh",
        }
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ScheduleType {
        default = Application;

        Application => "APPLICATION",
        Compose => "COMPOSE",
        Server => "SERVER",
        DokpanelServer => "DOKPANEL-SERVER",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ScheduleAction {
        default = Exec;
        Exec => "EXEC",
        Deploy => "DEPLOY",
        Redeploy => "REDEPLOY",
        Rebuild => "REBUILD",
        Reload => "RELOAD",
        Start => "START",
        Stop => "STOP",
    }
}

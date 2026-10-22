use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallBackend {
    Ufw,
    Firewalld,
    Iptables,
}

impl FirewallBackend {
    pub(crate) const fn executable(self) -> &'static str {
        match self {
            Self::Ufw => "ufw",
            Self::Firewalld => "firewall-cmd",
            Self::Iptables => "iptables",
        }
    }

    const fn arguments(self) -> &'static [&'static str] {
        match self {
            Self::Ufw => &["status", "verbose"],
            Self::Firewalld => &["--list-all"],
            Self::Iptables => &["-S"],
        }
    }
}

pub struct FirewallInspectBuilder<'a> {
    executor: &'a CommandExecutor,
    backend: FirewallBackend,
}

impl<'a> FirewallInspectBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, backend: FirewallBackend) -> Self {
        Self { executor, backend }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run(self.backend.executable(), self.backend.arguments())
            .await
    }
}

impl IntoCommand for FirewallInspectBuilder<'_> {
    fn build_str(&self) -> String {
        std::iter::once(self.backend.executable())
            .chain(self.backend.arguments().iter().copied())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

use crate::OsCli;
use crate::diagnostics::DiagnosticScope;
use crate::docker::DockerCli;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::firewall::FirewallBackend;

pub struct ServerDiagnosticsOutput {
    pub stdout: String,
    pub stderr: String,
}

pub struct ServerDiagnosticsBuilder<'a> {
    executor: &'a CommandExecutor,
    scopes: Vec<DiagnosticScope>,
}

impl<'a> ServerDiagnosticsBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            scopes: DiagnosticScope::ALL.to_vec(),
        }
    }

    pub fn only(mut self, scopes: impl IntoIterator<Item = DiagnosticScope>) -> Self {
        self.scopes = scopes.into_iter().collect();
        self
    }

    pub fn exclude(mut self, scope: DiagnosticScope) -> Self {
        self.scopes.retain(|candidate| *candidate != scope);
        self
    }

    pub async fn run(self) -> ExecResult<ServerDiagnosticsOutput> {
        let os = OsCli::new(self.executor);
        let mut stdout = String::new();
        let mut stderr = String::new();
        if self.includes(DiagnosticScope::System) {
            collect(os.system().info().run().await, &mut stdout, &mut stderr);
        }
        if self.includes(DiagnosticScope::Disk) {
            collect(
                os.disk().list_mounts().run().await,
                &mut stdout,
                &mut stderr,
            );
        }
        if self.includes(DiagnosticScope::Memory) {
            collect(
                os.resource().memory_usage().run().await,
                &mut stdout,
                &mut stderr,
            );
        }
        if self.includes(DiagnosticScope::Docker) {
            let docker = DockerCli::from_executor(self.executor.clone());
            collect_debug(docker.system().version().await, &mut stdout, &mut stderr);
            collect_debug(docker.system().info().run().await, &mut stdout, &mut stderr);
        }
        if self.includes(DiagnosticScope::Firewall) {
            for backend in [
                FirewallBackend::Ufw,
                FirewallBackend::Firewalld,
                FirewallBackend::Iptables,
            ] {
                if os.has_command(backend.executable()).run().await.is_ok() {
                    collect(
                        os.firewall().inspect(backend).run().await,
                        &mut stdout,
                        &mut stderr,
                    );
                }
            }
        }
        Ok(ServerDiagnosticsOutput { stdout, stderr })
    }

    fn includes(&self, scope: DiagnosticScope) -> bool {
        self.scopes.contains(&scope)
    }
}

fn collect(result: ExecResult<ExecOutput>, stdout: &mut String, stderr: &mut String) {
    match result {
        Ok(output) => {
            stdout.push_str(&output.stdout);
            stdout.push('\n');
            stderr.push_str(&output.stderr);
        }
        Err(error) => {
            stderr.push_str(&error.to_string());
            stderr.push('\n');
        }
    }
}

fn collect_debug<T: std::fmt::Debug, E: std::fmt::Display>(
    result: Result<T, E>,
    stdout: &mut String,
    stderr: &mut String,
) {
    match result {
        Ok(output) => {
            stdout.push_str(&format!("{output:#?}\n"));
        }
        Err(error) => {
            stderr.push_str(&error.to_string());
            stderr.push('\n');
        }
    }
}

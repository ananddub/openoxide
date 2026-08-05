use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;
use crate::utils::os::firewall::NetworkProtocol;

pub struct FirewallAllowBuilder<'a> {
    executor: &'a CommandExecutor,
    port: String,
    proto: Option<NetworkProtocol>,
    from_ip: Option<String>,
}

impl<'a> FirewallAllowBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, port: impl IntoCommand) -> Self {
        Self {
            executor,
            port: port.build_str(),
            proto: None,
            from_ip: None,
        }
    }
    pub fn protocol(mut self, protocol: NetworkProtocol) -> Self {
        self.proto = Some(protocol);
        self
    }
    pub fn from_ip(mut self, ip: impl Into<String>) -> Self {
        self.from_ip = Some(ip.into());
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = vec!["allow".to_string()];
        if let Some(ref ip) = self.from_ip {
            args.push("from".to_string());
            args.push(ip.clone());
            args.push("to".to_string());
            args.push("any".to_string());
            args.push("port".to_string());
            let port_str = if let Some(ref p) = self.proto {
                format!("{}/{}", self.port, p.as_str())
            } else {
                self.port.clone()
            };
            args.push(port_str);
        } else {
            let port_str = if let Some(ref p) = self.proto {
                format!("{}/{}", self.port, p.as_str())
            } else {
                self.port.clone()
            };
            args.push(port_str);
        }
        self.executor.run("ufw", &args).await
    }
}

impl<'a> IntoCommand for FirewallAllowBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["ufw".to_string(), "allow".to_string()];
        if let Some(ref ip) = self.from_ip {
            parts.push("from".to_string());
            parts.push(escape_arg(ip));
            parts.push("to".to_string());
            parts.push("any".to_string());
            parts.push("port".to_string());
            let port_str = if let Some(ref p) = self.proto {
                format!("{}/{}", self.port, p.as_str())
            } else {
                self.port.clone()
            };
            parts.push(escape_arg(port_str));
        } else {
            let port_str = if let Some(ref p) = self.proto {
                format!("{}/{}", self.port, p.as_str())
            } else {
                self.port.clone()
            };
            parts.push(escape_arg(port_str));
        }
        parts.join(" ")
    }
}

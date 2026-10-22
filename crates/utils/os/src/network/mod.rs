use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod command;

pub use command::NetworkCommandBuilder;

pub struct NetworkCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> NetworkCli<'a> {
    pub fn interfaces(&self) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "ip",
            vec!["-o".to_string(), "link".to_string(), "show".to_string()],
        )
    }
    pub fn ipv4_addresses(&self) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "ip",
            vec!["-4".into(), "-o".into(), "addr".into(), "show".into()],
        )
    }
    pub fn ip(&self, interface: impl IntoCommand) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "ip",
            vec![
                "-o".to_string(),
                "-4".to_string(),
                "addr".to_string(),
                "show".to_string(),
                "dev".to_string(),
                interface.build_str(),
            ],
        )
    }
    pub fn mac(&self, interface: impl IntoCommand) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "cat",
            vec![format!("/sys/class/net/{}/address", interface.build_str())],
        )
    }
    pub fn ping(&self, host: impl IntoCommand) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "ping",
            vec!["-c".to_string(), "4".to_string(), host.build_str()],
        )
    }
    pub fn download(
        &self,
        url: impl IntoCommand,
        dest: impl IntoCommand,
    ) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "curl",
            vec![
                "-sL".to_string(),
                url.build_str(),
                "-o".to_string(),
                dest.build_str(),
            ],
        )
    }
    pub fn upload(
        &self,
        file: impl IntoCommand,
        url: impl IntoCommand,
    ) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "curl",
            vec![
                "-F".to_string(),
                format!("file=@{}", file.build_str()),
                url.build_str(),
            ],
        )
    }
    pub fn port_open(
        &self,
        host: impl IntoCommand,
        port: impl IntoCommand,
    ) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "nc",
            vec![
                "-z".to_string(),
                "-w".to_string(),
                "3".to_string(),
                host.build_str(),
                port.build_str(),
            ],
        )
    }
    pub fn listen_ports(&self) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(self.executor, "ss", vec!["-tuln".to_string()])
    }
    pub fn listen_ports_detailed(&self) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(self.executor, "ss", vec!["-H".into(), "-ltnu".into()])
    }
    pub fn route_table(&self) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "ip",
            vec!["route".to_string(), "show".to_string()],
        )
    }
    pub fn dns_lookup(&self, host: impl IntoCommand) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "dig",
            vec!["+short".to_string(), host.build_str()],
        )
    }
    pub fn public_ip(&self) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "curl",
            vec!["-s".to_string(), "https://ifconfig.me".to_string()],
        )
    }
    pub fn interface_up(&self, name: impl IntoCommand) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "ip",
            vec![
                "link".to_string(),
                "set".to_string(),
                "dev".to_string(),
                name.build_str(),
                "up".to_string(),
            ],
        )
    }
    pub fn interface_down(&self, name: impl IntoCommand) -> NetworkCommandBuilder<'a> {
        NetworkCommandBuilder::new(
            self.executor,
            "ip",
            vec![
                "link".to_string(),
                "set".to_string(),
                "dev".to_string(),
                name.build_str(),
                "down".to_string(),
            ],
        )
    }
}

use std::{collections::BTreeMap, net::IpAddr};

use crate::exec::{CommandExecutor, ExecResult};

use super::DigResponse;
use crate::dns::types::{
    DigAddressFamily, DigEdnsBufferSize, DigEdnsVersion, DigPort, DigQueryClass, DigRecordType,
    DigSection, DigTimeout, DigTransport, DigTries, DnsName, command_error,
};

pub struct DigBuilder<'a> {
    executor: &'a CommandExecutor,
    name: Option<DnsName>,
    reverse: Option<IpAddr>,
    server: Option<IpAddr>,
    port: DigPort,
    record_type: DigRecordType,
    query_class: DigQueryClass,
    transport: DigTransport,
    family: DigAddressFamily,
    dnssec: bool,
    recurse: bool,
    trace: bool,
    short: bool,
    search: bool,
    show_search: bool,
    fail_on_servfail: bool,
    ignore_truncation: bool,
    multiline: bool,
    identify: bool,
    show_command: bool,
    ttl: bool,
    ttl_units: bool,
    cookie: Option<bool>,
    nsid: bool,
    edns_version: Option<DigEdnsVersion>,
    edns_buffer_size: Option<DigEdnsBufferSize>,
    timeout: DigTimeout,
    tries: DigTries,
    sections: BTreeMap<DigSection, bool>,
}

impl<'a> DigBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, name: DnsName) -> Self {
        Self::base(executor, Some(name), None)
    }

    pub(crate) fn reverse(executor: &'a CommandExecutor, address: IpAddr) -> Self {
        Self::base(executor, None, Some(address))
    }

    fn base(executor: &'a CommandExecutor, name: Option<DnsName>, reverse: Option<IpAddr>) -> Self {
        Self {
            executor,
            name,
            reverse,
            server: None,
            port: DigPort::default(),
            record_type: DigRecordType::default(),
            query_class: DigQueryClass::default(),
            transport: DigTransport::default(),
            family: DigAddressFamily::default(),
            dnssec: false,
            recurse: true,
            trace: false,
            short: false,
            search: false,
            show_search: false,
            fail_on_servfail: false,
            ignore_truncation: false,
            multiline: false,
            identify: false,
            show_command: true,
            ttl: true,
            ttl_units: false,
            cookie: None,
            nsid: false,
            edns_version: None,
            edns_buffer_size: None,
            timeout: DigTimeout::default(),
            tries: DigTries::default(),
            sections: BTreeMap::new(),
        }
    }

    pub fn server(mut self, server: IpAddr) -> Self {
        self.server = Some(server);
        self
    }
    pub fn port(mut self, port: DigPort) -> Self {
        self.port = port;
        self
    }
    pub fn record_type(mut self, value: DigRecordType) -> Self {
        self.record_type = value;
        self
    }
    pub fn query_class(mut self, value: DigQueryClass) -> Self {
        self.query_class = value;
        self
    }
    pub fn transport(mut self, value: DigTransport) -> Self {
        self.transport = value;
        self
    }
    pub fn address_family(mut self, value: DigAddressFamily) -> Self {
        self.family = value;
        self
    }
    pub fn dnssec(mut self, enabled: bool) -> Self {
        self.dnssec = enabled;
        self
    }
    pub fn recurse(mut self, enabled: bool) -> Self {
        self.recurse = enabled;
        self
    }
    pub fn trace(mut self, enabled: bool) -> Self {
        self.trace = enabled;
        self
    }
    pub fn short(mut self, enabled: bool) -> Self {
        self.short = enabled;
        self
    }
    pub fn search(mut self, enabled: bool) -> Self {
        self.search = enabled;
        self
    }
    pub fn show_search(mut self, enabled: bool) -> Self {
        self.show_search = enabled;
        self
    }
    pub fn fail_on_servfail(mut self, enabled: bool) -> Self {
        self.fail_on_servfail = enabled;
        self
    }
    pub fn ignore_truncation(mut self, enabled: bool) -> Self {
        self.ignore_truncation = enabled;
        self
    }
    pub fn multiline(mut self, enabled: bool) -> Self {
        self.multiline = enabled;
        self
    }
    pub fn identify(mut self, enabled: bool) -> Self {
        self.identify = enabled;
        self
    }
    pub fn show_command(mut self, enabled: bool) -> Self {
        self.show_command = enabled;
        self
    }
    pub fn ttl(mut self, enabled: bool) -> Self {
        self.ttl = enabled;
        self
    }
    pub fn ttl_units(mut self, enabled: bool) -> Self {
        self.ttl_units = enabled;
        self
    }
    pub fn cookie(mut self, enabled: bool) -> Self {
        self.cookie = Some(enabled);
        self
    }
    pub fn nsid(mut self, enabled: bool) -> Self {
        self.nsid = enabled;
        self
    }
    pub fn edns_version(mut self, value: DigEdnsVersion) -> Self {
        self.edns_version = Some(value);
        self
    }
    pub fn edns_buffer_size(mut self, value: DigEdnsBufferSize) -> Self {
        self.edns_buffer_size = Some(value);
        self
    }
    pub fn timeout(mut self, value: DigTimeout) -> Self {
        self.timeout = value;
        self
    }
    pub fn tries(mut self, value: DigTries) -> Self {
        self.tries = value;
        self
    }
    pub fn section(mut self, section: DigSection, enabled: bool) -> Self {
        self.sections.insert(section, enabled);
        self
    }
    pub fn include(self, section: DigSection) -> Self {
        self.section(section, true)
    }
    pub fn exclude(self, section: DigSection) -> Self {
        self.section(section, false)
    }

    pub async fn execute(self) -> ExecResult<DigResponse> {
        let output = self.executor.run("dig", self.arguments()?).await?;
        Ok(DigResponse::parse(output.stdout))
    }

    fn arguments(&self) -> ExecResult<Vec<String>> {
        let mut args = Vec::new();
        if let Some(server) = self.server {
            args.push(format!("@{server}"));
        }
        args.push(format!("-p{}", self.port.get()));
        args.push(format!("+time={}", self.timeout.get()));
        args.push(format!("+tries={}", self.tries.get()));
        toggle(&mut args, "recurse", self.recurse);
        if self.transport == DigTransport::Tcp {
            args.push("+tcp".into());
        }
        match self.family {
            DigAddressFamily::Any => {}
            DigAddressFamily::Ipv4 => args.push("-4".into()),
            DigAddressFamily::Ipv6 => args.push("-6".into()),
        }
        enabled(&mut args, "dnssec", self.dnssec);
        enabled(&mut args, "trace", self.trace);
        enabled(&mut args, "short", self.short);
        enabled(&mut args, "search", self.search);
        enabled(&mut args, "showsearch", self.show_search);
        enabled(&mut args, "fail", self.fail_on_servfail);
        enabled(&mut args, "ignore", self.ignore_truncation);
        enabled(&mut args, "multiline", self.multiline);
        enabled(&mut args, "identify", self.identify);
        toggle(&mut args, "cmd", self.show_command);
        toggle(&mut args, "ttlid", self.ttl);
        enabled(&mut args, "ttlunits", self.ttl_units);
        if let Some(cookie) = self.cookie {
            toggle(&mut args, "cookie", cookie);
        }
        enabled(&mut args, "nsid", self.nsid);
        if let Some(version) = self.edns_version {
            args.push(format!("+edns={}", version.get()));
        }
        if let Some(size) = self.edns_buffer_size {
            args.push(format!("+bufsize={}", size.get()));
        }
        for (section, value) in &self.sections {
            toggle(&mut args, section.as_str(), *value);
        }
        if let Some(address) = self.reverse {
            args.extend(["-x".into(), address.to_string()]);
        } else if let Some(name) = &self.name {
            args.extend([
                name.as_str().into(),
                self.record_type.as_str().into(),
                self.query_class.as_str().into(),
            ]);
        } else {
            return Err(command_error(
                "dig query requires a name or reverse address",
            ));
        }
        Ok(args)
    }
}

fn enabled(args: &mut Vec<String>, option: &str, value: bool) {
    if value {
        args.push(format!("+{option}"));
    }
}
fn toggle(args: &mut Vec<String>, option: &str, value: bool) {
    args.push(format!("+{}{option}", if value { "" } else { "no" }));
}

#[cfg(test)]
mod tests {
    use super::DigBuilder;
    use crate::dns::{DigPort, DigRecordType, DigTimeout, DigTransport, DigTries, DnsName};
    use crate::exec::{CommandExecutor, LocalExecutor};
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn builds_typed_query_arguments() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let args = DigBuilder::new(&executor, DnsName::new("pi.rustploy.internal").unwrap())
            .server(IpAddr::V4(Ipv4Addr::new(10, 91, 0, 1)))
            .port(DigPort::new(5353).unwrap())
            .record_type(DigRecordType::Aaaa)
            .transport(DigTransport::Tcp)
            .short(true)
            .show_command(false)
            .timeout(DigTimeout::new(2).unwrap())
            .tries(DigTries::new(1).unwrap())
            .arguments()
            .unwrap();
        assert_eq!(
            args,
            vec![
                "@10.91.0.1",
                "-p5353",
                "+time=2",
                "+tries=1",
                "+recurse",
                "+tcp",
                "+short",
                "+nocmd",
                "+ttlid",
                "pi.rustploy.internal",
                "AAAA",
                "IN"
            ]
        );
    }
}

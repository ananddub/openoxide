use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

#[derive(Debug, Clone)]
pub enum SocatAddress {
    UnixConnect(String),
    UnixListen(String),
    TcpConnect(String, u16),
    TcpListen(u16),
    Exec(String),
    Stdio,
    Pipe,
    Pty,
    Custom(String),
}

impl SocatAddress {
    pub fn to_address_str(&self) -> String {
        match self {
            SocatAddress::UnixConnect(path) => format!("UNIX-CONNECT:{}", path),
            SocatAddress::UnixListen(path) => format!("UNIX-LISTEN:{}", path),
            SocatAddress::TcpConnect(host, port) => format!("TCP-CONNECT:{}:{}", host, port),
            SocatAddress::TcpListen(port) => format!("TCP-LISTEN:{}", port),
            SocatAddress::Exec(cmd) => format!("EXEC:\"{}\"", cmd),
            SocatAddress::Stdio => "STDIO".to_string(),
            SocatAddress::Pipe => "PIPE".to_string(),
            SocatAddress::Pty => "PTY".to_string(),
            SocatAddress::Custom(s) => s.clone(),
        }
    }
}

pub struct SocatBuilder<'a> {
    executor: &'a CommandExecutor,
    src: Option<SocatAddress>,
    dst: Option<SocatAddress>,
    options: Vec<String>,
}

impl<'a> SocatBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            src: None,
            dst: None,
            options: Vec::new(),
        }
    }

    pub fn src(mut self, addr: SocatAddress) -> Self {
        self.src = Some(addr);
        self
    }

    pub fn dst(mut self, addr: SocatAddress) -> Self {
        self.dst = Some(addr);
        self
    }

    pub fn connect_unix(self, path: impl Into<String>) -> Self {
        self.src(SocatAddress::UnixConnect(path.into()))
    }

    pub fn listen_unix(self, path: impl Into<String>) -> Self {
        self.src(SocatAddress::UnixListen(path.into()))
    }

    pub fn connect_tcp(self, host: impl Into<String>, port: u16) -> Self {
        self.src(SocatAddress::TcpConnect(host.into(), port))
    }

    pub fn listen_tcp(self, port: u16) -> Self {
        self.src(SocatAddress::TcpListen(port))
    }

    pub fn exec(self, cmd: impl Into<String>) -> Self {
        self.dst(SocatAddress::Exec(cmd.into()))
    }

    pub fn to_unix(self, path: impl Into<String>) -> Self {
        self.dst(SocatAddress::UnixConnect(path.into()))
    }

    pub fn to_tcp(self, host: impl Into<String>, port: u16) -> Self {
        self.dst(SocatAddress::TcpConnect(host.into(), port))
    }

    pub fn stdio(self) -> Self {
        self.src(SocatAddress::Stdio)
    }

    pub fn pty(self) -> Self {
        self.dst(SocatAddress::Pty)
    }

    pub fn option(mut self, opt: impl Into<String>) -> Self {
        self.options.push(opt.into());
        self
    }

    pub fn raw(self) -> Self {
        self.option("raw").option("echo=0")
    }

    pub fn build_args(&self) -> Vec<String> {
        let mut args = self.options.clone();
        if let Some(src) = &self.src {
            args.push(src.to_address_str());
        }
        if let Some(dst) = &self.dst {
            args.push(dst.to_address_str());
        }
        args
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let args = self.build_args();
        self.executor.run("socat", &args).await
    }
}

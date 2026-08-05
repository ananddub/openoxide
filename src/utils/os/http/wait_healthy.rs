use crate::utils::exec::script::dsl::{ArgToken, Command, ShellIR};
use crate::utils::exec::script::{IntoCommand, sh};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::http::{HealthyStatus, HttpMethod};

pub struct HttpWaitHealthyBuilder<'a> {
    executor: &'a CommandExecutor,
    url: String,
    timeout: String,
    healthy_status: HealthyStatus,
    insecure: bool,
    method: HttpMethod,
    headers: Vec<(String, String)>,
}

impl<'a> HttpWaitHealthyBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        url: impl IntoCommand,
        timeout: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            url: url.build_str(),
            timeout: timeout.build_str(),
            healthy_status: HealthyStatus::SuccessOrRedirect,
            insecure: false,
            method: HttpMethod::Get,
            headers: Vec::new(),
        }
    }
    pub fn healthy_status(mut self, status: HealthyStatus) -> Self {
        self.healthy_status = status;
        self
    }
    pub fn allow_insecure_tls(mut self) -> Self {
        self.insecure = true;
        self
    }
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }
    pub fn header(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.headers.push((k.into(), v.into()));
        self
    }

    fn curl_command(&self) -> ShellIR {
        let mut args = vec![
            ArgToken::Literal("-s".into()),
            ArgToken::Literal("-o".into()),
            ArgToken::NullDevice,
            ArgToken::Literal("-w".into()),
            ArgToken::Literal("%{http_code}".into()),
        ];
        if self.insecure {
            args.push(ArgToken::Literal("-k".into()));
        }
        if self.method != HttpMethod::Get {
            args.push(ArgToken::Literal("-X".into()));
            args.push(ArgToken::Literal(self.method.as_str().into()));
        }
        for (k, v) in &self.headers {
            args.push(ArgToken::Literal("-H".into()));
            args.push(ArgToken::Literal(format!("{k}: {v}")));
        }
        args.push(ArgToken::dynamic(self.url.clone()));
        ShellIR::Command(Command {
            name: "curl".into(),
            args,
        })
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.script().execute(self.executor).await
    }

    fn script(&self) -> Vec<ShellIR> {
        let curl = self.curl_command();
        let timeout = self.timeout.as_str();
        let pattern = self.healthy_status.pattern();
        sh!(
            let start = capture_stdout! { cmd("date", "+%s"); };
            while cmd("true") {
                if pipe![ir!(curl.clone()), cmd("grep", "-qE", dynamic!(pattern))] {
                    exit(0);
                }
                let current = capture_stdout! { cmd("date", "+%s"); };
                let elapsed = capture_stdout! { cmd("expr", current, "-", start); };
                if cmd("test", elapsed, "-ge", dynamic!(timeout)) {
                    echo("Timeout waiting for healthy response").stderr(crate::utils::exec::script::dsl::OutputTarget::StandardError);
                    exit(1);
                }
                sleep(1);
            }
        )
    }
}

impl<'a> IntoCommand for HttpWaitHealthyBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}

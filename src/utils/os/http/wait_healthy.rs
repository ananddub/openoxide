use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct HttpWaitHealthyBuilder<'a> {
    executor: &'a CommandExecutor,
    url: String,
    timeout: String,
    status_pattern: String,
    insecure: bool,
    method: String,
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
            status_pattern: "^(2|3)".to_string(),
            insecure: false,
            method: "GET".to_string(),
            headers: Vec::new(),
        }
    }
    pub fn status_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.status_pattern = pattern.into();
        self
    }
    pub fn insecure(mut self, val: bool) -> Self {
        self.insecure = val;
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }
    pub fn header(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.headers.push((k.into(), v.into()));
        self
    }

    fn curl_args(&self) -> String {
        let mut parts = vec![
            "-s".to_string(),
            "-o".to_string(),
            "/dev/null".to_string(),
            "-w".to_string(),
            "%{http_code}".to_string(),
        ];
        if self.insecure {
            parts.push("-k".to_string());
        }
        if self.method != "GET" {
            parts.push("-X".to_string());
            parts.push(self.method.clone());
        }
        for (k, v) in &self.headers {
            parts.push("-H".to_string());
            parts.push(format!("{}: {}", k, v));
        }
        parts.push(self.url.clone());

        // Escape all parts for safe execution/stringifying
        parts
            .iter()
            .map(|p| escape_arg(p))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let curl_cmd = self.curl_args();
        let cmd = format!(
            "timeout=$1; start_time=$(date +%s); while true; do if curl {} | grep -qE \"$2\"; then exit 0; fi; current_time=$(date +%s); elapsed=$((current_time - start_time)); if [ $elapsed -ge $timeout ]; then echo \"Timeout waiting for healthy response\" >&2; exit 1; fi; sleep 1; done",
            curl_cmd
        );
        self.executor
            .run(
                "sh",
                &["-c", &cmd, "dummy", &self.timeout, &self.status_pattern],
            )
            .await
    }
}

impl<'a> IntoCommand for HttpWaitHealthyBuilder<'a> {
    fn build_str(&self) -> String {
        let curl_cmd = self.curl_args();
        let cmd = format!(
            "timeout=$1; start_time=$(date +%s); while true; do if curl {} | grep -qE \"$2\"; then exit 0; fi; current_time=$(date +%s); elapsed=$((current_time - start_time)); if [ $elapsed -ge $timeout ]; then echo \"Timeout waiting for healthy response\" >&2; exit 1; fi; sleep 1; done",
            curl_cmd
        );
        format!(
            "sh -c '{}' dummy {} {}",
            cmd,
            escape_arg(&self.timeout),
            escape_arg(&self.status_pattern)
        )
    }
}

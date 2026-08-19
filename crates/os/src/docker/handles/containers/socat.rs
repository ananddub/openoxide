use crate::docker::DockerCli;
use crate::socat::SocatRequestBuilder;

pub struct ContainerSocketStreamBuilder<'a> {
    pub(crate) _cli: &'a DockerCli,
    pub(crate) container_id: String,
    pub(crate) cmd: Vec<String>,
    pub(crate) envs: Vec<(String, String)>,
    pub(crate) user: Option<String>,
    pub(crate) workdir: Option<String>,
    pub(crate) privileged: bool,
    pub(crate) tty: bool,
    pub(crate) attach_stdin: bool,
    pub(crate) attach_stdout: bool,
    pub(crate) attach_stderr: bool,
    pub(crate) rows: u16,
    pub(crate) cols: u16,
    pub(crate) socket_path: String,
}

impl<'a> ContainerSocketStreamBuilder<'a> {
    pub fn new(cli: &'a DockerCli, container_id: impl Into<String>) -> Self {
        Self {
            _cli: cli,
            container_id: container_id.into(),
            cmd: vec!["/bin/sh".to_string()],
            envs: Vec::new(),
            user: None,
            workdir: None,
            privileged: false,
            tty: true,
            attach_stdin: true,
            attach_stdout: true,
            attach_stderr: true,
            rows: 24,
            cols: 80,
            socket_path: "/var/run/docker.sock".to_string(),
        }
    }

    pub fn cmd(mut self, cmd: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.cmd = cmd.into_iter().map(Into::into).collect();
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn workdir(mut self, workdir: impl Into<String>) -> Self {
        self.workdir = Some(workdir.into());
        self
    }

    pub fn privileged(mut self, enabled: bool) -> Self {
        self.privileged = enabled;
        self
    }

    pub fn interactive(mut self, enabled: bool) -> Self {
        self.attach_stdin = enabled;
        self
    }

    pub fn env(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.envs.push((k.into(), v.into()));
        self
    }

    pub fn envs(mut self, envs: impl IntoIterator<Item = (String, String)>) -> Self {
        self.envs.extend(envs);
        self
    }

    pub fn tty(mut self, enabled: bool) -> Self {
        self.tty = enabled;
        self
    }

    pub fn size(mut self, cols: u16, rows: u16) -> Self {
        self.cols = cols;
        self.rows = rows;
        self
    }

    pub fn socket_path(mut self, path: impl Into<String>) -> Self {
        self.socket_path = path.into();
        self
    }

    pub async fn connect(self) -> std::io::Result<ContainerSocketExecStream> {
        let env_strings: Vec<String> = self
            .envs
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        let mut payload = serde_json::json!({
            "AttachStdin": self.attach_stdin,
            "AttachStdout": self.attach_stdout,
            "AttachStderr": self.attach_stderr,
            "Tty": self.tty,
            "ConsoleSize": [self.rows, self.cols],
            "Cmd": self.cmd,
            "Env": env_strings,
            "Privileged": self.privileged,
        });

        if let Some(user) = &self.user {
            payload["User"] = serde_json::json!(user);
        }
        if let Some(workdir) = &self.workdir {
            payload["WorkingDir"] = serde_json::json!(workdir);
        }

        // Step 1: Create Exec Instance via SocatRequestBuilder
        let create_path = format!("/containers/{}/exec", self.container_id);
        let res1 = SocatRequestBuilder::post(create_path)
            .socket_path(&self.socket_path)
            .json(&payload)
            .send()
            .await?;

        let exec_id = res1
            .split("\"Id\":\"")
            .nth(1)
            .and_then(|s| s.split('"').next())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to parse exec Id from Docker response: {}", res1),
                )
            })?
            .to_string();

        // Step 2: Start & Upgrade Socket Connection via SocatRequestBuilder
        let start_path = format!("/exec/{}/start", exec_id);
        let start_payload = serde_json::json!({
            "Detach": false,
            "Tty": self.tty
        });
        let (stream, _header) = SocatRequestBuilder::post(start_path)
            .socket_path(&self.socket_path)
            .json(&start_payload)
            .upgrade()
            .await?;

        let (reader, writer) = stream.into_split();
        Ok(ContainerSocketExecStream {
            reader,
            writer,
            exec_id,
            socket_path: self.socket_path,
        })
    }
}

pub struct ContainerSocketExecStream {
    pub reader: tokio::net::unix::OwnedReadHalf,
    pub writer: tokio::net::unix::OwnedWriteHalf,
    pub exec_id: String,
    pub socket_path: String,
}

impl ContainerSocketExecStream {
    pub async fn resize(&self, width: u16, height: u16) -> std::io::Result<()> {
        resize_container_exec(&self.socket_path, &self.exec_id, width, height).await
    }
}

pub async fn resize_container_exec(
    socket_path: &str,
    exec_id: &str,
    width: u16,
    height: u16,
) -> std::io::Result<()> {
    let path = format!("/exec/{}/resize", exec_id);
    let payload = serde_json::json!({
        "Height": height,
        "Width": width
    });

    let _ = SocatRequestBuilder::post(path)
        .query("h", height.to_string())
        .query("w", width.to_string())
        .json(&payload)
        .socket_path(socket_path)
        .send()
        .await?;
    Ok(())
}

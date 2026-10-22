use crate::docker::{
    DockerOutput, DockerResult,
    client::DockerCli,
    core::{ArgBuilder, types::DataSource},
};

pub struct SecretCreateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
    data_source: Option<DataSource>,
}

impl<'a> SecretCreateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["secret", "create"]),
            name: name.into(),
            data_source: None,
        }
    }

    pub fn from_file(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.data_source = Some(DataSource::from_file(path));
        self
    }
    pub fn from_string(mut self, data: impl Into<String>) -> Self {
        self.data_source = Some(DataSource::from_string(data));
        self
    }
    pub fn from_bytes(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.data_source = Some(DataSource::from_bytes(data));
        self
    }

    pub fn label(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.args
            .pair("--label", format!("{}={}", key.as_ref(), value.as_ref()));
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        let source = self.data_source.take().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Data source required for secret creation",
            )
        })?;
        if self.args.api {
            use base64::Engine;
            let data = match source {
                DataSource::File(path) => tokio::fs::read(path).await?,
                DataSource::String(data) => data.into_bytes(),
                DataSource::Bytes(data) => data,
            };
            let mut labels = serde_json::Map::new();
            let built = self.args.clone().build();
            for pair in built.windows(2) {
                if pair[0] == "--label"
                    && let Some((k, v)) = pair[1].split_once('=')
                {
                    labels.insert(k.into(), serde_json::json!(v));
                }
            }
            return self.cli.api_json_request("/secrets/create", serde_json::json!({"Name": self.name, "Labels": labels, "Data": base64::engine::general_purpose::STANDARD.encode(data)})).await;
        }
        self.args.push(&self.name);

        match source {
            DataSource::File(path) => {
                self.args.push(path.to_string_lossy().to_string());
                self.cli.execute(&self.args).await
            }
            DataSource::String(data) => {
                self.args.push("-");
                self.cli
                    .run_with_stdin(&self.args.build(), data.as_bytes())
                    .await
            }
            DataSource::Bytes(data) => {
                self.args.push("-");
                self.cli.run_with_stdin(&self.args.build(), &data).await
            }
        }
    }
}
crate::impl_builder_opts!(SecretCreateBuilder);

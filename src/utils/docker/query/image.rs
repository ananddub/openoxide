use super::filter::ImageFilter;
use crate::utils::docker::{DockerCli, DockerResult, ImageSummary};

/// Fluent builder for `docker image ls`.
pub struct ImageQuery<'a> {
    cli: &'a DockerCli,
    all: bool,
    filters: Vec<ImageFilter>,
}

impl<'a> ImageQuery<'a> {
    pub(super) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            all: false,
            filters: vec![],
        }
    }

    /// Include intermediate layers (`--all`).
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    pub fn filter(mut self, f: ImageFilter) -> Self {
        self.filters.push(f);
        self
    }
    pub fn filters(mut self, fs: impl IntoIterator<Item = ImageFilter>) -> Self {
        self.filters.extend(fs);
        self
    }

    fn args(&self) -> Vec<String> {
        let mut a = vec![
            "image".into(),
            "ls".into(),
            "--format".into(),
            "{{json .}}".into(),
        ];
        if self.all {
            a.push("--all".into());
        }
        for f in &self.filters {
            a.extend(["--filter".into(), f.to_string()]);
        }
        a
    }

    pub async fn list(self) -> DockerResult<Vec<ImageSummary>> {
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json_lines(&refs).await
    }

    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}

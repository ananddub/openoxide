use crate::utils::docker::{DockerCli, DockerOutput, DockerResult, core::ArgBuilder};

pub struct ImagePushBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) image: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImagePushBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            image: image.into(),
            args: ArgBuilder::cmd(&["image", "push"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.image);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ImagePushBuilder);

pub struct ImageRmBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) image: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImageRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            image: image.into(),
            args: ArgBuilder::cmd(&["image", "rm"]),
        }
    }
    pub fn force(mut self) -> Self {
        self.args.flag("--force");
        self
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.image);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ImageRmBuilder);

pub struct ImageTagBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImageTagBuilder<'a> {
    pub(crate) fn new(
        cli: &'a DockerCli,
        source: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            cli,
            source: source.into(),
            target: target.into(),
            args: ArgBuilder::cmd(&["image", "tag"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.source);
        a.push(&self.target);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ImageTagBuilder);

pub struct ImageHistoryBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) image: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImageHistoryBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            image: image.into(),
            args: ArgBuilder::cmd(&["image", "history"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.image);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ImageHistoryBuilder);

pub struct ImageSaveBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) image: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImageSaveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            image: image.into(),
            args: ArgBuilder::cmd(&["image", "save"]),
        }
    }
    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--output", path.into());
        self
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.image);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ImageSaveBuilder);

pub struct ImageLoadBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImageLoadBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["image", "load"]),
        }
    }
    pub fn input(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--input", path.into());
        self
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ImageLoadBuilder);

pub struct ImageImportBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) source: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImageImportBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, source: impl Into<String>) -> Self {
        Self {
            cli,
            source: source.into(),
            args: ArgBuilder::cmd(&["image", "import"]),
        }
    }
    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.args.pair("--message", msg.into());
        self
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.source);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ImageImportBuilder);

// IntoCommand Impls
impl crate::utils::exec::script::IntoCommand for ImagePushBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.image);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ImageRmBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.image);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ImageTagBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.source);
        a.push(&self.target);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ImageHistoryBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.image);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ImageSaveBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.image);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ImageLoadBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ImageImportBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.source);
        a.preview()
    }
}

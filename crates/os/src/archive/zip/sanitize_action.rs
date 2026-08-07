use super::ZipError;
use crate::exec::CommandExecutor;
use std::path::PathBuf;

pub struct ZipSanitizeBuilder<'a> {
    executor: &'a CommandExecutor,
    input: PathBuf,
    output: Option<PathBuf>,
}
impl<'a> ZipSanitizeBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, input: PathBuf) -> Self {
        Self {
            executor,
            input,
            output: None,
        }
    }
    pub fn to(mut self, output: impl Into<PathBuf>) -> Self {
        self.output = Some(output.into());
        self
    }
    pub async fn run(self) -> Result<(), ZipError> {
        let output = self.output.ok_or(ZipError::MissingDestination)?;
        super::sanitize::sanitize_zip_with_executor(self.executor, &self.input, &output).await
    }
}

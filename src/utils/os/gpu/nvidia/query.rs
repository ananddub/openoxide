use super::{NvidiaQueryField, NvidiaQueryFormat};
use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct NvidiaQueryBuilder<'a> {
    executor: &'a CommandExecutor,
    fields: Vec<NvidiaQueryField>,
    format: NvidiaQueryFormat,
    header: bool,
    units: bool,
}
impl<'a> NvidiaQueryBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            fields: Vec::new(),
            format: NvidiaQueryFormat::Csv,
            header: true,
            units: true,
        }
    }
    pub fn field(mut self, field: NvidiaQueryField) -> Self {
        if !self.fields.contains(&field) {
            self.fields.push(field);
        }
        self
    }
    pub fn fields(mut self, fields: impl IntoIterator<Item = NvidiaQueryField>) -> Self {
        for field in fields {
            if !self.fields.contains(&field) {
                self.fields.push(field);
            }
        }
        self
    }
    pub fn format(mut self, format: NvidiaQueryFormat) -> Self {
        self.format = format;
        self
    }
    pub fn without_header(mut self) -> Self {
        self.header = false;
        self
    }
    pub fn without_units(mut self) -> Self {
        self.units = false;
        self
    }
    fn args(&self) -> Vec<String> {
        let fields = if self.fields.is_empty() {
            vec![NvidiaQueryField::Name, NvidiaQueryField::DriverVersion]
        } else {
            self.fields.clone()
        };
        let query = fields
            .iter()
            .map(|field| field.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let mut format = self.format.as_str().to_owned();
        if !self.header {
            format.push_str(",noheader");
        }
        if !self.units {
            format.push_str(",nounits");
        }
        vec![format!("--query-gpu={query}"), format!("--format={format}")]
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("nvidia-smi", self.args()).await
    }
}
impl IntoCommand for NvidiaQueryBuilder<'_> {
    fn build_str(&self) -> String {
        format!(
            "nvidia-smi {}",
            self.args()
                .iter()
                .map(escape_arg)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

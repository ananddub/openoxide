use crate::exec::CommandExecutor;

pub mod nvidia;

pub use nvidia::{
    ContainerRuntime, NvidiaConfigureBuilder, NvidiaGpuBuilder, NvidiaQueryBuilder,
    NvidiaQueryField, NvidiaQueryFormat,
};

pub struct GpuCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> GpuCli<'a> {
    pub fn nvidia(&self) -> NvidiaGpuBuilder<'a> {
        NvidiaGpuBuilder::new(self.executor)
    }
}

#[cfg(test)]
mod tests {
    use super::{ContainerRuntime, NvidiaQueryField, NvidiaQueryFormat};
    use crate::exec::script::IntoCommand;
    use crate::exec::{CommandExecutor, LocalExecutor};
    use crate::OsCli;

    #[test]
    fn nvidia_actions_build_commands() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let os = OsCli::new(&executor);

        assert_eq!(
            os.gpu()
                .nvidia()
                .query()
                .field(NvidiaQueryField::Name)
                .field(NvidiaQueryField::DriverVersion)
                .format(NvidiaQueryFormat::Csv)
                .without_header()
                .build_str(),
            "nvidia-smi '--query-gpu=name,driver_version' '--format=csv,noheader'"
        );
        assert_eq!(
            os.gpu()
                .nvidia()
                .configure()
                .runtime(ContainerRuntime::Docker)
                .build_str(),
            "nvidia-ctk 'runtime' 'configure' '--runtime=docker'"
        );
    }
}

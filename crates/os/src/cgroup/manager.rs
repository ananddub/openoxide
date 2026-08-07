use super::{
    cpu::CpuLimit, cpuset::CpusetLimit, error::CgroupError, freezer::FreezeState, io::IoLimit,
    memory::MemoryLimit, pids::PidsLimit,
};
use crate::exec::CommandExecutor;

#[derive(Clone, Debug)]
pub struct Cgroup {
    pub(crate) name: String,
    pub(crate) base_path: String,
    pub(crate) executor: CommandExecutor,
    pub(crate) memory_limit: Option<MemoryLimit>,
    pub(crate) memory_high: Option<MemoryLimit>,
    pub(crate) memory_low: Option<MemoryLimit>,
    pub(crate) memory_swap: Option<MemoryLimit>,
    pub(crate) cpu_limit: Option<CpuLimit>,
    pub(crate) cpu_weight: Option<u32>,
    pub(crate) cpuset: Option<CpusetLimit>,
    pub(crate) pids_limit: Option<PidsLimit>,
    pub(crate) io_limit: Option<IoLimit>,
}

impl Cgroup {
    pub fn open(name: impl Into<String>, executor: CommandExecutor) -> Self {
        Self {
            name: name.into(),
            base_path: "/sys/fs/cgroup".into(),
            executor,
            memory_limit: None,
            memory_high: None,
            memory_low: None,
            memory_swap: None,
            cpu_limit: None,
            cpu_weight: None,
            cpuset: None,
            pids_limit: None,
            io_limit: None,
        }
    }

    pub fn with_base_path(mut self, path: impl Into<String>) -> Self {
        self.base_path = path.into();
        self
    }

    pub fn cgroup_path(&self) -> String {
        format!("{}/{}", self.base_path, self.name)
    }

    pub fn validate(&self) -> Result<(), CgroupError> {
        let to_bytes = |limit: &MemoryLimit| -> Option<u64> {
            match limit {
                MemoryLimit::B(b) => Some(*b),
                MemoryLimit::KB(k) => Some(k * 1024),
                MemoryLimit::MB(m) => Some(m * 1024 * 1024),
                MemoryLimit::GB(g) => Some(g * 1024 * 1024 * 1024),
                MemoryLimit::Max => None,
            }
        };

        let mem_max = self.memory_limit.as_ref().and_then(to_bytes);
        let mem_high = self.memory_high.as_ref().and_then(to_bytes);
        let mem_low = self.memory_low.as_ref().and_then(to_bytes);

        if let (Some(low), Some(high)) = (mem_low, mem_high) {
            if low > high {
                return Err(CgroupError::ValidationError(
                    "memory_low cannot be greater than memory_high".into(),
                ));
            }
        }

        if let (Some(high), Some(max)) = (mem_high, mem_max) {
            if high > max {
                return Err(CgroupError::ValidationError(
                    "memory_high cannot be greater than memory_limit (max)".into(),
                ));
            }
        }

        if let (Some(low), Some(max)) = (mem_low, mem_max) {
            if low > max {
                return Err(CgroupError::ValidationError(
                    "memory_low cannot be greater than memory_limit (max)".into(),
                ));
            }
        }

        if let Some(CpuLimit::Cores(c)) = self.cpu_limit {
            if c <= 0.0 {
                return Err(CgroupError::ValidationError(
                    "cpu limit cores must be positive".into(),
                ));
            }
        }
        if let Some(CpuLimit::Millicores(m)) = self.cpu_limit {
            if m == 0 {
                return Err(CgroupError::ValidationError(
                    "cpu limit millicores must be greater than zero".into(),
                ));
            }
        }

        if let Some(w) = self.cpu_weight {
            if !(1..=10000).contains(&w) {
                return Err(CgroupError::ValidationError(
                    "cpu weight must be between 1 and 10000".into(),
                ));
            }
        }

        Ok(())
    }

    pub async fn available_controllers(&self) -> Result<Vec<String>, CgroupError> {
        let path = format!("{}/cgroup.controllers", self.base_path);
        let output = self
            .executor
            .read_file(&path)
            .await
            .map_err(|e| CgroupError::ExecutorError(e.to_string()))?;
        if !output.success() {
            return Err(CgroupError::ReadFailed {
                path,
                error: output.combined_output(),
            });
        }
        Ok(output
            .stdout_trimmed()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect())
    }

    pub async fn create(&self) -> Result<(), CgroupError> {
        let cgroup_dir = self.cgroup_path();
        let output = crate::OsCli::new(&self.executor)
            .dir(&cgroup_dir)
            .create()
            .run()
            .await
            .map_err(|e| CgroupError::ExecutorError(e.to_string()))?;
        if !output.success() {
            return Err(CgroupError::CreateFailed(output.combined_output()));
        }
        Ok(())
    }

    pub async fn enable_subtree_control(&self) -> Result<(), CgroupError> {
        let parts: Vec<&str> = self.name.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() <= 1 {
            return Ok(());
        }

        let available = self.available_controllers().await?;
        let mut enabled_str = String::new();
        for ctrl in &["cpu", "memory", "pids", "io", "cpuset"] {
            if available.contains(&ctrl.to_string()) {
                enabled_str.push_str(&format!(" +{}", ctrl));
            }
        }
        let enabled_str = enabled_str.trim().to_string();
        if enabled_str.is_empty() {
            return Ok(());
        }

        let mut current_path = String::new();
        for i in 0..(parts.len() - 1) {
            if !current_path.is_empty() {
                current_path.push('/');
            }
            current_path.push_str(parts[i]);

            let parent_dir = format!("{}/{}", self.base_path, current_path);
            let parent_subtree = format!("{}/cgroup.subtree_control", parent_dir);

            let mkdir_out = crate::OsCli::new(&self.executor)
                .dir(&parent_dir)
                .create()
                .run()
                .await
                .map_err(|e| CgroupError::ExecutorError(e.to_string()))?;
            if !mkdir_out.success() {
                return Err(CgroupError::CreateFailed(mkdir_out.combined_output()));
            }

            let write_out = self
                .executor
                .write_file(&parent_subtree, &enabled_str)
                .await
                .map_err(|e| CgroupError::ExecutorError(e.to_string()))?;
            if !write_out.success() {
                return Err(CgroupError::WriteFailed {
                    path: parent_subtree,
                    error: write_out.combined_output(),
                });
            }
        }

        Ok(())
    }

    pub async fn apply(&self) -> Result<(), CgroupError> {
        self.validate()?;
        self.enable_subtree_control().await?;
        self.create().await?;

        let available = self.available_controllers().await?;

        if self.memory_limit.is_some()
            || self.memory_high.is_some()
            || self.memory_low.is_some()
            || self.memory_swap.is_some()
        {
            if !available.contains(&"memory".to_string()) {
                return Err(CgroupError::ControllerUnavailable {
                    controller: "memory".into(),
                });
            }
            if let Some(ref max) = self.memory_limit {
                self.write_file("memory.max", &max.to_cgroup_value())
                    .await?;
            }
            if let Some(ref high) = self.memory_high {
                self.write_file("memory.high", &high.to_cgroup_value())
                    .await?;
            }
            if let Some(ref low) = self.memory_low {
                self.write_file("memory.low", &low.to_cgroup_value())
                    .await?;
            }
            if let Some(ref swap) = self.memory_swap {
                self.write_file("memory.swap.max", &swap.to_cgroup_value())
                    .await?;
            }
        }

        if self.cpu_limit.is_some() || self.cpu_weight.is_some() {
            if !available.contains(&"cpu".to_string()) {
                return Err(CgroupError::ControllerUnavailable {
                    controller: "cpu".into(),
                });
            }
            if let Some(ref limit) = self.cpu_limit {
                self.write_file("cpu.max", &limit.to_cgroup_value()).await?;
            }
            if let Some(weight) = self.cpu_weight {
                self.write_file("cpu.weight", &weight.to_string()).await?;
            }
        }

        if let Some(ref cpuset) = self.cpuset {
            if !available.contains(&"cpuset".to_string()) {
                return Err(CgroupError::ControllerUnavailable {
                    controller: "cpuset".into(),
                });
            }
            self.write_file("cpuset.cpus", cpuset.to_cgroup_value())
                .await?;
        }

        if let Some(ref limit) = self.pids_limit {
            if !available.contains(&"pids".to_string()) {
                return Err(CgroupError::ControllerUnavailable {
                    controller: "pids".into(),
                });
            }
            self.write_file("pids.max", &limit.to_cgroup_value())
                .await?;
        }

        if let Some(ref limit) = self.io_limit {
            if !available.contains(&"io".to_string()) {
                return Err(CgroupError::ControllerUnavailable {
                    controller: "io".into(),
                });
            }
            self.write_file("io.max", limit.to_cgroup_value()).await?;
        }

        Ok(())
    }

    pub fn to_apply_commands(&self) -> Result<Vec<String>, CgroupError> {
        self.validate()?;
        let mut cmds = Vec::new();
        let path = self.cgroup_path();

        // 1. Create the cgroup directory
        cmds.push(format!("mkdir -p {}", shell_single_quote(&path)));

        // 2. Enable subtree controls in parent directories if nested
        let parts: Vec<&str> = self.name.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() > 1 {
            let mut current_path = String::new();
            for i in 0..(parts.len() - 1) {
                if !current_path.is_empty() {
                    current_path.push('/');
                }
                current_path.push_str(parts[i]);

                let parent_dir = format!("{}/{}", self.base_path, current_path);
                cmds.push(format!("mkdir -p {}", shell_single_quote(&parent_dir)));

                // Write enabled controllers to subtree_control
                let parent_controllers = format!("{}/cgroup.controllers", parent_dir);
                let parent_subtree = format!("{}/cgroup.subtree_control", parent_dir);
                cmds.push(format!(
                    "if [ -f {0} ]; then for ctrl in cpu memory pids io cpuset; do if grep -q \"$ctrl\" {0}; then echo \"+$ctrl\" > {1}; fi; done; fi",
                    shell_single_quote(&parent_controllers),
                    shell_single_quote(&parent_subtree)
                ));
            }
        }

        // 3. Write limits conditionally (checking if control files exist)
        if let Some(ref max) = self.memory_limit {
            let filepath = format!("{}/memory.max", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(&max.to_cgroup_value())
            ));
        }
        if let Some(ref high) = self.memory_high {
            let filepath = format!("{}/memory.high", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(&high.to_cgroup_value())
            ));
        }
        if let Some(ref low) = self.memory_low {
            let filepath = format!("{}/memory.low", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(&low.to_cgroup_value())
            ));
        }
        if let Some(ref swap) = self.memory_swap {
            let filepath = format!("{}/memory.swap.max", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(&swap.to_cgroup_value())
            ));
        }

        if let Some(ref limit) = self.cpu_limit {
            let filepath = format!("{}/cpu.max", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(&limit.to_cgroup_value())
            ));
        }
        if let Some(weight) = self.cpu_weight {
            let filepath = format!("{}/cpu.weight", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(&weight.to_string())
            ));
        }

        if let Some(ref cpuset) = self.cpuset {
            let filepath = format!("{}/cpuset.cpus", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(cpuset.to_cgroup_value())
            ));
        }

        if let Some(ref limit) = self.pids_limit {
            let filepath = format!("{}/pids.max", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(&limit.to_cgroup_value())
            ));
        }

        if let Some(ref limit) = self.io_limit {
            let filepath = format!("{}/io.max", path);
            cmds.push(format!(
                "if [ -f {0} ]; then echo {1} > {0}; fi",
                shell_single_quote(&filepath),
                shell_single_quote(limit.to_cgroup_value())
            ));
        }

        Ok(cmds)
    }

    pub fn to_add_process_command(&self, pid: &str) -> String {
        let filepath = format!("{}/cgroup.procs", self.cgroup_path());
        format!(
            "if [ -f {0} ]; then echo {1} > {0}; fi",
            shell_single_quote(&filepath),
            shell_single_quote(pid)
        )
    }

    pub fn to_add_current_process_command(&self) -> String {
        self.to_add_process_command("$$")
    }

    pub async fn add_process(&self, pid: u32) -> Result<(), CgroupError> {
        self.add_processes(&[pid]).await
    }

    pub async fn add_processes(&self, pids: &[u32]) -> Result<(), CgroupError> {
        self.create().await?;
        let filepath = format!("{}/cgroup.procs", self.cgroup_path());
        for &pid in pids {
            let out = self
                .executor
                .write_file(&filepath, &pid.to_string())
                .await
                .map_err(|e| CgroupError::ExecutorError(e.to_string()))?;
            if !out.success() {
                return Err(CgroupError::WriteFailed {
                    path: filepath,
                    error: out.combined_output(),
                });
            }
        }
        Ok(())
    }

    pub async fn freeze(&self, state: FreezeState) -> Result<(), CgroupError> {
        self.write_file("cgroup.freeze", state.to_cgroup_value())
            .await
    }

    pub async fn delete(&self) -> Result<(), CgroupError> {
        let path = self.cgroup_path();
        match crate::OsCli::new(&self.executor)
            .dir(&path)
            .remove_if_empty()
            .run()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("No such file or directory")
                    || err_str.contains("does not exist")
                {
                    Ok(()) // Idempotent success
                } else {
                    Err(CgroupError::ExecutorError(err_str))
                }
            }
        }
    }

    pub async fn force_delete(&self) -> Result<(), CgroupError> {
        let path = self.cgroup_path();

        // 1. Idempotency Check: if directory doesn't exist, we are done
        let os = crate::OsCli::new(&self.executor);
        let check_dir = os.dir(&path).exists().run().await;
        if let Err(e) = check_dir {
            let err_str = e.to_string();
            if err_str.contains("exit code Some(1)") || err_str.contains("No such file") {
                return Ok(());
            }
            return Err(CgroupError::ExecutorError(err_str));
        }

        let kill_path = format!("{}/cgroup.kill", path);

        // 2. Try atomic termination if supported (kernel 5.14+)
        let check_kill_file = os.file(&kill_path).exists().run().await;
        let has_kill_file = match check_kill_file {
            Ok(_) => true,
            Err(e) => {
                let err_str = e.to_string();
                !err_str.contains("exit code Some(1)") && !err_str.contains("No such file")
            }
        };

        if has_kill_file {
            let write_res = self.executor.write_file(&kill_path, "1").await;
            match write_res {
                Ok(_) => {}
                Err(e) => {
                    tracing::warn!(
                        path = %kill_path,
                        error = %e,
                        "failed to write 1 to cgroup.kill, falling back to manual kill"
                    );
                    self.fallback_kill().await;
                }
            }
        } else {
            // 3. Fallback: Manual TERM-then-KILL loop
            self.fallback_kill().await;
        }

        // 4. Delete directory with retry backoff
        self.delete_with_retry(5).await
    }

    async fn fallback_kill(&self) {
        if let Ok(pids) = self.processes().await {
            // Send SIGTERM first for graceful application cleanup
            for pid in &pids {
                if let Err(e) = self
                    .executor
                    .run("kill", &["-TERM", &pid.to_string()])
                    .await
                {
                    tracing::warn!(pid = %pid, error = %e, "failed to send SIGTERM to process");
                }
            }

            // Allow a short grace period (1.5 seconds) for reaped children
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

            // Send SIGKILL (-9) to any remaining/zombie processes
            if let Ok(remaining) = self.processes().await {
                for pid in remaining {
                    if let Err(e) = self.executor.run("kill", &["-9", &pid.to_string()]).await {
                        tracing::warn!(pid = %pid, error = %e, "failed to send SIGKILL to process");
                    }
                }
            }
        }
    }

    async fn delete_with_retry(&self, attempts: u32) -> Result<(), CgroupError> {
        for i in 0..attempts {
            match self.delete().await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if i < attempts - 1 {
                        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }

    // --- Read API ---

    pub async fn memory_current(&self) -> Result<u64, CgroupError> {
        let content = self.read_file("memory.current").await?;
        content
            .trim()
            .parse::<u64>()
            .map_err(|e| CgroupError::ReadFailed {
                path: format!("{}/memory.current", self.cgroup_path()),
                error: e.to_string(),
            })
    }

    pub async fn cpu_stat(&self) -> Result<String, CgroupError> {
        self.read_file("cpu.stat").await
    }

    pub async fn events(&self) -> Result<String, CgroupError> {
        self.read_file("cgroup.events").await
    }

    pub async fn processes(&self) -> Result<Vec<u32>, CgroupError> {
        let content = self.read_file("cgroup.procs").await?;
        let mut pids = Vec::new();
        for line in content.lines() {
            if let Ok(pid) = line.trim().parse::<u32>() {
                pids.push(pid);
            }
        }
        Ok(pids)
    }

    async fn write_file(&self, filename: &str, content: &str) -> Result<(), CgroupError> {
        let filepath = format!("{}/{}", self.cgroup_path(), filename);
        let output = self
            .executor
            .write_file(&filepath, content)
            .await
            .map_err(|e| CgroupError::ExecutorError(e.to_string()))?;
        if !output.success() {
            return Err(CgroupError::WriteFailed {
                path: filepath,
                error: output.combined_output(),
            });
        }
        Ok(())
    }

    async fn read_file(&self, filename: &str) -> Result<String, CgroupError> {
        let filepath = format!("{}/{}", self.cgroup_path(), filename);
        let output = self
            .executor
            .read_file(&filepath)
            .await
            .map_err(|e| CgroupError::ExecutorError(e.to_string()))?;
        if !output.success() {
            return Err(CgroupError::ReadFailed {
                path: filepath,
                error: output.combined_output(),
            });
        }
        Ok(output.stdout_trimmed().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cgroup::builder::CgroupBuilder;
    use crate::exec::LocalExecutor;

    #[test]
    fn test_memory_limit_formatting() {
        assert_eq!(MemoryLimit::B(100).to_cgroup_value(), "100");
        assert_eq!(MemoryLimit::KB(256).to_cgroup_value(), "256K");
        assert_eq!(MemoryLimit::MB(512).to_cgroup_value(), "512M");
        assert_eq!(MemoryLimit::GB(4).to_cgroup_value(), "4G");
        assert_eq!(MemoryLimit::Max.to_cgroup_value(), "max");
    }

    #[test]
    fn test_cpu_limit_formatting() {
        assert_eq!(CpuLimit::Cores(1.5).to_cgroup_value(), "150000 100000");
        assert_eq!(CpuLimit::Millicores(500).to_cgroup_value(), "50000 100000");
        assert_eq!(CpuLimit::Max.to_cgroup_value(), "max");
    }

    #[test]
    fn test_cgroup_path_formatting() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cg = CgroupBuilder::new("project/api", executor.clone())
            .with_base_path("/tmp/cgroup")
            .build();
        assert_eq!(cg.cgroup_path(), "/tmp/cgroup/project/api");
    }

    #[test]
    fn test_validation_logic() {
        let executor = CommandExecutor::Local(LocalExecutor::new());

        // memory_low > memory_high should fail validation
        let cg = CgroupBuilder::new("test", executor.clone())
            .memory_low(MemoryLimit::MB(1024))
            .memory_high(MemoryLimit::MB(512))
            .build();
        assert!(cg.validate().is_err());

        // memory_high > memory_limit should fail validation
        let cg2 = CgroupBuilder::new("test", executor.clone())
            .memory_high(MemoryLimit::GB(2))
            .memory(MemoryLimit::GB(1))
            .build();
        assert!(cg2.validate().is_err());

        // invalid cpu core limit should fail validation
        let cg3 = CgroupBuilder::new("test", executor.clone())
            .cpu(CpuLimit::Cores(-0.5))
            .build();
        assert!(cg3.validate().is_err());

        // invalid weight limit should fail validation
        let cg4 = CgroupBuilder::new("test", executor.clone())
            .cpu_weight(20000)
            .build();
        assert!(cg4.validate().is_err());

        // Correct configurations should succeed
        let cg5 = CgroupBuilder::new("test", executor.clone())
            .memory_low(MemoryLimit::MB(256))
            .memory_high(MemoryLimit::MB(512))
            .memory(MemoryLimit::GB(1))
            .cpu(CpuLimit::Cores(2.0))
            .cpu_weight(5000)
            .build();
        assert!(cg5.validate().is_ok());
    }

    #[tokio::test]
    async fn test_delete_idempotency() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cg = CgroupBuilder::new("non_existent_cgroup_12345", executor)
            .with_base_path("target/tmp/non_existent_base")
            .build();
        let res = cg.delete().await;
        println!("DELETE RESULT: {:?}", res);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_force_delete_idempotency() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cg = CgroupBuilder::new("non_existent_cgroup_67890", executor)
            .with_base_path("target/tmp/non_existent_base")
            .build();
        // Force deleting non-existent should succeed idempotently
        assert!(cg.force_delete().await.is_ok());
    }

    #[tokio::test]
    async fn test_force_delete_concurrency() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cg = CgroupBuilder::new("concurrent_cgroup_test", executor)
            .with_base_path("target/tmp/concurrent_base")
            .build();

        let cg_arc = std::sync::Arc::new(cg);
        let mut handles = vec![];
        for _ in 0..5 {
            let cg_clone = cg_arc.clone();
            handles.push(tokio::spawn(async move { cg_clone.force_delete().await }));
        }
        for handle in handles {
            let res = handle.await;
            assert!(res.is_ok());
            assert!(res.unwrap().is_ok());
        }
    }

    #[test]
    fn test_cgroup_apply_commands_compilation() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let cg = CgroupBuilder::new("project/api", executor)
            .with_base_path("/sys/fs/cgroup")
            .memory(MemoryLimit::MB(512))
            .cpu(CpuLimit::Cores(2.0))
            .build();

        let cmds = cg.to_apply_commands().unwrap();
        let script = cmds.join("\n");
        assert!(script.contains("mkdir -p '/sys/fs/cgroup/project/api'"));
        assert!(script.contains("mkdir -p '/sys/fs/cgroup/project'"));
        assert!(script.contains("memory.max"));
        assert!(script.contains("cpu.max"));

        let add_proc = cg.to_add_current_process_command();
        assert!(add_proc.contains("cgroup.procs"));
        assert!(add_proc.contains("$$"));
    }
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

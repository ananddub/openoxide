use crate::utils::{
    cgroup::{CpuLimit, MemoryLimit},
    exec::CommandExecutor,
};

pub fn parse_memory_limit(s: &str) -> Option<MemoryLimit> {
    let s = s.trim().to_lowercase();
    if s == "max" {
        return Some(MemoryLimit::Max);
    }
    let num_str = s
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let suffix = s
        .chars()
        .skip_while(|c| c.is_ascii_digit())
        .collect::<String>();
    let val: u64 = num_str.parse().ok()?;

    match suffix.trim() {
        "k" | "kb" => Some(MemoryLimit::KB(val)),
        "m" | "mb" => Some(MemoryLimit::MB(val)),
        "g" | "gb" => Some(MemoryLimit::GB(val)),
        "b" | "" => Some(MemoryLimit::B(val)),
        _ => None,
    }
}

pub fn parse_cpu_limit(s: &str) -> Option<CpuLimit> {
    let s = s.trim().to_lowercase();
    if s == "max" {
        return Some(CpuLimit::Max);
    }
    if let Ok(cores) = s.parse::<f32>() {
        return Some(CpuLimit::Cores(cores));
    }
    None
}

pub async fn get_total_memory_kb(executor: &CommandExecutor) -> Option<u64> {
    if let Ok(res) = executor.run("cat", &["/proc/meminfo"]).await {
        for line in res.stdout.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<u64>().ok();
                }
            }
        }
    }
    None
}

pub async fn get_cpu_cores(executor: &CommandExecutor) -> Option<f32> {
    if let Ok(res) = executor.run("nproc", &[] as &[&str]).await {
        if let Ok(cores) = res.stdout.trim().parse::<f32>() {
            return Some(cores);
        }
    }
    if let Ok(res) = executor.run("cat", &["/proc/cpuinfo"]).await {
        let mut count = 0;
        for line in res.stdout.lines() {
            if line.starts_with("processor") {
                count += 1;
            }
        }
        if count > 0 {
            return Some(count as f32);
        }
    }
    None
}

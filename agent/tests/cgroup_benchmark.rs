use std::collections::HashMap;
use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};

use agent::docker::cgroup::CgroupReader;

#[derive(Debug, Default)]
struct DetailedCgroupSample {
    // CPU
    usage_usec: u64,
    user_usec: u64,
    system_usec: u64,
    nr_periods: u64,
    nr_throttled: u64,
    throttled_usec: u64,

    // Memory
    memory_current: u64,
    memory_max: u64,
    memory_high: u64,
    anon: u64,
    file: u64,
    kernel_stack: u64,
    slab: u64,
    sock: u64,
    file_mapped: u64,
    file_dirty: u64,
    file_writeback: u64,
    inactive_anon: u64,
    active_anon: u64,
    inactive_file: u64,
    active_file: u64,
    pgfault: u64,
    pgmajfault: u64,

    // Memory Events
    oom_events: u64,
    oom_kill_events: u64,

    // I/O
    io_read_bytes: u64,
    io_write_bytes: u64,
    io_read_ops: u64,
    io_write_ops: u64,

    // PIDs
    pids_current: u64,
    pids_max: u64,
}

fn parse_kv_file(content: &str) -> HashMap<String, u64> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
            if let Ok(v) = val.parse::<u64>() {
                map.insert(key.to_string(), v);
            }
        }
    }
    map
}

fn read_detailed_cgroup(dir: &std::path::Path) -> Option<DetailedCgroupSample> {
    let mut s = DetailedCgroupSample::default();

    // CPU Stat
    if let Ok(cpu_str) = fs::read_to_string(dir.join("cpu.stat")) {
        let kv = parse_kv_file(&cpu_str);
        s.usage_usec = *kv.get("usage_usec").unwrap_or(&0);
        s.user_usec = *kv.get("user_usec").unwrap_or(&0);
        s.system_usec = *kv.get("system_usec").unwrap_or(&0);
        s.nr_periods = *kv.get("nr_periods").unwrap_or(&0);
        s.nr_throttled = *kv.get("nr_throttled").unwrap_or(&0);
        s.throttled_usec = *kv.get("throttled_usec").unwrap_or(&0);
    } else {
        return None;
    }

    // Memory Current
    s.memory_current = fs::read_to_string(dir.join("memory.current"))
        .ok()?
        .trim()
        .parse()
        .unwrap_or(0);

    // Memory Max
    let max_str = fs::read_to_string(dir.join("memory.max")).unwrap_or_default();
    s.memory_max = max_str.trim().parse().unwrap_or(u64::MAX);

    // Memory High
    let high_str = fs::read_to_string(dir.join("memory.high")).unwrap_or_default();
    s.memory_high = high_str.trim().parse().unwrap_or(u64::MAX);

    // Memory Stat
    if let Ok(mem_stat_str) = fs::read_to_string(dir.join("memory.stat")) {
        let kv = parse_kv_file(&mem_stat_str);
        s.anon = *kv.get("anon").unwrap_or(&0);
        s.file = *kv.get("file").unwrap_or(&0);
        s.kernel_stack = *kv.get("kernel_stack").unwrap_or(&0);
        s.slab = *kv.get("slab").unwrap_or(&0);
        s.sock = *kv.get("sock").unwrap_or(&0);
        s.file_mapped = *kv.get("file_mapped").unwrap_or(&0);
        s.file_dirty = *kv.get("file_dirty").unwrap_or(&0);
        s.file_writeback = *kv.get("file_writeback").unwrap_or(&0);
        s.inactive_anon = *kv.get("inactive_anon").unwrap_or(&0);
        s.active_anon = *kv.get("active_anon").unwrap_or(&0);
        s.inactive_file = *kv.get("inactive_file").unwrap_or(&0);
        s.active_file = *kv.get("active_file").unwrap_or(&0);
        s.pgfault = *kv.get("pgfault").unwrap_or(&0);
        s.pgmajfault = *kv.get("pgmajfault").unwrap_or(&0);
    }

    // Memory Events
    if let Ok(mem_evt_str) = fs::read_to_string(dir.join("memory.events")) {
        let kv = parse_kv_file(&mem_evt_str);
        s.oom_events = *kv.get("oom").unwrap_or(&0);
        s.oom_kill_events = *kv.get("oom_kill").unwrap_or(&0);
    }

    // IO Stat
    if let Ok(io_str) = fs::read_to_string(dir.join("io.stat")) {
        for line in io_str.lines() {
            let mut parts = line.split_whitespace();
            let _dev = parts.next();
            for item in parts {
                if let Some((k, v)) = item.split_once('=') {
                    if let Ok(val) = v.parse::<u64>() {
                        match k {
                            "rbytes" => s.io_read_bytes += val,
                            "wbytes" => s.io_write_bytes += val,
                            "rios" => s.io_read_ops += val,
                            "wios" => s.io_write_ops += val,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // PIDs Current
    if let Ok(pids_str) = fs::read_to_string(dir.join("pids.current")) {
        s.pids_current = pids_str.trim().parse().unwrap_or(0);
    }

    // PIDs Max
    if let Ok(pids_max_str) = fs::read_to_string(dir.join("pids.max")) {
        s.pids_max = pids_max_str.trim().parse().unwrap_or(u64::MAX);
    }

    Some(s)
}

#[tokio::test]
async fn test_cgroup_exhaustive_metrics_benchmark() {
    println!(
        "\n=========================================================================================="
    );
    println!(
        " 🚀 EXHAUSTIVE CGROUP V2 READ BENCHMARK (ALL STATS: CPU, MEMORY, DISK I/O, PIDS, EVENTS)"
    );
    println!(
        "==========================================================================================\n"
    );

    let start_init = Instant::now();
    let reader_opt = CgroupReader::discover();
    let init_duration = start_init.elapsed();

    println!("⏱️ CgroupReader Discovery Time: {:?}", init_duration);

    if reader_opt.is_none() {
        println!("⚠️ Cgroup v2 is not mounted on this host.");
        return;
    }

    let mut container_paths = Vec::new();

    let system_slice = std::path::Path::new("/sys/fs/cgroup/system.slice");
    if let Ok(entries) = fs::read_dir(system_slice) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("docker-") && name.ends_with(".scope") {
                let id = name
                    .trim_start_matches("docker-")
                    .trim_end_matches(".scope")
                    .to_string();
                container_paths.push((id, entry.path()));
            }
        }
    }

    let docker_slice = std::path::Path::new("/sys/fs/cgroup/docker");
    if let Ok(entries) = fs::read_dir(docker_slice) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.len() >= 12 && entry.path().join("cpu.stat").exists() {
                container_paths.push((name, entry.path()));
            }
        }
    }

    if container_paths.is_empty() {
        println!("ℹ️ No active Docker container cgroups found currently running on this system.");
        return;
    }

    println!(
        "Found {} active container cgroup(s). Reading ALL cgroup v2 metrics...\n",
        container_paths.len()
    );

    let t1 = Instant::now();
    let mut sample1 = Vec::new();
    for (id, path) in &container_paths {
        if let Some(s) = read_detailed_cgroup(path) {
            sample1.push((id.clone(), path.clone(), s));
        }
    }
    let sample1_time = t1.elapsed();

    sleep(Duration::from_millis(200));

    let t2 = Instant::now();
    let mut sample2 = Vec::new();
    for (id, path) in &container_paths {
        if let Some(s) = read_detailed_cgroup(path) {
            sample2.push((id.clone(), s));
        }
    }
    let sample2_time = t2.elapsed();

    let avg_single_pass = sample1_time.as_micros() as f64 / sample1.len().max(1) as f64;

    println!(
        "------------------------------------------------------------------------------------------"
    );
    println!("📊 EXHAUSTIVE CGROUP BENCHMARK TIMING SUMMARY");
    println!(
        "------------------------------------------------------------------------------------------"
    );
    println!("Containers Monitored: {}", sample1.len());
    println!(
        "Pass 1 (Exhaustive Read of 9 Files per Container): {:?}",
        sample1_time
    );
    println!(
        "Pass 2 (Exhaustive Read of 9 Files per Container): {:?}",
        sample2_time
    );
    println!(
        "⚡ Average Exhaustive Reading Time Per Container: {:.2} µs ({:.0} ns)",
        avg_single_pass,
        avg_single_pass * 1000.0
    );

    let num_cpus = sysinfo::System::new_all().cpus().len().max(1) as f64;

    println!(
        "\n========================================================================================================="
    );
    println!("📋 DETAILED PER-CONTAINER EXHAUSTIVE METRICS BREAKDOWN");
    println!(
        "========================================================================================================="
    );

    for (id, _path, s1) in &sample1 {
        let s2 = sample2
            .iter()
            .find(|(s2_id, _)| s2_id == id)
            .map(|(_, s)| s);

        let cpu_perc = if let Some(s2) = s2 {
            let delta_cpu_us = s2.usage_usec.saturating_sub(s1.usage_usec) as f64;
            let elapsed_us = 200_000.0;
            ((delta_cpu_us / elapsed_us) * 100.0).min(100.0 * num_cpus)
        } else {
            0.0
        };

        let short_id = if id.len() > 12 { &id[..12] } else { id };

        let total_ram_mb = s1.memory_current as f64 / 1_048_576.0;
        let anon_ram_mb = s1.anon as f64 / 1_048_576.0;
        let file_cache_mb = s1.file as f64 / 1_048_576.0;
        let kernel_slab_mb = s1.slab as f64 / 1_048_576.0;
        let actual_rss_mb =
            (s1.memory_current.saturating_sub(s1.inactive_file)) as f64 / 1_048_576.0;

        let disk_read_mb = s1.io_read_bytes as f64 / 1_048_576.0;
        let disk_write_mb = s1.io_write_bytes as f64 / 1_048_576.0;

        println!("🔹 CONTAINER ID: {}", short_id);
        println!(
            "   ├─ 💻 CPU Usage        : {:.2}% (User: {:.2}s, System: {:.2}s, Throttled: {} times / {:.2}ms)",
            cpu_perc,
            s1.user_usec as f64 / 1_000_000.0,
            s1.system_usec as f64 / 1_000_000.0,
            s1.nr_throttled,
            s1.throttled_usec as f64 / 1000.0
        );
        println!(
            "   ├─ 🧠 Memory Footprint : Actual RSS: {:.2} MB | Total: {:.2} MB (Anon: {:.2} MB, Cache: {:.2} MB, Slab: {:.2} MB)",
            actual_rss_mb, total_ram_mb, anon_ram_mb, file_cache_mb, kernel_slab_mb
        );
        println!(
            "   ├─ 📄 Page Faults      : Total Faults: {} | Major Faults (Disk Reads): {}",
            s1.pgfault, s1.pgmajfault
        );
        println!(
            "   ├─ 💾 Block I/O        : Read: {:.2} MB ({} ops) | Write: {:.2} MB ({} ops)",
            disk_read_mb, s1.io_read_ops, disk_write_mb, s1.io_write_ops
        );
        println!(
            "   ├─ ⚠️ OOM Events       : OOM Triggered: {} times | OOM Kills: {} processes",
            s1.oom_events, s1.oom_kill_events
        );
        println!(
            "   └─ 🧵 Tasks & Processes: Running PIDs: {} (Limit: {})",
            s1.pids_current,
            if s1.pids_max < u64::MAX / 2 {
                s1.pids_max.to_string()
            } else {
                "Unlimited".to_string()
            }
        );
        println!(
            "---------------------------------------------------------------------------------------------------------"
        );
    }

    println!(
        "==========================================================================================\n"
    );
}

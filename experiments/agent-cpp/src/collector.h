#pragma once
#include <atomic>
#include <cstdint>
#include <mutex>
#include <string>
#include <thread>

// One live host sample, mirroring agent/src/telemetry/collector/system's
// ServerMetric. Host-static facts are collected once; vitals on every tick.
struct HostSample {
	int64_t timestamp = 0;
	double cpu = 0.0;
	std::string cpu_model = "unknown";
	int cpu_cores = 0;
	int cpu_physical_cores = 0;
	double cpu_speed = 0.0;
	std::string os = "unknown";
	std::string distro = "unknown";
	std::string kernel = "unknown";
	std::string arch = "unknown";
	double mem_used_percent = 0.0;
	double mem_used_gb = 0.0;
	double mem_total_gb = 0.0;
	uint64_t uptime = 0;
	double disk_used_percent = 0.0;
	double disk_total_gb = 0.0;
	double net_in_mb = 0.0;
	double net_out_mb = 0.0;
};

// Background sampler that refreshes the cached latest sample every METRICS_TICK
// (5 s), exactly like the Rust agent's loop: requests read the cache and never
// trigger host scanning themselves.
class HostCollector {
public:
	HostCollector() = default;
	~HostCollector();
	HostCollector(const HostCollector&) = delete;
	HostCollector& operator=(const HostCollector&) = delete;

	void start();
	void stop();
	HostSample latest(); // blocks until the first sample exists

private:
	void loop();

	std::thread worker_;
	std::atomic<bool> running_{false};
	std::mutex mutex_;
	HostSample cached_;
	bool has_sample_ = false;
	std::atomic<bool> first_ready_{false};

	// /proc deltas
	unsigned long long prev_cpu_total_ = 0;
	unsigned long long prev_cpu_idle_ = 0;
};

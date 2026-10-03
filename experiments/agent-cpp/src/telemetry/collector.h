#pragma once
#include <asio/any_io_executor.hpp>
#include <asio/awaitable.hpp>
#include <asio/steady_timer.hpp>
#include <cstdint>
#include <memory>
#include <string>

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

// Sampling loop coroutine on the agent's io thread, refreshing the cached
// sample every 5 s like the Rust agent's loop. /proc reads are syscalls only
// (sub-millisecond), so running them on the io thread costs nothing and the
// agent holds no dedicated sampler thread.
class HostCollector {
public:
	explicit HostCollector(asio::any_io_executor executor);
	~HostCollector();
	HostCollector(const HostCollector&) = delete;
	HostCollector& operator=(const HostCollector&) = delete;

	void start();
	void stop();
	// Cached sample; on the very first call waits for the loop's first tick.
	asio::awaitable<HostSample> latest();

private:
	asio::awaitable<void> loop();
	void sample();

	asio::any_io_executor ex_;
	asio::steady_timer tick_;
	std::shared_ptr<asio::steady_timer> ready_;
	HostSample cached_;
	bool running_ = false;
	bool has_sample_ = false;

	// /proc deltas
	unsigned long long prev_cpu_total_ = 0;
	unsigned long long prev_cpu_idle_ = 0;
};

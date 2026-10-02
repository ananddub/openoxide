#include "collector.h"

#include <sys/statvfs.h>
#include <sys/utsname.h>
#include <unistd.h>

#include <cctype>
#include <chrono>
#include <cstdio>
#include <fstream>
#include <set>
#include <sstream>
#include <vector>

#include "docker_stats.h"

namespace {

constexpr auto kTick = std::chrono::seconds(5);

std::string read_first_line(const char* path) {
	std::ifstream file(path);
	std::string line;
	std::getline(file, line);
	return line;
}

// Aggregate CPU jiffies: total and idle (idle + iowait).
bool read_cpu_ticks(unsigned long long& total, unsigned long long& idle) {
	std::ifstream file("/proc/stat");
	std::string line;
	if (!std::getline(file, line) || !line.starts_with("cpu ")) {
		return false;
	}
	std::istringstream stream(line.substr(4));
	unsigned long long value = 0;
	size_t index = 0;
	total = 0;
	idle = 0;
	while (stream >> value) {
		total += value;
		if (index == 4 || index == 5) { // idle, iowait
			idle += value;
		}
		++index;
	}
	return index > 0;
}

struct Meminfo {
	unsigned long long total_kb = 0;
	unsigned long long available_kb = 0;
};

Meminfo read_meminfo() {
	Meminfo info;
	std::ifstream file("/proc/meminfo");
	std::string key;
	unsigned long long value = 0;
	std::string unit;
	std::string line;
	while (std::getline(file, line)) {
		std::istringstream stream(line);
		stream >> key >> value >> unit;
		if (key == "MemTotal:") {
			info.total_kb = value;
		} else if (key == "MemAvailable:") {
			info.available_kb = value;
		}
	}
	return info;
}

// Net counters summed over interfaces (sysinfo includes loopback, so we do too).
void read_net_counters(unsigned long long& rx, unsigned long long& tx) {
	rx = 0;
	tx = 0;
	std::ifstream file("/proc/net/dev");
	std::string line;
	while (std::getline(file, line)) {
		const auto colon = line.find(':');
		if (colon == std::string::npos) {
			continue;
		}
		std::istringstream stream(line.substr(colon + 1));
		unsigned long long fields[16] = {0};
		size_t count = 0;
		while (count < 16 && (stream >> fields[count])) {
			++count;
		}
		if (count >= 9) {
			rx += fields[0];
			tx += fields[8];
		}
	}
}

void read_cpu_identity(std::string& model, int& logical, int& physical, double& speed) {
	std::ifstream file("/proc/cpuinfo");
	std::string line;
	std::set<std::string> core_ids;
	size_t processors = 0;
	int current_physical = -1;
	std::string current_core;
	logical = 0;
	physical = 0;
	speed = 0.0;
	while (std::getline(file, line)) {
		const auto sep = line.find(':');
		if (sep == std::string::npos) {
			continue;
		}
		std::string key = line.substr(0, sep);
		std::string value = line.substr(sep + 1);
		while (!key.empty() && std::isspace(static_cast<unsigned char>(key.back()))) {
			key.pop_back();
		}
		while (!value.empty() && value.front() == ' ') {
			value.erase(value.begin());
		}
		if (key == "processor") {
			++processors;
		} else if (key == "model name" && model == "unknown") {
			model = value;
		} else if (key == "physical id") {
			current_physical = std::atoi(value.c_str());
		} else if (key == "core id") {
			current_core = value;
			if (current_physical >= 0) {
				core_ids.insert(std::to_string(current_physical) + ":" + current_core);
			}
		} else if (key == "cpu MHz" && speed == 0.0) {
			speed = std::atof(value.c_str());
		}
	}
	logical = static_cast<int>(processors);
	physical = core_ids.empty() ? logical : static_cast<int>(core_ids.size());
}

// Host root filesystem usage; prefers /host (container bind-mount) then /,
// mirroring the Rust collector's mount preference.
bool read_disk(unsigned long long& used, unsigned long long& total) {
	for (const char* mount : {"/host", "/"}) {
		struct statvfs vfs {};
		if (statvfs(mount, &vfs) == 0 && vfs.f_blocks > 0) {
			total = vfs.f_blocks * vfs.f_frsize;
			used = (vfs.f_blocks - vfs.f_bfree) * vfs.f_frsize;
			return true;
		}
	}
	return false;
}

void read_host_identity(HostSample& sample) {
	if (sample.cpu_model != "unknown") {
		return; // static facts collected already
	}
	std::string model = "unknown";
	int logical = 0;
	int physical = 0;
	double speed = 0.0;
	read_cpu_identity(model, logical, physical, speed);
	sample.cpu_model = model;
	sample.cpu_cores = logical;
	sample.cpu_physical_cores = physical;
	sample.cpu_speed = speed;

	struct utsname uts {};
	if (uname(&uts) == 0) {
		sample.os = uts.sysname;
		sample.kernel = uts.release;
		sample.arch = uts.machine;
	}

	std::ifstream release("/etc/os-release");
	std::string line;
	while (std::getline(release, line)) {
		if (line.starts_with("PRETTY_NAME=")) {
			std::string value = line.substr(12);
			if (!value.empty() && value.front() == '"') {
				value.erase(value.begin());
			}
			if (!value.empty() && value.back() == '"') {
				value.pop_back();
			}
			sample.distro = value;
			break;
		}
	}
	if (sample.distro.empty()) {
		sample.distro = sample.os;
	}
}

} // namespace

HostCollector::~HostCollector() {
	stop();
}

void HostCollector::start() {
	running_ = true;
	worker_ = std::thread([this] { loop(); });
}

void HostCollector::stop() {
	running_ = false;
	if (worker_.joinable()) {
		worker_.join();
	}
}

void HostCollector::loop() {
	// Prime the CPU delta once so the first published sample has a real usage
	// figure (sysinfo does the same with its minimum update interval).
	read_cpu_ticks(prev_cpu_total_, prev_cpu_idle_);
	std::this_thread::sleep_for(std::chrono::milliseconds(200));
	while (running_) {
		{
			std::lock_guard<std::mutex> guard(mutex_);
			unsigned long long total = 0;
			unsigned long long idle = 0;
			if (read_cpu_ticks(total, idle)) {
				const unsigned long long total_delta = total > prev_cpu_total_ ? total - prev_cpu_total_ : 0;
				const unsigned long long idle_delta = idle > prev_cpu_idle_ ? idle - prev_cpu_idle_ : 0;
				cached_.cpu = total_delta > 0 ? 100.0 * static_cast<double>(total_delta - idle_delta) / static_cast<double>(total_delta) : cached_.cpu;
				prev_cpu_total_ = total;
				prev_cpu_idle_ = idle;
			}

			const auto mem = read_meminfo();
			const double total_bytes = static_cast<double>(mem.total_kb) * 1024.0;
			const double used_bytes = static_cast<double>(mem.total_kb > mem.available_kb ? mem.total_kb - mem.available_kb : 0) * 1024.0;
			cached_.mem_total_gb = bytes_to_gb(total_bytes);
			cached_.mem_used_gb = bytes_to_gb(used_bytes);
			cached_.mem_used_percent = total_bytes > 0.0 ? used_bytes / total_bytes * 100.0 : 0.0;

			unsigned long long rx = 0;
			unsigned long long tx = 0;
			read_net_counters(rx, tx);
			cached_.net_in_mb = bytes_to_mb(static_cast<double>(rx));
			cached_.net_out_mb = bytes_to_mb(static_cast<double>(tx));

			unsigned long long disk_used = 0;
			unsigned long long disk_total = 0;
			if (read_disk(disk_used, disk_total)) {
				cached_.disk_total_gb = bytes_to_gb(static_cast<double>(disk_total));
				cached_.disk_used_percent = disk_total > 0 ? static_cast<double>(disk_used) / static_cast<double>(disk_total) * 100.0 : 0.0;
			}

			const std::string uptime_line = read_first_line("/proc/uptime");
			cached_.uptime = static_cast<uint64_t>(std::strtoull(uptime_line.c_str(), nullptr, 10));
			cached_.timestamp = static_cast<int64_t>(std::time(nullptr));

			read_host_identity(cached_);
			has_sample_ = true;
		}
		first_ready_ = true;

		// Tick in small slices so stop() joins promptly.
		for (int i = 0; i < 50 && running_; ++i) {
			std::this_thread::sleep_for(kTick / 50);
		}
	}
}

HostSample HostCollector::latest() {
	while (!first_ready_ && running_) {
		std::this_thread::sleep_for(std::chrono::milliseconds(50));
	}
	std::lock_guard<std::mutex> guard(mutex_);
	return cached_;
}

#include "docker/docker_stats.h"

#include <fmt/format.h>
#include <algorithm>

namespace {

unsigned long long json_u64(const nlohmann::json& obj, const char* key) {
	if (!obj.is_object()) {
		return 0;
	}
	const auto it = obj.find(key);
	if (it == obj.end() || !it->is_number()) {
		return 0;
	}
	return it->get<unsigned long long>();
}

std::string json_str(const nlohmann::json& obj, const char* key) {
	if (!obj.is_object()) {
		return "";
	}
	const auto it = obj.find(key);
	if (it == obj.end() || !it->is_string()) {
		return "";
	}
	return it->get<std::string>();
}

} // namespace

double DockerStatsDoc::cpu_percent() const {
	const double cpu_delta = static_cast<double>(cpu_total > precpu_total ? cpu_total - precpu_total : 0);
	const double system_delta = static_cast<double>(cpu_system > precpu_system ? cpu_system - precpu_system : 0);
	if (system_delta <= 0.0 || cpu_delta <= 0.0) {
		return 0.0;
	}
	double cores = 1.0;
	if (cpu_online > 0) {
		cores = static_cast<double>(cpu_online);
	} else if (cpu_percpu_count > 0) {
		cores = static_cast<double>(cpu_percpu_count);
	}
	return (cpu_delta / system_delta) * cores * 100.0;
}

unsigned long long DockerStatsDoc::mem_used_bytes() const {
	return mem_usage > mem_inactive_file ? mem_usage - mem_inactive_file : 0;
}

double DockerStatsDoc::mem_used_percent() const {
	if (mem_limit == 0) {
		return 0.0;
	}
	return static_cast<double>(mem_used_bytes()) / static_cast<double>(mem_limit) * 100.0;
}

std::string DockerStatsDoc::display_name() const {
	std::string name = this->name;
	if (!name.empty() && name.front() == '/') {
		name.erase(name.begin());
	}
	return name;
}

DockerStatsDoc parse_stats_doc(const nlohmann::json& doc) {
	DockerStatsDoc out;
	out.id = json_str(doc, "id");
	out.name = json_str(doc, "name");
	if (const auto pids = doc.find("pids_stats"); pids != doc.end() && pids->is_object()) {
		out.pids_current = json_u64(*pids, "current");
	}

	const auto read_cpu = [](const nlohmann::json& node, unsigned long long& total, unsigned long long& system,
	                         unsigned long long& online, size_t& percpu_count) {
		if (!node.is_object()) {
			return;
		}
		system = json_u64(node, "system_cpu_usage");
		online = json_u64(node, "online_cpus");
		if (const auto usage = node.find("cpu_usage"); usage != node.end() && usage->is_object()) {
			total = json_u64(*usage, "total_usage");
			if (const auto percpu = usage->find("percpu_usage"); percpu != usage->end() && percpu->is_array()) {
				percpu_count = percpu->size();
			}
		}
	};
	unsigned long long precpu_online = 0;
	size_t precpu_percpu = 0;
	read_cpu(doc.value("cpu_stats", nlohmann::json::object()), out.cpu_total, out.cpu_system, out.cpu_online, out.cpu_percpu_count);
	read_cpu(doc.value("precpu_stats", nlohmann::json::object()), out.precpu_total, out.precpu_system, precpu_online, precpu_percpu);

	if (const auto mem = doc.find("memory_stats"); mem != doc.end() && mem->is_object()) {
		out.mem_usage = json_u64(*mem, "usage");
		out.mem_limit = json_u64(*mem, "limit");
		if (const auto stats = mem->find("stats"); stats != mem->end() && stats->is_object()) {
			for (const char* key : {"inactive_file", "total_inactive_file", "cache"}) {
				if (stats->contains(key)) {
					out.mem_inactive_file = json_u64(*stats, key);
					break;
				}
			}
		}
	}

	if (const auto nets = doc.find("networks"); nets != doc.end() && nets->is_object()) {
		for (const auto& [_ifname, node] : nets->items()) {
			out.net_rx += json_u64(node, "rx_bytes");
			out.net_tx += json_u64(node, "tx_bytes");
		}
	}

	if (const auto blkio = doc.find("blkio_stats"); blkio != doc.end() && blkio->is_object()) {
		if (const auto entries = blkio->find("io_service_bytes_recursive");
		    entries != blkio->end() && entries->is_array()) {
			for (const auto& entry : *entries) {
				std::string op = json_str(entry, "op");
				std::transform(op.begin(), op.end(), op.begin(), [](unsigned char c) { return std::tolower(c); });
				const auto value = json_u64(entry, "value");
				if (op == "read") {
					out.blk_read += value;
				} else if (op == "write") {
					out.blk_write += value;
				}
			}
		}
	}
	return out;
}

std::string human_bytes(unsigned long long bytes, bool binary) {
	const double divisor = binary ? 1024.0 : 1000.0;
	static const char* kBinary[] = {"B", "KiB", "MiB", "GiB", "TiB", "PiB"};
	static const char* kSi[] = {"B", "kB", "MB", "GB", "TB", "PB"};
	const char** units = binary ? kBinary : kSi;
	const size_t count = 6;
	double value = static_cast<double>(bytes);
	size_t unit = 0;
	while (value >= divisor && unit + 1 < count) {
		value /= divisor;
		++unit;
	}
	if (unit == 0) {
		return fmt::format("{}B", static_cast<unsigned long long>(value));
	}
	return fmt::format("{:.2f}{}", value, units[unit]);
}

double bytes_to_mb(double bytes) {
	return bytes / (1024.0 * 1024.0);
}

double bytes_to_gb(double bytes) {
	return bytes / (1024.0 * 1024.0 * 1024.0);
}

bool docker_id_matches(const std::string& id, const std::string& doc_id) {
	if (doc_id == id) {
		return true;
	}
	if (doc_id.starts_with(id)) {
		return true;
	}
	const auto doc_short = doc_id.substr(0, 12);
	return id.starts_with(doc_short);
}

bool docker_id_matches_any(const std::vector<std::string>& ids, const std::string& doc_id) {
	for (const std::string& id : ids) {
		if (docker_id_matches(id, doc_id)) {
			return true;
		}
	}
	return false;
}

std::string stats_to_cli_line(const DockerStatsDoc& doc, unsigned long long size_rw) {
	nlohmann::json out;
	std::string name = doc.name;
	if (!name.empty() && name.front() == '/') {
		name.erase(name.begin());
	}
	const std::string short_id = doc.id.substr(0, 12);

	out["CPUPerc"] = fmt::format("{:.2f}%", doc.cpu_percent());
	out["MemPerc"] = fmt::format("{:.2f}%", doc.mem_used_percent());
	out["Container"] = short_id;
	out["ID"] = doc.id;
	out["MemUsage"] = fmt::format("{} / {}", human_bytes(doc.mem_used_bytes(), true), human_bytes(doc.mem_limit, true));
	out["Name"] = name;
	out["NetIO"] = fmt::format("{} / {}", human_bytes(doc.net_rx, false), human_bytes(doc.net_tx, false));
	out["BlockIO"] = fmt::format("{} / {}", human_bytes(doc.blk_read, false), human_bytes(doc.blk_write, false));
	out["PIDs"] = fmt::format("{}", doc.pids_current);
	out["SizeRw"] = human_bytes(size_rw, true);
	return out.dump();
}

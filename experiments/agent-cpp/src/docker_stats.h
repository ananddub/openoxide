#pragma once
#include <nlohmann/json.hpp>
#include <string>
#include <vector>

// Parsed view of one Docker /stats document — the C++ counterpart of
// agent/src/utils/docker/query/types.rs (ContainerStats + RawStats).
struct DockerStatsDoc {
	std::string id;
	std::string name; // as delivered (leading '/' kept); use display_name()
	unsigned long long pids_current = 0;

	unsigned long long cpu_total = 0;
	unsigned long long cpu_system = 0;
	unsigned long long cpu_online = 0;
	size_t cpu_percpu_count = 0;
	unsigned long long precpu_total = 0;
	unsigned long long precpu_system = 0;

	unsigned long long mem_usage = 0;
	unsigned long long mem_limit = 0;
	unsigned long long mem_inactive_file = 0; // inactive/total_inactive/cache fallbacks

	unsigned long long net_rx = 0;
	unsigned long long net_tx = 0;
	unsigned long long blk_read = 0;
	unsigned long long blk_write = 0;

	[[nodiscard]] double cpu_percent() const;
	[[nodiscard]] unsigned long long mem_used_bytes() const;
	[[nodiscard]] double mem_used_percent() const;
	[[nodiscard]] std::string display_name() const;
};

DockerStatsDoc parse_stats_doc(const nlohmann::json& doc);

// Renders one doc as the JSON object emitted by `docker stats
// --format {{json .}}` (mirrors stats.rs to_cli_line).
std::string stats_to_cli_line(const DockerStatsDoc& doc, unsigned long long size_rw);

// Docker's formatter: binary units for memory (MiB/GiB), SI for io (kB/MB).
std::string human_bytes(unsigned long long bytes, bool binary);

double bytes_to_mb(double bytes);
double bytes_to_gb(double bytes);

// Id equality tolerant of full-vs-short (12 char) forms, like stats.rs.
bool docker_id_matches(const std::string& id, const std::string& doc_id);

// True when the doc id matches any requested id (StreamStatsRequest filter).
bool docker_id_matches_any(const std::vector<std::string>& ids, const std::string& doc_id);

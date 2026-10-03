#include "core/config.h"

#include <fmt/format.h>
#include <algorithm>
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <stdexcept>

namespace {

std::string env_str(const char* key, const std::string& fallback = "") {
	const char* raw = std::getenv(key);
	if (!raw) {
		return fallback;
	}
	return raw;
}

std::string trim(const std::string& value) {
	const auto begin = value.find_first_not_of(" \t\r\n");
	if (begin == std::string::npos) {
		return "";
	}
	const auto end = value.find_last_not_of(" \t\r\n");
	return value.substr(begin, end - begin + 1);
}

template <typename T>
T env_num(const char* key, T fallback) {
	const std::string raw = trim(env_str(key));
	if (raw.empty()) {
		return fallback;
	}
	std::istringstream stream(raw);
	T value{};
	if (!(stream >> value)) {
		throw std::runtime_error(std::string(key) + " is set to \"" + raw + "\", which is not a valid value");
	}
	return value;
}

std::optional<std::string> env_opt(const char* key) {
	const std::string raw = trim(env_str(key));
	if (raw.empty()) {
		return std::nullopt;
	}
	return raw;
}

} // namespace

bool Config::validate(std::string& error) const {
	if (server_id == 0 || server_id < -1) {
		error = fmt::format("SERVER_ID must be a positive integer or -1 for the local Agent, got {}", server_id);
		return false;
	}
	if (trim(metrics_token).empty()) {
		error = "METRICS_TOKEN must be set; anonymous metric pushes are disabled";
		return false;
	}
	if (max_upload_bytes == 0 || upload_timeout_seconds == 0) {
		error = "upload size and timeout limits must be positive";
		return false;
	}
	if (upload_roots.empty()) {
		error = "UPLOAD_ROOTS must contain absolute paths";
		return false;
	}
	for (const auto& root : upload_roots) {
		if (root.empty() || root.front() != '/') {
			error = "UPLOAD_ROOTS must contain absolute paths";
			return false;
		}
	}
	if (tls_cert.has_value() != tls_key.has_value()) {
		error = "AGENT_TLS_CERT and AGENT_TLS_KEY must be configured together";
		return false;
	}
	return true;
}

Config Config::from_env() {
	Config config;
	config.server_id = env_num<int64_t>("SERVER_ID", 1);
	config.grpc_port = env_num<uint16_t>("GRPC_PORT", 50051);
	config.metrics_token = env_str("METRICS_TOKEN");
	config.docker_socket = env_str("DOCKER_SOCKET", "/var/run/docker.sock");
	if (trim(config.docker_socket).empty()) {
		config.docker_socket = "/var/run/docker.sock";
	}
	config.max_upload_bytes = env_num<uint64_t>("MAX_UPLOAD_BYTES", 1024ULL * 1024 * 1024);
	config.upload_timeout_seconds = env_num<uint64_t>("UPLOAD_TIMEOUT_SECONDS", 300);

	std::istringstream roots(env_str("UPLOAD_ROOTS", "/tmp,/etc/openoxide,/app/workspace"));
	std::string root;
	while (std::getline(roots, root, ',')) {
		const auto trimmed = trim(root);
		if (!trimmed.empty()) {
			config.upload_roots.push_back(trimmed);
		}
	}

	config.tls_cert = env_opt("AGENT_TLS_CERT");
	config.tls_key = env_opt("AGENT_TLS_KEY");
	config.tls_client_ca = env_opt("AGENT_TLS_CLIENT_CA");

	std::string error;
	if (!config.validate(error)) {
		throw std::runtime_error(error);
	}
	return config;
}

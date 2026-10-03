#pragma once
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

// Mirrors agent/src/core/config.rs: same env vars, same validation rules.
struct Config {
	int64_t server_id = 1;
	uint16_t grpc_port = 50051;
	std::string metrics_token;
	std::string docker_socket = "/var/run/docker.sock";
	uint64_t max_upload_bytes = 1024ULL * 1024 * 1024;
	uint64_t upload_timeout_seconds = 300;
	std::vector<std::string> upload_roots;
	std::optional<std::string> tls_cert;
	std::optional<std::string> tls_key;
	std::optional<std::string> tls_client_ca;

	// Throws std::runtime_error on invalid configuration, like Config::from_env.
	static Config from_env();
	bool validate(std::string& error) const;
};

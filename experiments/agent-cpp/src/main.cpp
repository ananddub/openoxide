#include <grpcpp/grpcpp.h>

#include <atomic>
#include <chrono>
#include <csignal>
#include <fstream>
#include <iostream>
#include <sstream>
#include <thread>

#include "agent_service.h"
#include "collector.h"
#include "config.h"
#include "docker_client.h"
#include "file_transfer.h"
#include "hubs.h"
#include "shell_service.h"

namespace {

std::atomic<bool> g_shutdown{false};

void handle_signal(int) {
	g_shutdown = true;
}

// Mirrors read_with_wait in app/grpc.rs: the panel provisions the shared
// mTLS material at its own startup, which may lag the agent's boot, so wait
// ~60s for each file instead of failing fast.
std::string read_with_wait(const std::string& path) {
	for (int attempt = 0; attempt < 60; ++attempt) {
		std::ifstream file(path);
		if (file) {
			std::ostringstream buffer;
			buffer << file.rdbuf();
			if (!buffer.str().empty()) {
				return buffer.str();
			}
		}
		if (attempt == 0) {
			std::cerr << "waiting for mTLS material to be provisioned: " << path << "\n";
		}
		std::this_thread::sleep_for(std::chrono::seconds(1));
	}
	std::ifstream file(path);
	std::ostringstream buffer;
	buffer << file.rdbuf();
	return buffer.str();
}

} // namespace

int main() {
	std::signal(SIGTERM, handle_signal);
	std::signal(SIGINT, handle_signal);

	Config config;
	try {
		config = Config::from_env();
	} catch (const std::exception& error) {
		std::cerr << "invalid configuration: " << error.what() << "\n";
		return 1;
	}
	std::cerr << "starting openoxide-agent-cpp server_id=" << config.server_id
	          << " grpc_port=" << config.grpc_port << "\n";

	DockerClient docker(config.docker_socket);
	try {
		docker.get("/_ping");
	} catch (const std::exception& error) {
		std::cerr << "docker daemon unreachable: " << error.what() << "\n";
	}

	HostCollector collector;
	collector.start();
	StatsHub stats(docker);
	EventsHub events(docker);
	FileTransferService files(config);
	ShellService shells(config, docker);
	AgentServiceImpl service(config, docker, collector, stats, events, files, shells);

	grpc::ServerBuilder builder;
	std::shared_ptr<grpc::ServerCredentials> credentials = grpc::InsecureServerCredentials();
	if (config.tls_cert && config.tls_key) {
		const std::string cert = read_with_wait(*config.tls_cert);
		const std::string key = read_with_wait(*config.tls_key);
		if (cert.empty() || key.empty()) {
			std::cerr << "gRPC TLS certificate/key never appeared\n";
			return 1;
		}
		grpc::SslServerCredentialsOptions options(
			config.tls_client_ca ? GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY
			                     : GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_BUT_DONT_VERIFY);
		if (config.tls_client_ca) {
			const std::string ca = read_with_wait(*config.tls_client_ca);
			if (ca.empty()) {
				std::cerr << "gRPC client CA never appeared\n";
				return 1;
			}
			options.pem_root_certs = ca;
		}
		grpc::SslServerCredentialsOptions::PemKeyCertPair pair;
		pair.private_key = key;
		pair.cert_chain = cert;
		options.pem_key_cert_pairs.push_back(pair);
		credentials = grpc::SslServerCredentials(options);
		std::cerr << "gRPC TLS enabled mtls=" << (config.tls_client_ca ? "true" : "false") << "\n";
	} else {
		std::cerr << "gRPC TLS disabled; configure AGENT_TLS_CERT and AGENT_TLS_KEY for encrypted transport\n";
	}
	builder.AddListeningPort("0.0.0.0:" + std::to_string(config.grpc_port), credentials);
	builder.RegisterService(&service);
	builder.SetSyncServerOption(grpc::ServerBuilder::NUM_CQS, 2);

	std::unique_ptr<grpc::Server> server;
	try {
		server = builder.BuildAndStart();
	} catch (...) {
		std::cerr << "could not start gRPC server on port " << config.grpc_port << "\n";
		return 1;
	}
	if (!server) {
		std::cerr << "could not bind port " << config.grpc_port << "\n";
		return 1;
	}
	std::cerr << "gRPC query server listening on 0.0.0.0:" << config.grpc_port << "\n";

	std::thread shutdown_watchdog([&] {
		while (!g_shutdown) {
			std::this_thread::sleep_for(std::chrono::milliseconds(100));
		}
		std::cerr << "shutdown signal received, stopping tasks\n";
		collector.stop();
		server->Shutdown();
	});

	server->Wait();
	shutdown_watchdog.join();
	return 0;
}

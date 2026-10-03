#include <grpcpp/grpcpp.h>

#include <asio/co_spawn.hpp>
#include <asio/detached.hpp>
#include <asio/executor_work_guard.hpp>
#include <asio/io_context.hpp>
#include <asio/signal_set.hpp>
#include <asio/use_awaitable.hpp>

#include <chrono>
#include <cstdlib>
#include <fmt/core.h>
#include <fmt/format.h>
#include <fstream>
#include <memory>
#include <signal.h>
#include <thread>

#if defined(__GLIBC__)
#include <malloc.h>
#endif

#include "services/agent_service.h"
#include "telemetry/collector.h"
#include "core/config.h"
#include "docker/docker_client.h"
#include "services/file_transfer.h"
#include "docker/hubs.h"
#include "services/shell_service.h"
#include "core/tiny_event_engine.h"

namespace {

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
			fmt::print(stderr, "waiting for mTLS material to be provisioned: {}\n", path);
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
	// gRPC's built-in EventEngine spawns a hardware_concurrency-sized thread
	// pool (32 threads here) plus an EE poller per connection; the default
	// engine would otherwise be created on first grpc init and dominate RSS.
	InstallTinyEventEngine();
	// Belt-and-braces: classic iomgr keeps doing all socket work, so the tiny
	// engine above only runs closures and timers. Must precede any grpc_init.
	setenv("GRPC_EXPERIMENTS", "-event_engine_client", 0);
#if defined(__GLIBC__)
	// One malloc arena: default spawns an arena per core, each holding pages.
	mallopt(M_ARENA_MAX, 1);
#endif

	Config config;
	try {
		config = Config::from_env();
	} catch (const std::exception& error) {
		fmt::print(stderr, "invalid configuration: {}\n", error.what());
		return 1;
	}
	fmt::print(stderr, "starting openoxide-agent-cpp server_id={} grpc_port={}\n", config.server_id,
	       config.grpc_port);

	asio::io_context io;
	// The work guard keeps io.run() from returning while gRPC EventEngine
	// threads may still post completions onto it; released after Wait().
	auto work = asio::make_work_guard(io);
	const asio::any_io_executor ex = io.get_executor();

	DockerClient docker(ex, config.docker_socket);
	HostCollector collector(ex);
	StatsHub stats(ex, docker);
	EventsHub events(ex, docker);
	FileTransferService files(ex, config);
	ShellService shells(ex, config, docker);
	AgentServiceImpl service(ex, config, docker, collector, stats, events, files, shells);

	grpc::ServerBuilder builder;
	std::shared_ptr<grpc::ServerCredentials> credentials = grpc::InsecureServerCredentials();
	if (config.tls_cert && config.tls_key) {
		const std::string cert = read_with_wait(*config.tls_cert);
		const std::string key = read_with_wait(*config.tls_key);
		if (cert.empty() || key.empty()) {
			fmt::print(stderr, "gRPC TLS certificate/key never appeared\n");
			return 1;
		}
		grpc::SslServerCredentialsOptions options(
			config.tls_client_ca ? GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY
			                     : GRPC_SSL_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_BUT_DONT_VERIFY);
		if (config.tls_client_ca) {
			const std::string ca = read_with_wait(*config.tls_client_ca);
			if (ca.empty()) {
				fmt::print(stderr, "gRPC client CA never appeared\n");
				return 1;
			}
			options.pem_root_certs = ca;
		}
		grpc::SslServerCredentialsOptions::PemKeyCertPair pair;
		pair.private_key = key;
		pair.cert_chain = cert;
		options.pem_key_cert_pairs.push_back(pair);
		credentials = grpc::SslServerCredentials(options);
		fmt::print(stderr, "gRPC TLS enabled mtls={}\n", config.tls_client_ca ? "true" : "false");
	} else {
		fmt::print(stderr, "gRPC TLS disabled; configure AGENT_TLS_CERT and AGENT_TLS_KEY for encrypted transport\n");
	}
	builder.AddListeningPort(fmt::format("0.0.0.0:{}", config.grpc_port), credentials);
	builder.RegisterService(&service);

	std::unique_ptr<grpc::Server> server;
	try {
		server = builder.BuildAndStart();
	} catch (...) {
		fmt::print(stderr, "could not start gRPC server on port {}\n", config.grpc_port);
		return 1;
	}
	if (!server) {
		fmt::print(stderr, "could not bind port {}\n", config.grpc_port);
		return 1;
	}
	fmt::print(stderr, "gRPC query server listening on 0.0.0.0:{}\n", config.grpc_port);

	// Callback-API server: gRPC hands rpcs to handlers on EventEngine
	// closures and every handler continues as a coroutine on this single io
	// thread — RSS stays in the low megabytes regardless of stream count.
	collector.start();
	grpc::Server* server_ptr = server.get();
	asio::co_spawn(
	    ex,
	    [&docker]() -> asio::awaitable<void> {
		    try {
			    co_await docker.get("/_ping");
		    } catch (const std::exception& error) {
			    fmt::print(stderr, "docker daemon unreachable: {}\n", error.what());
		    }
	    },
	    asio::detached);
	asio::co_spawn(
	    ex,
	    [server_ptr, &collector, ex]() -> asio::awaitable<void> {
		    auto signals = std::make_shared<asio::signal_set>(ex, SIGTERM, SIGINT);
		    int signo = 0;
		    try {
			    signo = co_await signals->async_wait(asio::use_awaitable);
		    } catch (const asio::system_error&) {
			    co_return;
		    }
		    fmt::print(stderr, "shutdown signal {} received, stopping tasks\n", signo);
		    collector.stop();
		    // The deadline'd Shutdown tells in-flight rpcs (watch/stats/shell
		    // loops) to return after that long, so a stuck stream cannot make
		    // us miss the docker stop deadline. Shutdown blocks until the rpcs
		    // drain — and those rpc coroutines run on this io thread — so it
		    // must never be called here; give it a thread of its own.
		    std::thread([server_ptr] {
			    server_ptr->Shutdown(std::chrono::system_clock::now() + std::chrono::seconds(3));
		    }).detach();
	    },
	    asio::detached);

	std::thread io_thread([&io] { io.run(); });

	server->Wait();
	// Every rpc is done; release the guard, abort any hub timer still armed,
	// and let the io thread unwind the remaining coroutine frames.
	work.reset();
	io.stop();
	io_thread.join();
	return 0;
}

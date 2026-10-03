#pragma once
#include <asio/any_io_executor.hpp>
#include <grpcpp/grpcpp.h>

#include "agent.grpc.pb.h"
#include "core/async_util.h"
#include "core/config.h"
#include "docker/docker_client.h"

// Per-shell session state; defined in shell_service.cpp.
struct ShellState;

// Mirrors agent/src/services/monitoring/shell.rs: interactive shells over
// hijacked Docker exec streams (container / host-escape helper) or a local
// PTY, plus one-shot ExecTask runs. Everything runs as coroutines on the
// agent's io thread — a shell holds no thread, only a pty fd, two small
// coroutine frames and one mailbox.
class ShellService {
public:
	ShellService(asio::any_io_executor executor, const Config& config, DockerClient& docker)
	    : ex_(std::move(executor)), config_(config), docker_(docker) {}

	grpc::ServerBidiReactor<agent::ShellMessage, agent::ShellMessage>* open(
	    grpc::CallbackServerContext* context);

	grpc::ServerWriteReactor<agent::ExecTaskOutput>* start_exec_task(
	    grpc::CallbackServerContext* context, const agent::ExecTaskRequest* request);

private:
	using ShellReactor = CoroutineBidiReactor<agent::ShellMessage, agent::ShellMessage>;
	using TaskReactor = CoroutineWriteReactor<agent::ExecTaskOutput>;

	asio::awaitable<grpc::Status> run_shell(ShellReactor* reactor, std::shared_ptr<ShellState> state);
	asio::awaitable<grpc::Status> run_exec_task(agent::ExecTaskRequest request, TaskReactor* reactor);
	asio::awaitable<void> shell_output_loop(ShellReactor* reactor, std::shared_ptr<ShellState> state);
	asio::awaitable<void> shell_input_loop(ShellReactor* reactor, std::shared_ptr<ShellState> state);
	// open_container(): exact compose-service label match wins, then name
	// substring hits; falls back to the raw name when Docker is unreachable.
	asio::awaitable<std::string> resolve_container(const std::string& requested);

	asio::any_io_executor ex_;
	const Config& config_;
	DockerClient& docker_;
};

// Releases this cgroup's reclaimable page cache after a deploy task, exactly
// like the Rust agent (memory.reclaim, best effort).
void reclaim_own_page_cache();

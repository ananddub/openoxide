#pragma once
#include <grpcpp/server_context.h>
#include <grpcpp/support/sync_stream.h>

#include "agent.pb.h"
#include "config.h"
#include "docker_client.h"

// Mirrors agent/src/services/monitoring/shell.rs: interactive shells over
// hijacked Docker exec streams (container / host-escape helper) or a local
// PTY, plus one-shot ExecTask runs.
class ShellService {
public:
	ShellService(const Config& config, DockerClient& docker) : config_(config), docker_(docker) {}

	grpc::Status Open(grpc::ServerContext* context,
	                 grpc::ServerReaderWriter<agent::ShellMessage, agent::ShellMessage>* stream);

	grpc::Status ExecTask(grpc::ServerContext* context,
	                      const agent::ExecTaskRequest& request,
	                      grpc::ServerWriter<agent::ExecTaskOutput>* writer);

private:
	const Config& config_;
	DockerClient& docker_;
};

// Releases this cgroup's reclaimable page cache after a deploy task, exactly
// like the Rust agent (memory.reclaim, best effort).
void reclaim_own_page_cache();

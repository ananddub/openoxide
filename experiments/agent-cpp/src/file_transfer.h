#pragma once
#include <grpcpp/server_context.h>
#include <grpcpp/support/sync_stream.h>
#include <string>

#include "agent.pb.h"
#include "config.h"

// Mirrors agent/src/services/file_transfer.rs: header-validated streaming
// upload, SHA-256 checked, atomically installed under UPLOAD_ROOTS.
// Throws std::runtime_error for the failures the Rust service maps to Status.
class FileTransferService {
public:
	explicit FileTransferService(const Config& config) : config_(config) {}

	agent::AgentInfoResponse agent_info() const;

	agent::FileUploadResult upload(grpc::ServerContext* context,
	                               grpc::ServerReader<agent::FileUploadMessage>* reader);

private:
	const Config& config_;
};

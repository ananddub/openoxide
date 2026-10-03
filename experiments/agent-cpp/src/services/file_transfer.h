#pragma once
#include <asio/any_io_executor.hpp>
#include <grpcpp/grpcpp.h>
#include <string>

#include "agent.grpc.pb.h"
#include "core/async_util.h"
#include "core/config.h"

// Mirrors agent/src/services/file_transfer.rs: header-validated streaming
// upload, SHA-256 checked, atomically installed under UPLOAD_ROOTS. The
// inbound stream arrives through a CoroutineReadReactor mailbox, so the
// writing coroutine never blocks a thread between chunks.
class FileTransferService {
public:
	FileTransferService(asio::any_io_executor executor, const Config& config)
	    : ex_(std::move(executor)), config_(config) {}

	agent::AgentInfoResponse agent_info() const;

	grpc::ServerReadReactor<agent::FileUploadMessage>* start(grpc::CallbackServerContext* context,
	                                                         agent::FileUploadResult* response);

private:
	using UploadReactor = CoroutineReadReactor<agent::FileUploadMessage>;

	asio::awaitable<grpc::Status> upload_call(std::shared_ptr<UploadReactor::Box> box,
	                                          std::shared_ptr<CallPilot> pilot,
	                                          agent::FileUploadResult* response);

	asio::any_io_executor ex_;
	const Config& config_;
};

#pragma once
#include <grpcpp/grpcpp.h>

#include "agent.grpc.pb.h"
#include "collector.h"
#include "config.h"
#include "docker_client.h"
#include "file_transfer.h"
#include "hubs.h"
#include "shell_service.h"

// Mirrors agent/src/api/agent.rs: x-metrics-token auth on every call
// (core/middleware.rs), check_server_id, and delegation to the same
// services the Rust agent wires up.
class AgentServiceImpl final : public agent::AgentService::Service {
public:
	AgentServiceImpl(const Config& config, DockerClient& docker, HostCollector& collector,
	                 StatsHub& stats, EventsHub& events, FileTransferService& files,
	                 ShellService& shells)
	    : config_(config), docker_(docker), collector_(collector), stats_(stats), events_(events),
	      files_(files), shells_(shells) {}

	grpc::Status GetAgentInfo(grpc::ServerContext* context, const agent::AgentInfoRequest* request,
	                          agent::AgentInfoResponse* response) override;
	grpc::Status GetServerMetrics(grpc::ServerContext* context, const agent::GetMetricsRequest* request,
	                              agent::ServerMetricsResponse* response) override;
	grpc::Status GetContainerMetrics(grpc::ServerContext* context,
	                                 const agent::GetContainerMetricsRequest* request,
	                                 agent::ContainerMetricsResponse* response) override;
	grpc::Status GetContainerStates(grpc::ServerContext* context, const agent::GetMetricsRequest* request,
	                                agent::ContainerStatesResponse* response) override;
	grpc::Status WatchContainerStates(grpc::ServerContext* context, const agent::WatchRequest* request,
	                                  grpc::ServerWriter<agent::ContainerStatesResponse>* writer) override;
	grpc::Status Shell(grpc::ServerContext* context,
	                   grpc::ServerReaderWriter<agent::ShellMessage, agent::ShellMessage>* stream) override;
	grpc::Status ExecTask(grpc::ServerContext* context, const agent::ExecTaskRequest* request,
	                      grpc::ServerWriter<agent::ExecTaskOutput>* writer) override;
	grpc::Status StreamContainerStats(grpc::ServerContext* context, const agent::StreamStatsRequest* request,
	                                  grpc::ServerWriter<agent::StatsFrame>* writer) override;
	grpc::Status UploadFile(grpc::ServerContext* context,
	                        grpc::ServerReader<agent::FileUploadMessage>* reader,
	                        agent::FileUploadResult* response) override;

private:
	// core/middleware.rs: the x-metrics-token metadata must equal the token
	// the agent was configured with, on every RPC.
	grpc::Status authenticate(grpc::ServerContext* context) const;
	grpc::Status check_server_id(int64_t requested) const;

	static agent::ContainerStatePoint to_state_point(const ContainerSummary& row);
	static int64_t label_id(const ContainerSummary& row, const std::string& key);

	const Config& config_;
	DockerClient& docker_;
	HostCollector& collector_;
	StatsHub& stats_;
	EventsHub& events_;
	FileTransferService& files_;
	ShellService& shells_;
};

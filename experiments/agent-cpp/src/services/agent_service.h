#pragma once
#include <asio/any_io_executor.hpp>
#include <grpcpp/grpcpp.h>

#include "agent.grpc.pb.h"
#include "core/async_util.h"
#include "telemetry/collector.h"
#include "core/config.h"
#include "docker/docker_client.h"
#include "services/file_transfer.h"
#include "docker/hubs.h"
#include "services/shell_service.h"

struct MetricsAgg;

// Mirrors agent/src/api/agent.rs: x-metrics-token auth on every call
// (core/middleware.rs), check_server_id, and delegation to the same
// services the Rust agent wires up. Every handler returns a callback-API
// reactor and continues as an Asio coroutine on the agent's io thread, so
// no thread is pinned per RPC or per open stream.
class AgentServiceImpl final : public agent::AgentService::CallbackService {
public:
	AgentServiceImpl(asio::any_io_executor executor, const Config& config, DockerClient& docker,
	                 HostCollector& collector, StatsHub& stats, EventsHub& events,
	                 FileTransferService& files, ShellService& shells)
	    : ex_(executor), config_(config), docker_(docker), collector_(collector), stats_(stats),
	      events_(events), files_(files), shells_(shells) {}

	grpc::ServerUnaryReactor* GetAgentInfo(grpc::CallbackServerContext* context,
	                                       const agent::AgentInfoRequest* request,
	                                       agent::AgentInfoResponse* response) override;
	grpc::ServerUnaryReactor* GetServerMetrics(grpc::CallbackServerContext* context,
	                                           const agent::GetMetricsRequest* request,
	                                           agent::ServerMetricsResponse* response) override;
	grpc::ServerUnaryReactor* GetContainerMetrics(grpc::CallbackServerContext* context,
	                                              const agent::GetContainerMetricsRequest* request,
	                                              agent::ContainerMetricsResponse* response) override;
	grpc::ServerUnaryReactor* GetContainerStates(grpc::CallbackServerContext* context,
	                                             const agent::GetMetricsRequest* request,
	                                             agent::ContainerStatesResponse* response) override;
	grpc::ServerWriteReactor<agent::ContainerStatesResponse>* WatchContainerStates(
	    grpc::CallbackServerContext* context, const agent::WatchRequest* request) override;
	grpc::ServerBidiReactor<agent::ShellMessage, agent::ShellMessage>* Shell(
	    grpc::CallbackServerContext* context) override;
	grpc::ServerWriteReactor<agent::ExecTaskOutput>* ExecTask(grpc::CallbackServerContext* context,
	                                                          const agent::ExecTaskRequest* request) override;
	grpc::ServerWriteReactor<agent::StatsFrame>* StreamContainerStats(
	    grpc::CallbackServerContext* context, const agent::StreamStatsRequest* request) override;
	grpc::ServerReadReactor<agent::FileUploadMessage>* UploadFile(grpc::CallbackServerContext* context,
	                                                              agent::FileUploadResult* response) override;

private:
	// core/middleware.rs: the x-metrics-token metadata must equal the token
	// the agent was configured with, on every RPC.
	grpc::Status authenticate(const grpc::CallbackServerContext* context) const;
	grpc::Status check_server_id(int64_t requested) const;

	static agent::ContainerStatePoint to_state_point(const ContainerSummary& row);
	static int64_t label_id(const ContainerSummary& row, const std::string& key);

	asio::awaitable<grpc::Status> info_call(agent::AgentInfoRequest request, agent::AgentInfoResponse* response);
	asio::awaitable<grpc::Status> server_metrics_call(agent::GetMetricsRequest request,
	                                                  agent::ServerMetricsResponse* response);
	asio::awaitable<grpc::Status> container_metrics_call(agent::GetContainerMetricsRequest request,
	                                                     agent::ContainerMetricsResponse* response);
	asio::awaitable<grpc::Status> states_call(agent::GetMetricsRequest request,
	                                          agent::ContainerStatesResponse* response);
	asio::awaitable<grpc::Status> watch_call(agent::WatchRequest request,
	                                         CoroutineWriteReactor<agent::ContainerStatesResponse>* reactor);
	asio::awaitable<grpc::Status> stats_stream_call(agent::StreamStatsRequest request,
	                                                CoroutineWriteReactor<agent::StatsFrame>* reactor);
	// One one-shot stats sample per container, fan-out like the Rust join_all.
	asio::awaitable<void> sample_container(ContainerSummary summary, int64_t timestamp,
	                                       std::shared_ptr<MetricsAgg> agg);

	asio::any_io_executor ex_;
	const Config& config_;
	DockerClient& docker_;
	HostCollector& collector_;
	StatsHub& stats_;
	EventsHub& events_;
	FileTransferService& files_;
	ShellService& shells_;
};

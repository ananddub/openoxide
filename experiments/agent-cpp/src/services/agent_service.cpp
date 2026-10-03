#include "services/agent_service.h"

#include <fmt/format.h>
#include <algorithm>
#include <asio/co_spawn.hpp>
#include <asio/detached.hpp>
#include <chrono>
#include <nlohmann/json.hpp>
#include <stdexcept>

#include "docker/docker_stats.h"

namespace {

int64_t now_seconds() {
	return std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch())
	    .count();
}

constexpr auto kBurstSilence = std::chrono::milliseconds(25);
constexpr auto kHeartbeat = std::chrono::milliseconds(15000);
constexpr auto kStatsPoll = std::chrono::milliseconds(1000);

// Unsubscribes a hub mailbox when the owning coroutine frame unwinds; runs on
// the io thread, which is where hub state may be touched.
template <typename Hub, typename Box>
struct HubGuard {
	Hub& hub;
	Box box;
	~HubGuard() { hub.unsubscribe(box); }
};

} // namespace

// One shared fan-out point for GetContainerMetrics' parallel samples. All
// access happens on the single io thread, so no locking is needed.
struct MetricsAgg {
	MetricsAgg(asio::any_io_executor ex, agent::ContainerMetricsResponse* out)
	    : response(out), signal(std::move(ex)) {}
	agent::ContainerMetricsResponse* response;
	asio::steady_timer signal;
	std::size_t pending = 0;
	bool waiting = false;
};

grpc::Status AgentServiceImpl::authenticate(const grpc::CallbackServerContext* context) const {
	return authenticate_token(context, config_.metrics_token);
}

grpc::Status AgentServiceImpl::check_server_id(int64_t requested) const {
	if (requested > 0 && (config_.server_id == -1 || requested == config_.server_id)) {
		return grpc::Status::OK;
	}
	return grpc::Status(grpc::StatusCode::NOT_FOUND,
	                        fmt::format("this agent serves server_id {}, not {}", config_.server_id, requested));
}

agent::ContainerStatePoint AgentServiceImpl::to_state_point(const ContainerSummary& row) {
	agent::ContainerStatePoint point;
	point.set_container_id(row.id.size() > 12 ? row.id.substr(0, 12) : row.id);
	point.set_name(row.name);
	point.set_state(row.state);
	point.set_status(row.status);
	point.set_application_id(label_id(row, "com.openoxide.application-id"));
	point.set_compose_id(label_id(row, "com.openoxide.compose-id"));
	point.set_compose_project(row.label("com.docker.compose.project"));
	point.set_compose_service(row.label("com.docker.compose.service"));
	return point;
}

int64_t AgentServiceImpl::label_id(const ContainerSummary& row, const std::string& key) {
	const std::string value = row.label(key);
	if (value.empty()) {
		return 0;
	}
	try {
		return std::stoll(value);
	} catch (...) {
		return 0;
	}
}

grpc::ServerUnaryReactor* AgentServiceImpl::GetAgentInfo(grpc::CallbackServerContext* context,
                                                         const agent::AgentInfoRequest* request,
                                                         agent::AgentInfoResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	return start_unary_call(ex_, context,
	                        [this, request, response] { return info_call(*request, response); });
}

grpc::ServerUnaryReactor* AgentServiceImpl::GetServerMetrics(grpc::CallbackServerContext* context,
                                                             const agent::GetMetricsRequest* request,
                                                             agent::ServerMetricsResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	return start_unary_call(ex_, context,
	                        [this, request, response] { return server_metrics_call(*request, response); });
}

grpc::ServerUnaryReactor* AgentServiceImpl::GetContainerMetrics(grpc::CallbackServerContext* context,
                                                                const agent::GetContainerMetricsRequest* request,
                                                                agent::ContainerMetricsResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	return start_unary_call(ex_, context,
	                        [this, request, response] { return container_metrics_call(*request, response); });
}

grpc::ServerUnaryReactor* AgentServiceImpl::GetContainerStates(grpc::CallbackServerContext* context,
                                                               const agent::GetMetricsRequest* request,
                                                               agent::ContainerStatesResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return start_failed_unary_call(context, status);
	}
	return start_unary_call(ex_, context,
	                        [this, request, response] { return states_call(*request, response); });
}

asio::awaitable<grpc::Status> AgentServiceImpl::info_call(agent::AgentInfoRequest /*request*/,
                                                          agent::AgentInfoResponse* response) {
	*response = files_.agent_info();
	co_return grpc::Status::OK;
}

asio::awaitable<grpc::Status> AgentServiceImpl::server_metrics_call(agent::GetMetricsRequest request,
                                                                    agent::ServerMetricsResponse* response) {
	const HostSample row = co_await collector_.latest();

	agent::HostInfo* host = response->mutable_host();
	host->set_cpu_model(row.cpu_model);
	host->set_cpu_cores(row.cpu_cores);
	host->set_cpu_physical_cores(row.cpu_physical_cores);
	host->set_cpu_speed(row.cpu_speed);
	host->set_os(row.os);
	host->set_distro(row.distro);
	host->set_kernel(row.kernel);
	host->set_arch(row.arch);

	// The sample keeps disk usage as a percentage of total; the wire contract
	// is GB, so derive the used amount (server_point in metrics.rs).
	agent::ServerMetricPoint* point = response->add_metrics();
	point->set_timestamp(row.timestamp);
	point->set_cpu(row.cpu);
	point->set_mem_used_gb(row.mem_used_gb);
	point->set_mem_total_gb(row.mem_total_gb);
	point->set_disk_used_gb(row.disk_total_gb * row.disk_used_percent / 100.0);
	point->set_disk_total_gb(row.disk_total_gb);
	point->set_network_in(row.net_in_mb);
	point->set_network_out(row.net_out_mb);
	point->set_uptime(row.uptime);
	co_return grpc::Status::OK;
}

asio::awaitable<grpc::Status> AgentServiceImpl::container_metrics_call(agent::GetContainerMetricsRequest request,
                                                                       agent::ContainerMetricsResponse* response) {
	std::vector<ContainerSummary> summaries;
	try {
		summaries = co_await ContainerSummary::list(docker_, false);
	} catch (const std::exception& error) {
		co_return grpc::Status(grpc::StatusCode::INTERNAL, std::string("docker container list: ") + error.what());
	}
	std::vector<ContainerSummary> matched;
	for (const auto& summary : summaries) {
		if (summary.name.find(request.app_name()) != std::string::npos) {
			matched.push_back(summary);
		}
	}
	if (matched.empty()) {
		co_return grpc::Status::OK;
	}

	// One one-shot stats coroutine per matched container, in parallel — same as
	// `docker stats --no-stream`.
	const int64_t timestamp = now_seconds();
	auto agg = std::make_shared<MetricsAgg>(ex_, response);
	agg->pending = matched.size();
	for (auto& summary : matched) {
		asio::co_spawn(ex_, sample_container(std::move(summary), timestamp, agg), asio::detached);
	}
	while (agg->pending > 0) {
		agg->waiting = true;
		agg->signal.expires_after(std::chrono::seconds(60)); // bounded fallback
		try {
			co_await agg->signal.async_wait(asio::use_awaitable);
		} catch (const asio::system_error&) {
		}
		agg->waiting = false;
	}
	co_return grpc::Status::OK;
}

asio::awaitable<void> AgentServiceImpl::sample_container(ContainerSummary summary, int64_t timestamp,
                                                         std::shared_ptr<MetricsAgg> agg) {
	DockerStatsDoc doc;
	bool sampled = true;
	try {
		const std::string body = co_await docker_.get(fmt::format("/containers/{}/stats?stream=false", summary.id));
		doc = parse_stats_doc(nlohmann::json::parse(body, nullptr, false));
	} catch (...) {
		sampled = false; // a failed sample is skipped, like the Rust None branch
	}
	if (sampled) {
		agent::ContainerMetricPoint* point = agg->response->add_metrics();
		point->set_id(0);
		point->set_timestamp(timestamp);
		point->set_container_id(doc.id);
		std::string name = summary.name;
		if (name.empty()) {
			name = doc.display_name();
		}
		point->set_name(name);
		point->set_cpu_perc(doc.cpu_percent());
		point->set_mem_perc(doc.mem_used_percent());
		point->set_mem_used_mb(bytes_to_mb(static_cast<double>(doc.mem_used_bytes())));
		point->set_mem_total_mb(bytes_to_mb(static_cast<double>(doc.mem_limit)));
		point->set_net_in_mb(bytes_to_mb(static_cast<double>(doc.net_rx)));
		point->set_net_out_mb(bytes_to_mb(static_cast<double>(doc.net_tx)));
		point->set_block_read_mb(bytes_to_mb(static_cast<double>(doc.blk_read)));
		point->set_block_write_mb(bytes_to_mb(static_cast<double>(doc.blk_write)));
		point->set_application_id(label_id(summary, "com.openoxide.application-id"));
		point->set_compose_id(label_id(summary, "com.openoxide.compose-id"));
	}
	if (--agg->pending == 0 && agg->waiting) {
		agg->signal.cancel();
	}
}

asio::awaitable<grpc::Status> AgentServiceImpl::states_call(agent::GetMetricsRequest /*request*/,
                                                            agent::ContainerStatesResponse* response) {
	std::vector<ContainerSummary> rows;
	try {
		rows = co_await ContainerSummary::list(docker_, true);
	} catch (const std::exception& error) {
		co_return grpc::Status(grpc::StatusCode::UNAVAILABLE, error.what());
	}
	for (const auto& row : rows) {
		*response->add_containers() = to_state_point(row);
	}
	co_return grpc::Status::OK;
}

grpc::ServerWriteReactor<agent::ContainerStatesResponse>* AgentServiceImpl::WatchContainerStates(
    grpc::CallbackServerContext* context, const agent::WatchRequest* request) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_write_call<agent::ContainerStatesResponse>(ex_, status);
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return start_failed_write_call<agent::ContainerStatesResponse>(ex_, status);
	}
	return start_write_stream<agent::ContainerStatesResponse>(
	    ex_, [this, request](auto* reactor) { return watch_call(*request, reactor); });
}

grpc::ServerWriteReactor<agent::StatsFrame>* AgentServiceImpl::StreamContainerStats(
    grpc::CallbackServerContext* context, const agent::StreamStatsRequest* request) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_write_call<agent::StatsFrame>(ex_, status);
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return start_failed_write_call<agent::StatsFrame>(ex_, status);
	}
	return start_write_stream<agent::StatsFrame>(
	    ex_, [this, request](auto* reactor) { return stats_stream_call(*request, reactor); });
}

// Same contract as lifecycle.rs: initial snapshot, then a refresh on every
// coalesced burst of Docker events and on a 15 s heartbeat. Only the newest
// snapshot is delivered (events drained before each refresh).
asio::awaitable<grpc::Status> AgentServiceImpl::watch_call(
    agent::WatchRequest request,
    CoroutineWriteReactor<agent::ContainerStatesResponse>* reactor) {
	HubGuard<EventsHub, std::shared_ptr<Mailbox<std::string>>> guard{events_, events_.subscribe()};

	auto refresh = [this](agent::ContainerStatesResponse* snapshot) -> asio::awaitable<bool> {
		std::vector<ContainerSummary> rows;
		try {
			rows = co_await ContainerSummary::list(docker_, true);
		} catch (const std::exception&) {
			// Rust logs at debug and keeps the previous state; a failed read
			// must not kill the watch.
			co_return false;
		}
		std::vector<agent::ContainerStatePoint> containers;
		containers.reserve(rows.size());
		for (const auto& row : rows) {
			containers.push_back(to_state_point(row));
		}
		std::sort(containers.begin(), containers.end(),
		          [](const agent::ContainerStatePoint& a, const agent::ContainerStatePoint& b) {
			          return a.container_id() < b.container_id();
		          });
		snapshot->clear_containers();
		for (auto& container : containers) {
			*snapshot->add_containers() = std::move(container);
		}
		co_return true;
	};

	agent::ContainerStatesResponse snapshot;
	if (!co_await refresh(&snapshot)) {
		co_return grpc::Status(grpc::StatusCode::UNAVAILABLE, "could not read initial container state");
	}
	if (!co_await reactor->write(snapshot)) {
		co_return grpc::Status::OK;
	}

	while (!reactor->cancelled()) {
		auto item = co_await guard.box->next_within(kHeartbeat);
		if (item) {
			// Coalesce the burst: keep draining while events keep coming.
			while (true) {
				auto extra = co_await guard.box->next_within(kBurstSilence);
				if (!extra) {
					break;
				}
			}
		}
		if (reactor->cancelled()) {
			break;
		}
		if (co_await refresh(&snapshot) && !co_await reactor->write(snapshot)) {
			break;
		}
	}
	co_return grpc::Status::OK;
}

asio::awaitable<grpc::Status> AgentServiceImpl::stats_stream_call(
    agent::StreamStatsRequest request,
    CoroutineWriteReactor<agent::StatsFrame>* reactor) {
	std::vector<std::string> ids(request.container_ids().begin(), request.container_ids().end());
	const bool streaming = request.stream();

	HubGuard<StatsHub, std::shared_ptr<Mailbox<std::shared_ptr<DockerStatsDoc>>>> guard{stats_, stats_.subscribe()};
	auto emit = [&](const DockerStatsDoc& doc) -> asio::awaitable<bool> {
		agent::StatsFrame frame;
		frame.set_json_line(stats_to_cli_line(doc, stats_.size_rw(doc.id)));
		co_return co_await reactor->write(std::move(frame));
	};

	std::vector<std::string> seen; // one-shot, all containers
	// Rust hands ids straight to the daemon, which resolves names too; the
	// shared hub filters locally, so match the stats doc name likewise.
	const auto matches_doc = [&ids](const DockerStatsDoc& doc) {
		if (ids.empty() || docker_id_matches_any(ids, doc.id)) {
			return true;
		}
		std::string name = doc.name;
		if (!name.empty() && name.front() == '/') {
			name.erase(name.begin());
		}
		return std::any_of(ids.begin(), ids.end(),
		                   [&](const std::string& id) { return id == name; });
	};
	while (!reactor->cancelled()) {
		auto item = co_await guard.box->next_within(kStatsPoll);
		if (!item) {
			continue;
		}
		const DockerStatsDoc& doc = **item;
		if (!ids.empty() && !matches_doc(doc)) {
			continue;
		}
		if (streaming) {
			if (!co_await emit(doc)) {
				break;
			}
			continue;
		}
		if (!ids.empty()) {
			// One-shot, named containers: one sample each.
			if (!co_await emit(doc)) {
				break;
			}
			std::string name = doc.name;
			if (!name.empty() && name.front() == '/') {
				name.erase(name.begin());
			}
			std::erase_if(ids, [&](const std::string& id) {
				return docker_id_matches(id, doc.id) || id == name;
			});
			if (ids.empty()) {
				break;
			}
			continue;
		}
		// One-shot, all containers: the hub cycles every running container,
		// so stop when a repeat arrives.
		if (std::any_of(seen.begin(), seen.end(),
		                [&](const std::string& id) { return docker_id_matches(id, doc.id); })) {
			break;
		}
		seen.push_back(doc.id);
		if (!co_await emit(doc)) {
			break;
		}
	}
	co_return grpc::Status::OK;
}

grpc::ServerBidiReactor<agent::ShellMessage, agent::ShellMessage>* AgentServiceImpl::Shell(
    grpc::CallbackServerContext* context) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_bidi_call<agent::ShellMessage, agent::ShellMessage>(ex_, status);
	}
	return shells_.open(context);
}

grpc::ServerWriteReactor<agent::ExecTaskOutput>* AgentServiceImpl::ExecTask(
    grpc::CallbackServerContext* context, const agent::ExecTaskRequest* request) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_write_call<agent::ExecTaskOutput>(ex_, status);
	}
	return shells_.start_exec_task(context, request);
}

grpc::ServerReadReactor<agent::FileUploadMessage>* AgentServiceImpl::UploadFile(
    grpc::CallbackServerContext* context, agent::FileUploadResult* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return start_failed_read_call<agent::FileUploadMessage>(ex_, status);
	}
	// Rust's upload() validates server_id itself; errors are user-input
	// failures mapped to INVALID_ARGUMENT, same as the Rust Status codes.
	return files_.start(context, response);
}

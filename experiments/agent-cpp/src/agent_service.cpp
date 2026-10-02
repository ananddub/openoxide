#include "agent_service.h"

#include <algorithm>
#include <chrono>
#include <future>
#include <nlohmann/json.hpp>
#include <stdexcept>
#include <thread>

#include "docker_stats.h"

namespace {

int64_t now_seconds() {
	return std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch())
	    .count();
}

} // namespace

grpc::Status AgentServiceImpl::authenticate(grpc::ServerContext* context) const {
	const auto metadata = context->client_metadata();
	const auto found = metadata.find("x-metrics-token");
	if (found != metadata.end() &&
	    std::string_view(found->second.data(), found->second.size()) == config_.metrics_token) {
		return grpc::Status::OK;
	}
	return grpc::Status(grpc::StatusCode::UNAUTHENTICATED, "invalid agent token");
}

grpc::Status AgentServiceImpl::check_server_id(int64_t requested) const {
	if (requested > 0 && (config_.server_id == -1 || requested == config_.server_id)) {
		return grpc::Status::OK;
	}
	return grpc::Status(grpc::StatusCode::NOT_FOUND,
	                    "this agent serves server_id " + std::to_string(config_.server_id) + ", not " +
	                        std::to_string(requested));
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

grpc::Status AgentServiceImpl::GetAgentInfo(grpc::ServerContext* context,
                                            const agent::AgentInfoRequest* request,
                                            agent::AgentInfoResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return status;
	}
	*response = files_.agent_info();
	return grpc::Status::OK;
}

grpc::Status AgentServiceImpl::GetServerMetrics(grpc::ServerContext* context,
                                                const agent::GetMetricsRequest* request,
                                                agent::ServerMetricsResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return status;
	}
	const HostSample row = collector_.latest();

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
	return grpc::Status::OK;
}

grpc::Status AgentServiceImpl::GetContainerMetrics(grpc::ServerContext* context,
                                                   const agent::GetContainerMetricsRequest* request,
                                                   agent::ContainerMetricsResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return status;
	}

	std::vector<ContainerSummary> summaries;
	try {
		summaries = ContainerSummary::list(docker_, false);
	} catch (const std::exception& error) {
		return grpc::Status(grpc::StatusCode::INTERNAL,
		                    std::string("docker container list: ") + error.what());
	}
	std::vector<ContainerSummary> matched;
	for (const auto& summary : summaries) {
		if (summary.name.find(request->app_name()) != std::string::npos) {
			matched.push_back(summary);
		}
	}
	if (matched.empty()) {
		return grpc::Status::OK;
	}

	// One one-shot stats call per matched container, in parallel — same as
	// `docker stats --no-stream`.
	const int64_t timestamp = now_seconds();
	std::vector<std::future<void>> tasks;
	std::mutex response_mutex;
	for (const auto& summary : matched) {
		tasks.push_back(std::async(std::launch::async, [&, summary] {
			DockerStatsDoc doc;
			try {
				const std::string body = docker_.get("/containers/" + summary.id + "/stats?stream=false");
				doc = parse_stats_doc(nlohmann::json::parse(body, nullptr, false));
			} catch (...) {
				return; // a failed sample is skipped, like the Rust None branch
			}
			agent::ContainerMetricPoint* point;
			{
				std::lock_guard<std::mutex> guard(response_mutex);
				point = response->add_metrics();
			}
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
		}));
	}
	for (auto& task : tasks) {
		task.wait();
	}
	return grpc::Status::OK;
}

grpc::Status AgentServiceImpl::GetContainerStates(grpc::ServerContext* context,
                                                  const agent::GetMetricsRequest* request,
                                                  agent::ContainerStatesResponse* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return status;
	}
	std::vector<ContainerSummary> rows;
	try {
		rows = ContainerSummary::list(docker_, true);
	} catch (const std::exception& error) {
		return grpc::Status(grpc::StatusCode::UNAVAILABLE, error.what());
	}
	for (const auto& row : rows) {
		*response->add_containers() = to_state_point(row);
	}
	return grpc::Status::OK;
}

grpc::Status AgentServiceImpl::WatchContainerStates(grpc::ServerContext* context,
                                                    const agent::WatchRequest* request,
                                                    grpc::ServerWriter<agent::ContainerStatesResponse>* writer) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return status;
	}

	// Same contract as lifecycle.rs: initial snapshot, then a refresh on every
	// coalesced burst of Docker events and on a 15 s heartbeat. Only the
	// newest snapshot is delivered (events drained before each refresh).
	auto refresh = [&](agent::ContainerStatesResponse* snapshot) -> bool {
		std::vector<ContainerSummary> rows;
		try {
			rows = ContainerSummary::list(docker_, true);
		} catch (const std::exception& error) {
			// Rust logs at debug and keeps the previous state; a failed read
			// must not kill the watch.
			(void)error;
			return false;
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
		return true;
	};

	agent::ContainerStatesResponse snapshot;
	if (!refresh(&snapshot)) {
		return grpc::Status(grpc::StatusCode::UNAVAILABLE, "could not read initial container state");
	}
	if (!writer->Write(snapshot)) {
		return grpc::Status::OK;
	}

	auto events = events_.subscribe();
	struct Unsubscribe {
		EventsHub& hub;
		std::shared_ptr<Subscriber<std::string>>& sub;
		~Unsubscribe() { hub.unsubscribe(sub); }
	} unsubscribe{events_, events};

	bool changed = false;
	while (!context->IsCancelled()) {
		const auto item = events->wait_for(changed ? 25 : 15000);
		if (item.has_value()) {
			changed = true;
			continue;
		}
		if (changed) {
			// Coalesce the burst: drop pending events, read state once.
			while (events->wait_for(0).has_value()) {
			}
			changed = false;
		} else {
			changed = false; // heartbeat
		}
		if (context->IsCancelled()) {
			break;
		}
		if (refresh(&snapshot) && !writer->Write(snapshot)) {
			break;
		}
	}
	return grpc::Status::OK;
}

grpc::Status AgentServiceImpl::Shell(grpc::ServerContext* context,
                                     grpc::ServerReaderWriter<agent::ShellMessage, agent::ShellMessage>* stream) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	return shells_.Open(context, stream);
}

grpc::Status AgentServiceImpl::ExecTask(grpc::ServerContext* context, const agent::ExecTaskRequest* request,
                                        grpc::ServerWriter<agent::ExecTaskOutput>* writer) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	return shells_.ExecTask(context, *request, writer);
}

grpc::Status AgentServiceImpl::StreamContainerStats(grpc::ServerContext* context,
                                                    const agent::StreamStatsRequest* request,
                                                    grpc::ServerWriter<agent::StatsFrame>* writer) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	if (auto status = check_server_id(request->server_id()); !status.ok()) {
		return status;
	}

	std::vector<std::string> ids(request->container_ids().begin(), request->container_ids().end());
	const bool streaming = request->stream();

	auto sub = stats_.subscribe();
	struct Unsubscribe {
		StatsHub& hub;
		std::shared_ptr<Subscriber<DockerStatsDoc>>& sub;
		~Unsubscribe() { hub.unsubscribe(sub); }
	} unsubscribe{stats_, sub};

	auto emit = [&](const DockerStatsDoc& doc) -> bool {
		agent::StatsFrame frame;
		frame.set_json_line(stats_to_cli_line(doc, stats_.size_rw(doc.id)));
		return writer->Write(frame);
	};

	std::vector<std::string> seen; // one-shot, all containers
	while (!context->IsCancelled()) {
		const auto item = sub->wait_for(1000);
		if (!item.has_value()) {
			continue;
		}
		const DockerStatsDoc& doc = **item;
		if (!ids.empty() && !docker_id_matches_any(ids, doc.id)) {
			continue;
		}
		if (streaming) {
			if (!emit(doc)) {
				break;
			}
			continue;
		}
		if (!ids.empty()) {
			// One-shot, named containers: one sample each.
			if (!emit(doc)) {
				break;
			}
			std::erase_if(ids, [&](const std::string& id) { return docker_id_matches(id, doc.id); });
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
		if (!emit(doc)) {
			break;
		}
	}
	return grpc::Status::OK;
}

grpc::Status AgentServiceImpl::UploadFile(grpc::ServerContext* context,
                                          grpc::ServerReader<agent::FileUploadMessage>* reader,
                                          agent::FileUploadResult* response) {
	if (auto status = authenticate(context); !status.ok()) {
		return status;
	}
	// Rust's upload() validates server_id itself; errors are user-input
	// failures mapped to INVALID_ARGUMENT, same as the Rust Status codes.
	try {
		*response = files_.upload(context, reader);
	} catch (const std::exception& error) {
		return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, error.what());
	}
	return grpc::Status::OK;
}

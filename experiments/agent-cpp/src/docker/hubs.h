#pragma once

#include <asio/any_io_executor.hpp>
#include <asio/awaitable.hpp>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "core/async_util.h"
#include "docker/docker_client.h"
#include "docker/docker_stats.h"

// All hub methods must run on the hub's io executor (RPC coroutines), so the
// internal state is single-threaded; Mailbox itself is safe to push from the
// gRPC EventEngine threads.

// A container entry from GET /containers/json (ContainerSummary in Rust).
struct ContainerSummary {
	std::string id;
	std::string name; // leading '/' stripped
	std::string state;
	std::string status;
	std::vector<std::pair<std::string, std::string>> labels;

	std::string label(const std::string& key) const;
	static asio::awaitable<std::vector<ContainerSummary>> list(DockerClient& docker, bool all);
};

// One shared /stats feed per running container while listeners exist —
// mirrors agent/src/services/monitoring/hub.rs (StatsHub). Feeds are
// coroutines, not threads: an idle container costs no scheduler resource.
class StatsHub {
public:
	StatsHub(asio::any_io_executor executor, DockerClient& docker) : ex_(executor), docker_(docker), reconcile_timer_(executor) {}
	~StatsHub();

	std::shared_ptr<Mailbox<std::shared_ptr<DockerStatsDoc>>> subscribe();
	void unsubscribe(const std::shared_ptr<Mailbox<std::shared_ptr<DockerStatsDoc>>>& sub);
	unsigned long long size_rw(const std::string& doc_id);

private:
	struct FeedState {
		std::string id;
		std::shared_ptr<DockerStream> stream;
		bool stop = false;
	};

	void ensure_pump();
	asio::awaitable<void> pump();
	asio::awaitable<void> feed(std::shared_ptr<FeedState> state);
	void stop_all_feeds();

	asio::any_io_executor ex_;
	DockerClient& docker_;
	std::vector<std::shared_ptr<Mailbox<std::shared_ptr<DockerStatsDoc>>>> subs_;
	std::map<std::string, unsigned long long> sizes_;
	std::vector<std::shared_ptr<FeedState>> feeds_;
	asio::steady_timer reconcile_timer_;
	bool running_ = false;
};

// One shared /events connection, broadcast as raw JSON lines (EventsHub).
class EventsHub {
public:
	EventsHub(asio::any_io_executor executor, DockerClient& docker) : ex_(executor), docker_(docker), idle_timer_(executor) {}
	~EventsHub();

	std::shared_ptr<Mailbox<std::string>> subscribe();
	void unsubscribe(const std::shared_ptr<Mailbox<std::string>>& sub);

private:
	void ensure_pump();
	asio::awaitable<void> pump();

	asio::any_io_executor ex_;
	DockerClient& docker_;
	std::vector<std::shared_ptr<Mailbox<std::string>>> subs_;
	std::shared_ptr<DockerStream> stream_;
	asio::steady_timer idle_timer_;
	bool running_ = false;
};

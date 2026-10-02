#pragma once
#include <atomic>
#include <condition_variable>
#include <deque>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <string>
#include <thread>
#include <vector>

#include "docker_client.h"
#include "docker_stats.h"

// Bounded fan-out queue; mirrors the Rust broadcast channel (capacity 256,
// oldest dropped on lag). Each live RPC stream owns one subscriber and must
// unsubscribe when the stream ends so the hubs can tear their feeds down.
template <typename T>
class Subscriber {
public:
	void push(std::shared_ptr<T> item) {
		std::lock_guard<std::mutex> guard(mutex_);
		if (queue_.size() >= kCapacity) {
			queue_.pop_front();
		}
		queue_.push_back(std::move(item));
		cv_.notify_one();
	}

	// Returns an item, or nullopt when the timeout fires.
	std::optional<std::shared_ptr<T>> wait_for(int timeout_ms) {
		std::unique_lock<std::mutex> lock(mutex_);
		if (queue_.empty()) {
			cv_.wait_for(lock, std::chrono::milliseconds(timeout_ms), [this] { return !queue_.empty(); });
		}
		if (queue_.empty()) {
			return std::nullopt;
		}
		auto item = queue_.front();
		queue_.pop_front();
		return item;
	}

private:
	static constexpr size_t kCapacity = 256;
	std::mutex mutex_;
	std::condition_variable cv_;
	std::deque<std::shared_ptr<T>> queue_;
};

// A container entry from GET /containers/json (ContainerSummary in Rust).
struct ContainerSummary {
	std::string id;
	std::string name; // leading '/' stripped
	std::string state;
	std::string status;
	std::vector<std::pair<std::string, std::string>> labels;

	std::string label(const std::string& key) const;
	static std::vector<ContainerSummary> list(DockerClient& docker, bool all);
};

// One shared /stats feed per running container while listeners exist —
// mirrors agent/src/services/monitoring/hub.rs (StatsHub).
class StatsHub {
public:
	explicit StatsHub(DockerClient& docker) : docker_(docker) {}
	~StatsHub();

	std::shared_ptr<Subscriber<DockerStatsDoc>> subscribe();
	void unsubscribe(const std::shared_ptr<Subscriber<DockerStatsDoc>>& sub);
	unsigned long long size_rw(const std::string& doc_id);

private:
	struct Feed {
		std::string id;
		std::shared_ptr<std::atomic<bool>> stop;
		std::thread thread;
	};

	void pump();

	DockerClient& docker_;
	std::mutex mutex_;
	std::vector<std::shared_ptr<Subscriber<DockerStatsDoc>>> subs_;
	std::map<std::string, unsigned long long> sizes_;
	std::vector<std::shared_ptr<Subscriber<DockerStatsDoc>>> subs_snapshot();
	std::unique_ptr<std::thread> pump_;
	std::shared_ptr<std::atomic<bool>> pump_stop_;
};

// One shared /events connection, broadcast as raw JSON lines (EventsHub).
class EventsHub {
public:
	explicit EventsHub(DockerClient& docker) : docker_(docker) {}
	~EventsHub();

	std::shared_ptr<Subscriber<std::string>> subscribe();
	void unsubscribe(const std::shared_ptr<Subscriber<std::string>>& sub);

private:
	void pump();

	DockerClient& docker_;
	std::mutex mutex_;
	std::vector<std::shared_ptr<Subscriber<std::string>>> subs_;
	std::vector<std::shared_ptr<Subscriber<std::string>>> subs_snapshot();
	std::unique_ptr<std::thread> pump_;
	std::shared_ptr<std::atomic<bool>> pump_stop_;
};

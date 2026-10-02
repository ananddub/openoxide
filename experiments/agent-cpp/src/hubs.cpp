#include "hubs.h"

#include <algorithm>
#include <chrono>
#include <cstring>
#include <nlohmann/json.hpp>
#include <unistd.h>

#include "json_split.h"

namespace {

constexpr int kReconcileMs = 5000;

void sleep_slices(int ms, const std::atomic<bool>& stop) {
	const auto deadline = std::chrono::steady_clock::now() + std::chrono::milliseconds(ms);
	while (std::chrono::steady_clock::now() < deadline && !stop) {
		std::this_thread::sleep_for(std::chrono::milliseconds(100));
	}
}

} // namespace

std::string ContainerSummary::label(const std::string& key) const {
	for (const auto& [name, value] : labels) {
		if (name == key) {
			return value;
		}
	}
	return "";
}

std::vector<ContainerSummary> ContainerSummary::list(DockerClient& docker, bool all) {
	const std::string path = all ? "/containers/json?all=true" : "/containers/json";
	const auto body = docker.get(path);
	const auto parsed = nlohmann::json::parse(body, nullptr, false);
	std::vector<ContainerSummary> out;
	if (!parsed.is_array()) {
		return out;
	}
	for (const auto& row : parsed) {
		ContainerSummary summary;
		summary.id = row.value("Id", "");
		if (const auto names = row.find("Names"); names != row.end() && names->is_array() && !names->empty()) {
			summary.name = names->front().get<std::string>();
			if (!summary.name.empty() && summary.name.front() == '/') {
				summary.name.erase(summary.name.begin());
			}
		}
		summary.state = row.value("State", "");
		summary.status = row.value("Status", "");
		if (const auto labels = row.find("Labels"); labels != row.end() && labels->is_object()) {
			for (const auto& [key, value] : labels->items()) {
				summary.labels.emplace_back(key, value.is_string() ? value.get<std::string>() : "");
			}
		}
		out.push_back(std::move(summary));
	}
	return out;
}

// ------------------------------ StatsHub ------------------------------

StatsHub::~StatsHub() {
	std::lock_guard<std::mutex> guard(mutex_);
	if (pump_stop_) {
		*pump_stop_ = true;
	}
	if (pump_ && pump_->joinable()) {
		pump_->join();
	}
	pump_.reset();
}

std::shared_ptr<Subscriber<DockerStatsDoc>> StatsHub::subscribe() {
	std::lock_guard<std::mutex> guard(mutex_);
	auto sub = std::make_shared<Subscriber<DockerStatsDoc>>();
	subs_.push_back(sub);
	if (!pump_) {
		pump_stop_ = std::make_shared<std::atomic<bool>>(false);
		pump_ = std::make_unique<std::thread>([this] { pump(); });
	}
	return sub;
}

void StatsHub::unsubscribe(const std::shared_ptr<Subscriber<DockerStatsDoc>>& sub) {
	std::unique_lock<std::mutex> lock(mutex_);
	subs_.erase(std::remove(subs_.begin(), subs_.end(), sub), subs_.end());
	if (!subs_.empty() || !pump_) {
		return;
	}
	*pump_stop_ = true;
	auto thread = std::move(pump_);
	lock.unlock();
	if (thread->joinable()) {
		thread->join();
	}
	lock.lock();
	pump_stop_.reset();
}

std::vector<std::shared_ptr<Subscriber<DockerStatsDoc>>> StatsHub::subs_snapshot() {
	std::lock_guard<std::mutex> guard(mutex_);
	return subs_;
}

unsigned long long StatsHub::size_rw(const std::string& doc_id) {
	std::lock_guard<std::mutex> guard(mutex_);
	for (const auto& [id, size] : sizes_) {
		if (doc_id == id || doc_id.starts_with(id) || id.starts_with(doc_id.substr(0, 12))) {
			return size;
		}
	}
	return 0;
}

void StatsHub::pump() {
	std::vector<Feed> feeds;
	const std::atomic<bool>& stop = *pump_stop_;

	while (!stop) {
		{
			std::lock_guard<std::mutex> guard(mutex_);
			if (subs_.empty()) {
				break;
			}
		}

		std::vector<std::string> live_ids;
		try {
			for (const auto& row : ContainerSummary::list(docker_, false)) {
				live_ids.push_back(row.id);
			}
		} catch (const std::exception&) {
			sleep_slices(kReconcileMs, stop);
			continue;
		}

		// Reap feeds for containers that are gone.
		feeds.erase(std::remove_if(feeds.begin(), feeds.end(),
		                    [&live_ids](Feed& feed) {
			                    const bool live = std::find(live_ids.begin(), live_ids.end(), feed.id) != live_ids.end();
			                    if (!live) {
				                    *feed.stop = true;
				                    if (feed.thread.joinable()) {
					                    feed.thread.join();
				                    }
			                    }
			                    return !live;
		                    }),
		    feeds.end());
		{
			std::lock_guard<std::mutex> guard(mutex_);
			for (auto it = sizes_.begin(); it != sizes_.end();) {
				if (std::find(live_ids.begin(), live_ids.end(), it->first) == live_ids.end()) {
					it = sizes_.erase(it);
				} else {
					++it;
				}
			}
		}

		// Open feeds for new containers; inspect size_rw once per stream open.
		for (const auto& id : live_ids) {
			const bool exists = std::any_of(feeds.begin(), feeds.end(), [&id](const Feed& f) { return f.id == id; });
			if (exists) {
				continue;
			}
			unsigned long long size = 0;
			try {
				const auto body = docker_.get("/containers/" + id + "/json?size=1");
				const auto doc = nlohmann::json::parse(body, nullptr, false);
				if (!doc.is_discarded() && doc.contains("SizeRw")) {
					size = doc["SizeRw"].get<unsigned long long>();
				}
			} catch (const std::exception&) {
			}
			{
				std::lock_guard<std::mutex> guard(mutex_);
				sizes_[id] = size;
			}

			Feed feed;
			feed.id = id;
			feed.stop = std::make_shared<std::atomic<bool>>(false);
			auto stop_flag = feed.stop;
			auto* hub = this;
			feed.thread = std::thread([hub, id, stop_flag] {
				JsonDocumentScanner scanner;
				std::unique_ptr<DockerStream> stream;
				try {
					stream = hub->docker_.open_stream("/containers/" + id + "/stats?stream=true");
				} catch (const std::exception&) {
					*stop_flag = true;
					return;
				}
				char buffer[64 * 1024];
				while (!*stop_flag) {
					const int n = stream->read(buffer, sizeof(buffer));
					if (n < 0) {
						break;
					}
					if (n == 0) {
						continue; // receive-timeout; re-check stop flag
					}
					for (auto& doc : scanner.push(buffer, static_cast<size_t>(n))) {
						if (doc.is_discarded() || !doc.is_object()) {
							continue;
						}
						auto stats = std::make_shared<DockerStatsDoc>(parse_stats_doc(doc));
						for (const auto& sub : hub->subs_snapshot()) {
							sub->push(stats);
						}
					}
				}
			});
			feeds.push_back(std::move(feed));
		}

		sleep_slices(kReconcileMs, stop);
	}

	for (auto& feed : feeds) {
		*feed.stop = true;
	}
	for (auto& feed : feeds) {
		if (feed.thread.joinable()) {
			feed.thread.join();
		}
	}
	std::lock_guard<std::mutex> guard(mutex_);
	sizes_.clear();
}

// ------------------------------ EventsHub ------------------------------

EventsHub::~EventsHub() {
	std::lock_guard<std::mutex> guard(mutex_);
	if (pump_stop_) {
		*pump_stop_ = true;
	}
	if (pump_ && pump_->joinable()) {
		pump_->join();
	}
}

std::shared_ptr<Subscriber<std::string>> EventsHub::subscribe() {
	std::lock_guard<std::mutex> guard(mutex_);
	auto sub = std::make_shared<Subscriber<std::string>>();
	subs_.push_back(sub);
	if (!pump_) {
		pump_stop_ = std::make_shared<std::atomic<bool>>(false);
		pump_ = std::make_unique<std::thread>([this] { pump(); });
	}
	return sub;
}

void EventsHub::unsubscribe(const std::shared_ptr<Subscriber<std::string>>& sub) {
	std::unique_lock<std::mutex> lock(mutex_);
	subs_.erase(std::remove(subs_.begin(), subs_.end(), sub), subs_.end());
	if (!subs_.empty() || !pump_) {
		return;
	}
	*pump_stop_ = true;
	auto thread = std::move(pump_);
	lock.unlock();
	if (thread->joinable()) {
		thread->join();
	}
	lock.lock();
	pump_stop_.reset();
}

std::vector<std::shared_ptr<Subscriber<std::string>>> EventsHub::subs_snapshot() {
	std::lock_guard<std::mutex> guard(mutex_);
	return subs_;
}

void EventsHub::pump() {
	std::unique_ptr<DockerStream> stream;
	try {
		stream = docker_.open_stream("/events?filters=%7B%7D");
	} catch (const std::exception&) {
		return;
	}
	std::string pending;
	char buffer[64 * 1024];
	while (!*pump_stop_) {
		const int n = stream->read(buffer, sizeof(buffer));
		if (n < 0) {
			break;
		}
		if (n == 0) {
			std::lock_guard<std::mutex> guard(mutex_);
			if (subs_.empty()) {
				break;
			}
			continue;
		}
		pending.append(buffer, static_cast<size_t>(n));
		if (pending.size() > 1024 * 1024) {
			pending.clear();
			continue;
		}
		size_t pos;
		while ((pos = pending.find('\n')) != std::string::npos) {
			auto line = std::make_shared<std::string>(pending.substr(0, pos + 1));
			pending.erase(0, pos + 1);
			for (const auto& sub : subs_snapshot()) {
				sub->push(line);
			}
		}
	}
}

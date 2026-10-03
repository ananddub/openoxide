#include "docker/hubs.h"

#include <fmt/format.h>
#include <algorithm>
#include <asio/co_spawn.hpp>
#include <asio/detached.hpp>
#include <chrono>
#include <nlohmann/json.hpp>

#include "core/json_split.h"

namespace {

constexpr int kReconcileMs = 5000;
constexpr std::size_t kLineCarryMax = 1024 * 1024;

} // namespace

std::string ContainerSummary::label(const std::string& key) const {
	for (const auto& [name, value] : labels) {
		if (name == key) {
			return value;
		}
	}
	return "";
}

asio::awaitable<std::vector<ContainerSummary>> ContainerSummary::list(DockerClient& docker, bool all) {
	const std::string path = all ? "/containers/json?all=true" : "/containers/json";
	const auto body = co_await docker.get(path);
	const auto parsed = nlohmann::json::parse(body, nullptr, false);
	std::vector<ContainerSummary> out;
	if (!parsed.is_array()) {
		co_return out;
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
	co_return out;
}

// ------------------------------ StatsHub ------------------------------

namespace {

// Sleep ms on the hub timer. Returns false when cancelled — that is how the
// last unsubscribe tears the pump down.
asio::awaitable<bool> timer_wait(asio::steady_timer& timer, int ms) {
	timer.expires_after(std::chrono::milliseconds(ms));
	try {
		co_await timer.async_wait(asio::use_awaitable);
	} catch (const asio::system_error&) {
		co_return false;
	}
	co_return true;
}

} // namespace

StatsHub::~StatsHub() {
	stop_all_feeds();
	reconcile_timer_.cancel();
}

std::shared_ptr<Mailbox<std::shared_ptr<DockerStatsDoc>>> StatsHub::subscribe() {
	auto box = std::make_shared<Mailbox<std::shared_ptr<DockerStatsDoc>>>(ex_);
	subs_.push_back(box);
	ensure_pump();
	return box;
}

void StatsHub::unsubscribe(const std::shared_ptr<Mailbox<std::shared_ptr<DockerStatsDoc>>>& sub) {
	subs_.erase(std::remove(subs_.begin(), subs_.end(), sub), subs_.end());
	if (subs_.empty()) {
		stop_all_feeds();
		reconcile_timer_.cancel();
	}
}

unsigned long long StatsHub::size_rw(const std::string& doc_id) {
	for (const auto& [id, size] : sizes_) {
		if (doc_id == id || doc_id.starts_with(id) || id.starts_with(doc_id.substr(0, 12))) {
			return size;
		}
	}
	return 0;
}

void StatsHub::ensure_pump() {
	if (running_) {
		return;
	}
	running_ = true;
	asio::co_spawn(ex_, pump(), asio::detached);
}

void StatsHub::stop_all_feeds() {
	for (auto& state : feeds_) {
		state->stop = true;
		if (state->stream) {
			state->stream->close();
		}
	}
	feeds_.clear();
}

asio::awaitable<void> StatsHub::pump() {
	while (true) {
		if (subs_.empty()) {
			break;
		}

		std::vector<std::string> live_ids;
		bool listed = true;
		try {
			for (const auto& row : co_await ContainerSummary::list(docker_, false)) {
				live_ids.push_back(row.id);
			}
		} catch (const std::exception&) {
			listed = false;
		}

		if (listed) {
			// Reap feeds for containers that are gone or whose read died.
			for (auto it = feeds_.begin(); it != feeds_.end();) {
				const bool keep = !(*it)->stop && std::find(live_ids.begin(), live_ids.end(), (*it)->id) != live_ids.end();
				if (keep) {
					++it;
					continue;
				}
				(*it)->stop = true;
				if ((*it)->stream) {
					(*it)->stream->close();
				}
				it = feeds_.erase(it);
			}
			for (auto it = sizes_.begin(); it != sizes_.end();) {
				if (std::find(live_ids.begin(), live_ids.end(), it->first) == live_ids.end()) {
					it = sizes_.erase(it);
				} else {
					++it;
				}
			}

			// Open feeds for new containers; inspect size_rw once per feed.
			for (const auto& id : live_ids) {
				const bool exists = std::any_of(feeds_.begin(), feeds_.end(),
				                                [&id](const std::shared_ptr<FeedState>& f) { return f->id == id; });
				if (exists) {
					continue;
				}
				unsigned long long size = 0;
				try {
					const auto body = co_await docker_.get(fmt::format("/containers/{}/json?size=1", id));
					const auto doc = nlohmann::json::parse(body, nullptr, false);
					if (!doc.is_discarded() && doc.contains("SizeRw")) {
						size = doc["SizeRw"].get<unsigned long long>();
					}
				} catch (const std::exception&) {
				}
				sizes_[id] = size;

				std::unique_ptr<DockerStream> stream;
				try {
					stream = co_await docker_.open_stream(fmt::format("/containers/{}/stats?stream=true", id));
				} catch (const std::exception&) {
					continue; // daemon hiccup; next reconcile retries
				}
				auto state = std::make_shared<FeedState>();
				state->id = id;
				state->stream = std::shared_ptr<DockerStream>(std::move(stream));
				feeds_.push_back(state);
				asio::co_spawn(ex_, feed(state), asio::detached);
			}
		}

		if (!co_await timer_wait(reconcile_timer_, kReconcileMs)) {
			break;
		}
	}
	stop_all_feeds();
	sizes_.clear();
	running_ = false;
}

asio::awaitable<void> StatsHub::feed(std::shared_ptr<FeedState> state) {
	JsonDocumentScanner scanner;
	while (!state->stop && !subs_.empty()) {
		std::optional<std::string> chunk;
		try {
			chunk = co_await state->stream->read();
		} catch (const std::exception&) {
			break;
		}
		if (!chunk) {
			break;
		}
		for (auto& doc : scanner.push(chunk->data(), chunk->size())) {
			if (doc.is_discarded() || !doc.is_object()) {
				continue;
			}
			auto stats = std::make_shared<DockerStatsDoc>(parse_stats_doc(doc));
			for (auto& sub : subs_) {
				sub->push(stats); // fire-and-forget: live frames only
			}
		}
	}
	state->stop = true;
	state->stream->close();
}

// ------------------------------ EventsHub ------------------------------

EventsHub::~EventsHub() {
	if (stream_) {
		stream_->close();
	}
	idle_timer_.cancel();
}

std::shared_ptr<Mailbox<std::string>> EventsHub::subscribe() {
	auto box = std::make_shared<Mailbox<std::string>>(ex_);
	subs_.push_back(box);
	ensure_pump();
	return box;
}

void EventsHub::unsubscribe(const std::shared_ptr<Mailbox<std::string>>& sub) {
	subs_.erase(std::remove(subs_.begin(), subs_.end(), sub), subs_.end());
	if (subs_.empty()) {
		if (stream_) {
			stream_->close();
		}
		idle_timer_.cancel();
	}
}

void EventsHub::ensure_pump() {
	if (running_) {
		return;
	}
	running_ = true;
	asio::co_spawn(ex_, pump(), asio::detached);
}

asio::awaitable<void> EventsHub::pump() {
	std::string pending;
	while (!subs_.empty()) {
		pending.clear();
		std::unique_ptr<DockerStream> opened;
		try {
			opened = co_await docker_.open_stream("/events?filters=%7B%7D");
		} catch (const std::exception&) {
		}
		if (!opened) {
			if (!co_await timer_wait(idle_timer_, kReconcileMs)) {
				break;
			}
			continue;
		}
		stream_ = std::shared_ptr<DockerStream>(std::move(opened));
		while (!subs_.empty()) {
			std::optional<std::string> chunk;
			try {
				chunk = co_await stream_->read();
			} catch (const std::exception&) {
				break;
			}
			if (!chunk) {
				break;
			}
			pending.append(*chunk);
			if (pending.size() > kLineCarryMax) {
				pending.clear(); // runaway line; drop carry, keep streaming
				continue;
			}
			std::size_t pos;
			while ((pos = pending.find('\n')) != std::string::npos) {
				std::string line = pending.substr(0, pos + 1);
				pending.erase(0, pos + 1);
				for (auto& sub : subs_) {
					sub->push(line);
				}
			}
		}
		stream_->close();
		stream_.reset();
		if (subs_.empty()) {
			break;
		}
		// Daemon closed the events stream: reconnect on the next tick.
		if (!co_await timer_wait(idle_timer_, kReconcileMs)) {
			break;
		}
	}
	running_ = false;
}

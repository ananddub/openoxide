#include "core/tiny_event_engine.h"

#include <grpc/event_engine/event_engine.h>

#include <atomic>
#include <chrono>
#include <condition_variable>
#include <deque>
#include <memory>
#include <mutex>
#include <thread>
#include <unordered_map>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"

#if defined(__linux__)
#include <sys/prctl.h>
#endif

namespace ee = grpc_event_engine::experimental;

namespace {

using Closure = ee::EventEngine::Closure;
using TaskHandle = ee::EventEngine::TaskHandle;

// Timer state shared between the timer thread, the worker queue and Cancel().
struct Entry {
	// kPending: in the deadline heap. kDispatched: in the worker queue.
	// Once running/erased, Cancel() must report false.
	enum class State { kPending, kDispatched, kRunning };
	State state = State::kPending;
	long id = 0;
	std::chrono::steady_clock::time_point deadline{};
	Closure* raw = nullptr; // not owned; caller cleans up on cancel
	absl::AnyInvocable<void()> fn;
};

class TinyEventEngine final : public ee::EventEngine {
public:
	TinyEventEngine() {
		workers_.emplace_back([this] { WorkerLoop(); });
		workers_.emplace_back([this] { WorkerLoop(); });
		timer_ = std::thread([this] { TimerLoop(); });
	}

	~TinyEventEngine() override {
		{
			std::lock_guard<std::mutex> guard(mutex_);
			stopping_ = true;
		}
		cv_.notify_all();
		timer_cv_.notify_all();
		for (auto& worker : workers_) {
			if (worker.joinable()) {
				worker.join();
			}
		}
		if (timer_.joinable()) {
			timer_.join();
		}
	}

	// Networking never reaches the engine: the event-engine transport and DNS
	// experiments are disabled at startup, so classic iomgr owns all sockets.
	absl::StatusOr<std::unique_ptr<Listener>> CreateListener(
		Listener::AcceptCallback, absl::AnyInvocable<void(absl::Status)>, const ee::EndpointConfig&,
		std::unique_ptr<ee::MemoryAllocatorFactory>) override {
		return absl::UnimplementedError("TinyEventEngine: listeners unsupported");
	}

	ConnectionHandle Connect(OnConnectCallback, const ResolvedAddress&, const ee::EndpointConfig&,
	                         ee::MemoryAllocator, Duration) override {
		return ConnectionHandle{0, 0};
	}

	bool CancelConnect(ConnectionHandle) override { return false; }

	bool IsWorkerThread() override { return WorkerFlag(); }

	std::unique_ptr<DNSResolver> GetDNSResolver(const DNSResolver::ResolverOptions&) override {
		return nullptr;
	}

	void Run(Closure* closure) override {
		Queued item;
		item.raw = closure;
		Post(std::move(item));
	}

	void Run(absl::AnyInvocable<void()> closure) override {
		Queued item;
		item.fn = std::move(closure);
		Post(std::move(item));
	}

	TaskHandle RunAfter(Duration when, Closure* closure) override {
		return ScheduleAt(Clock::now() + std::chrono::nanoseconds(when.count()), closure, nullptr);
	}

	TaskHandle RunAfter(Duration when, absl::AnyInvocable<void()> closure) override {
		return ScheduleAt(Clock::now() + std::chrono::nanoseconds(when.count()), nullptr,
		                  std::move(closure));
	}

	bool Cancel(TaskHandle handle) override {
		std::lock_guard<std::mutex> guard(mutex_);
		const auto found = tasks_.find(handle.keys[0]);
		if (found == tasks_.end() || found->second.state != Entry::State::kPending) {
			return false;
		}
		const long id = handle.keys[0];
		tasks_.erase(found);
		RemoveFromHeap(id);
		timer_cv_.notify_all();
		return true;
	}

private:
	using Clock = std::chrono::steady_clock;

	struct Queued {
		long id = 0; // 0 = immediate Run with no registry entry
		Closure* raw = nullptr;
		absl::AnyInvocable<void()> fn;
	};

	void Post(Queued item) {
		std::lock_guard<std::mutex> guard(mutex_);
		queue_.push_back(std::move(item));
		cv_.notify_one();
	}

	// Min-heap by deadline; pointers reference entries in tasks_ (stable:
	// unordered_map nodes don't move on rehash).
	static bool HeapLess(const Entry* a, const Entry* b) {
		if (a->deadline != b->deadline) {
			return a->deadline > b->deadline;
		}
		return a->id > b->id;
	}

	TaskHandle ScheduleAt(Clock::time_point deadline, Closure* raw,
	                          absl::AnyInvocable<void()> fn) {
		std::lock_guard<std::mutex> guard(mutex_);
		const long id = next_id_.fetch_add(1, std::memory_order_relaxed);
		auto [it, inserted] = tasks_.emplace(id, Entry{});
		Entry& entry = it->second;
		entry.state = Entry::State::kPending;
		entry.id = id;
		entry.deadline = deadline;
		entry.raw = raw;
		entry.fn = std::move(fn);
		heap_.push_back(&entry);
		std::push_heap(heap_.begin(), heap_.end(), HeapLess);
		timer_cv_.notify_all();
		return TaskHandle{id, 0};
	}

	// Caller holds mutex_.
	void RemoveFromHeap(long id) {
		for (std::size_t i = 0; i < heap_.size(); ++i) {
			if (heap_[i]->id == id) {
				std::iter_swap(heap_.begin() + static_cast<std::ptrdiff_t>(i), heap_.end() - 1);
				heap_.pop_back();
				std::make_heap(heap_.begin(), heap_.end(), HeapLess);
				return;
			}
		}
	}

	void TimerLoop() {
#if defined(__linux__)
		::prctl(PR_SET_NAME, "tiny-ee-timer", 0, 0, 0);
#endif
		std::unique_lock<std::mutex> lock(mutex_);
		while (!stopping_) {
			if (heap_.empty()) {
				timer_cv_.wait(lock, [&] { return stopping_ || !heap_.empty(); });
				continue;
			}
			const auto next = heap_.front()->deadline;
			timer_cv_.wait_until(lock, next, [&] { return stopping_; });
			const auto now = Clock::now();
			bool posted = false;
			while (!heap_.empty() && heap_.front()->deadline <= now) {
				std::pop_heap(heap_.begin(), heap_.end(), HeapLess);
				Entry* entry = heap_.back();
				heap_.pop_back();
				if (entry->state == Entry::State::kPending) {
					entry->state = Entry::State::kDispatched;
					Queued item;
					item.id = entry->id;
					queue_.push_back(std::move(item));
					posted = true;
				}
			}
			if (posted) {
				cv_.notify_all();
			}
		}
	}

	void WorkerLoop() {
#if defined(__linux__)
		::prctl(PR_SET_NAME, "tiny-ee", 0, 0, 0);
#endif
		WorkerFlag() = true;
		while (true) {
			Queued item;
			{
				std::unique_lock<std::mutex> lock(mutex_);
				cv_.wait(lock, [&] { return stopping_ || !queue_.empty(); });
				if (stopping_ && queue_.empty()) {
					return;
				}
				item = std::move(queue_.front());
				queue_.pop_front();
				if (item.id != 0) {
					const auto found = tasks_.find(item.id);
					if (found == tasks_.end() || found->second.state != Entry::State::kDispatched) {
						continue; // cancelled between dispatch and pickup
					}
					Entry& entry = found->second;
					entry.state = Entry::State::kRunning;
					item.raw = entry.raw;
					item.fn = std::move(entry.fn);
				}
			}
			if (item.raw != nullptr) {
				item.raw->Run();
			} else if (item.fn) {
				std::move(item.fn)();
			}
			if (item.id != 0) {
				std::lock_guard<std::mutex> lock(mutex_);
				tasks_.erase(item.id);
			}
		}
	}

	std::vector<std::thread> workers_;
	std::thread timer_;
	std::mutex mutex_;
	std::condition_variable cv_;
	std::condition_variable timer_cv_;
	std::deque<Queued> queue_;
	std::unordered_map<long, Entry> tasks_;
	std::vector<Entry*> heap_;
	std::atomic<long> next_id_{1};
	bool stopping_ = false;
	static bool& WorkerFlag() {
		thread_local bool flag = false;
		return flag;
	}
};

} // namespace

void InstallTinyEventEngine() {
	ee::SetEventEngineFactory([]() -> std::unique_ptr<ee::EventEngine> {
		return std::make_unique<TinyEventEngine>();
	});
}

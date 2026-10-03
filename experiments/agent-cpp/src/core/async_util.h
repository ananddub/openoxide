#pragma once

#include <grpcpp/grpcpp.h>

#include <asio/any_io_executor.hpp>
#include <asio/awaitable.hpp>
#include <asio/co_spawn.hpp>
#include <asio/detached.hpp>
#include <asio/post.hpp>
#include <asio/steady_timer.hpp>
#include <asio/use_awaitable.hpp>

#include <concurrentqueue/concurrentqueue.h>

#include <atomic>
#include <chrono>
#include <functional>
#include <memory>
#include <mutex>
#include <optional>
#include <string_view>
#include <utility>

// Shared coroutine plumbing: every gRPC callback-API reaction fires on a
// gRPC EventEngine thread, so anything that resumes an Asio coroutine must
// hop back onto the io_context executor first. Everything here is built on
// that rule, and nothing here blocks: waiting is done with cancellable
// steady_timers, never condition_variable, so a handler never pins a thread.

// Fire-and-forget awaitable queue on moodycamel::ConcurrentQueue: items are
// broadcast live and never buffered for late readers — no retained history,
// nothing held per connection beyond one queue block. push()/close() may be
// called from any thread (including gRPC EventEngine threads); next() must be
// awaited on the mailbox executor.
template <typename T>
class Mailbox {
public:
	explicit Mailbox(asio::any_io_executor ex)
	    : ex_(std::move(ex)), queue_(kQueueBlock),
	      timer_(std::make_shared<asio::steady_timer>(ex_)) {}

	void push(T value) {
		if (closed_.load(std::memory_order_relaxed)) {
			return;
		}
		queue_.enqueue(std::move(value));
		Wake();
	}

	void close() {
		closed_.store(true, std::memory_order_release);
		Wake();
	}

	// Resolves with the next item, or nullopt once closed (and drained).
	asio::awaitable<std::optional<T>> next() {
		while (true) {
			if (T value; queue_.try_dequeue(value)) {
				co_return value;
			}
			if (closed_.load(std::memory_order_acquire)) {
				co_return std::nullopt;
			}
			auto timer = timer_;
			timer->expires_after(kIdleBound);
			try {
				co_await timer->async_wait(asio::use_awaitable);
			} catch (const asio::system_error&) {
				// Woken by push/close: loop re-checks the queue.
			}
		}
	}

	bool try_pop(T& value) {
		if (queue_.try_dequeue(value)) {
			return true;
		}
		return false;
	}

	// Resolves with the next item, nullopt on timeout, or nullopt once closed.
	// The caller distinguishes the two by checking closed().
	asio::awaitable<std::optional<T>> next_within(std::chrono::milliseconds timeout) {
		while (true) {
			if (T value; queue_.try_dequeue(value)) {
				co_return value;
			}
			if (closed_.load(std::memory_order_acquire)) {
				co_return std::nullopt;
			}
			auto timer = timer_;
			timer->expires_after(timeout);
			bool timed_out = true;
			try {
				co_await timer->async_wait(asio::use_awaitable);
			} catch (const asio::system_error&) {
				timed_out = false;
			}
			if (timed_out) {
				co_return std::nullopt;
			}
			// Woken by push/close: loop re-checks the queue.
		}
	}

	bool closed() const { return closed_.load(std::memory_order_acquire); }

private:
	// Bounds how long a reader parks with no traffic; pushes wake it sooner.
	static constexpr auto kIdleBound = std::chrono::milliseconds(2000);
	// Small blocks keep the queue's memory footprint down (1 MB target).
	static constexpr int kQueueBlock = 1024;

	void Wake() {
		auto timer = timer_;
		asio::post(ex_, [timer] { timer->cancel(); });
	}

	asio::any_io_executor ex_;
	moodycamel::ConcurrentQueue<T> queue_;
	std::atomic<bool> closed_{false};
	std::shared_ptr<asio::steady_timer> timer_;
};

// Cancellation view of one RPC. gRPC calls OnCancel on an EventEngine thread;
// handlers poll the flag between awaits (mailbox wakeups bound the latency).
class CallPilot {
public:
	explicit CallPilot(asio::any_io_executor ex) : ex_(std::move(ex)) {}

	void Cancel() { cancelled_ = true; }
	bool cancelled() const { return cancelled_.load(std::memory_order_relaxed); }
	asio::any_io_executor executor() const { return ex_; }

private:
	asio::any_io_executor ex_;
	std::atomic<bool> cancelled_{false};
};

// Server-write reactor driven by a coroutine: write() starts one message and
// suspends until OnWriteDone, so streams never hold a thread between frames.
template <typename Msg>
class CoroutineWriteReactor : public grpc::ServerWriteReactor<Msg> {
public:
	explicit CoroutineWriteReactor(std::shared_ptr<CallPilot> pilot)
	    : pilot_(std::move(pilot)) {}

	// Awaited on the io thread. Returns false when the write failed or the
	// call was cancelled; the caller then Finishes with its chosen status.
	asio::awaitable<bool> write(Msg message) { return write_one(std::move(message)); }

	void finish(grpc::Status status) { this->Finish(std::move(status)); }

	bool cancelled() const { return pilot_->cancelled(); }

protected:
	asio::awaitable<bool> write_one(Msg message) {
		msg_ = std::move(message);
		// A default-constructed steady_timer counts as already-expired in
		// this Asio version, so arm it at +infinity: the wait must hang until
		// OnWriteDone posts the cancel.
		auto timer = std::make_shared<asio::steady_timer>(pilot_->executor(),
		                                                  asio::steady_timer::time_point::max());
		{
			std::lock_guard<std::mutex> guard(mutex_);
			write_timer_ = timer;
			write_done_ = false;
			write_ok_ = false;
		}
		this->StartWrite(&msg_);
		try {
			co_await timer->async_wait(asio::use_awaitable);
		} catch (const asio::system_error&) {
		}
		std::lock_guard<std::mutex> guard(mutex_);
		write_timer_.reset();
		co_return write_done_ && write_ok_ && !pilot_->cancelled();
	}

protected:
	void OnWriteDone(bool ok) override {
		std::shared_ptr<asio::steady_timer> timer;
		{
			std::lock_guard<std::mutex> guard(mutex_);
			write_done_ = true;
			write_ok_ = ok;
			timer = write_timer_;
		}
		WakeWith(std::move(timer));
	}

	// A cancelled call never completes an in-flight StartWrite, so OnCancel
	// must wake the write handshake itself or the handler coroutine parks on
	// the +infinity timer and the RPC never Finishes (hangs server shutdown).
	void OnCancel() override {
		pilot_->Cancel();
		std::shared_ptr<asio::steady_timer> timer;
		{
			std::lock_guard<std::mutex> guard(mutex_);
			timer = write_timer_;
		}
		WakeWith(std::move(timer));
	}

	// gRPC owns the reactor until OnDone; releasing here matches that
	// contract — handlers must not touch it after Finish().
	void OnDone() override { delete this; }

	void WakeWith(std::shared_ptr<asio::steady_timer> timer) {
		if (timer) {
			auto ex = pilot_->executor();
			asio::post(ex, [timer] { timer->cancel(); });
		}
	}

	const std::shared_ptr<CallPilot>& pilot() const { return pilot_; }

private:
	std::shared_ptr<CallPilot> pilot_;
	Msg msg_;
	std::mutex mutex_;
	std::shared_ptr<asio::steady_timer> write_timer_;
	bool write_done_ = false;
	bool write_ok_ = false;
};

// Client-streaming reactor feeding a mailbox: reads are re-armed from
// OnReadDone on the EventEngine thread, the consuming coroutine never blocks.
// Messages travel as shared_ptr so large payloads (upload chunks) are never
// copied through the queue.
template <typename Msg>
class CoroutineReadReactor : public grpc::ServerReadReactor<Msg> {
public:
	using Box = Mailbox<std::shared_ptr<Msg>>;

	CoroutineReadReactor(std::shared_ptr<CallPilot> pilot, std::shared_ptr<Box> box)
	    : pilot_(std::move(pilot)), box_(std::move(box)) {
		this->StartRead(&msg_);
	}

	void finish(grpc::Status status) { this->Finish(std::move(status)); }

protected:
	void OnReadDone(bool ok) override {
		if (ok) {
			box_->push(std::make_shared<Msg>(std::move(msg_)));
			this->StartRead(&msg_);
			return;
		}
		box_->close();
	}

	void OnCancel() override {
		pilot_->Cancel();
		box_->close();
	}

	void OnDone() override { delete this; }

private:
	std::shared_ptr<CallPilot> pilot_;
	std::shared_ptr<Box> box_;
	Msg msg_;
};

// Bidi reactor driven by coroutines on both sides: inbound messages land in a
// mailbox (never blocks the EventEngine), outbound frames are written from a
// coroutine like CoroutineWriteReactor. The optional cancel hook runs on an
// EventEngine thread and is the shell's chance to post teardown onto the io
// thread; the mailbox close alone would leave a pty wait parked forever.
template <typename Req, typename Resp>
class CoroutineBidiReactor : public grpc::ServerBidiReactor<Req, Resp> {
public:
	using Box = Mailbox<std::shared_ptr<Req>>;

	CoroutineBidiReactor(std::shared_ptr<CallPilot> pilot, std::shared_ptr<Box> box,
	                     std::function<void()> on_cancel)
	    : pilot_(std::move(pilot)), box_(std::move(box)), on_cancel_(std::move(on_cancel)) {
		this->StartRead(&msg_);
	}

	asio::awaitable<bool> write(Resp message) {
		out_ = std::move(message);
		auto timer = std::make_shared<asio::steady_timer>(pilot_->executor(),
		                                                  asio::steady_timer::time_point::max());
		{
			std::lock_guard<std::mutex> guard(mutex_);
			write_timer_ = timer;
			write_done_ = false;
			write_ok_ = false;
		}
		this->StartWrite(&out_);
		try {
			co_await timer->async_wait(asio::use_awaitable);
		} catch (const asio::system_error&) {
		}
		std::lock_guard<std::mutex> guard(mutex_);
		write_timer_.reset();
		co_return write_done_ && write_ok_ && !pilot_->cancelled();
	}

	void finish(grpc::Status status) { this->Finish(std::move(status)); }

	bool cancelled() const { return pilot_->cancelled(); }

	std::shared_ptr<Box>& inbox() { return box_; }

protected:
	void OnReadDone(bool ok) override {
		if (ok) {
			box_->push(std::make_shared<Req>(std::move(msg_)));
			this->StartRead(&msg_);
			return;
		}
		box_->close();
	}

	void OnWriteDone(bool ok) override {
		std::shared_ptr<asio::steady_timer> timer;
		{
			std::lock_guard<std::mutex> guard(mutex_);
			write_done_ = true;
			write_ok_ = ok;
			timer = write_timer_;
		}
		if (timer) {
			auto ex = pilot_->executor();
			asio::post(ex, [timer] { timer->cancel(); });
		}
	}

	void OnCancel() override {
		pilot_->Cancel();
		box_->close();
		// Cancelled calls never complete an in-flight StartWrite: wake the
		// write handshake so the coroutine can Finish and unblock shutdown.
		std::shared_ptr<asio::steady_timer> timer;
		{
			std::lock_guard<std::mutex> guard(mutex_);
			timer = write_timer_;
		}
		if (timer) {
			auto ex = pilot_->executor();
			asio::post(ex, [timer] { timer->cancel(); });
		}
		if (on_cancel_) {
			on_cancel_();
		}
	}

	void OnDone() override { delete this; }

private:
	std::shared_ptr<CallPilot> pilot_;
	std::shared_ptr<Box> box_;
	std::function<void()> on_cancel_;
	Req msg_;
	Resp out_;
	std::mutex mutex_;
	std::shared_ptr<asio::steady_timer> write_timer_;
	bool write_done_ = false;
	bool write_ok_ = false;
};

// x-metrics-token auth shared by services that own their reactors
// (core/middleware.rs parity).
inline grpc::Status authenticate_token(const grpc::CallbackServerContext* context,
                                       const std::string& token) {
	const auto metadata = context->client_metadata();
	const auto found = metadata.find("x-metrics-token");
	if (found != metadata.end() &&
	    std::string_view(found->second.data(), found->second.size()) == token) {
		return grpc::Status::OK;
	}
	return grpc::Status(grpc::StatusCode::UNAUTHENTICATED, "invalid agent token");
}

// Unary callback call: the handler coroutine computes the final status and
// the reactor is finished from the io thread — no thread is held meanwhile.
template <typename Handler>
grpc::ServerUnaryReactor* start_unary_call(asio::any_io_executor ex, grpc::CallbackServerContext* context, Handler handler) {
	auto* reactor = context->DefaultReactor();
	asio::co_spawn(ex, [reactor, handler = std::move(handler)]() -> asio::awaitable<void> {
		grpc::Status status = grpc::Status::OK;
		try {
			status = co_await handler();
		} catch (const std::exception& error) {
			status = grpc::Status(grpc::StatusCode::INTERNAL, error.what());
		}
		reactor->Finish(status);
	}, asio::detached);
	return reactor;
}

// Server-streaming callback call over a CoroutineWriteReactor: the handler
// coroutine writes frames and returns the final status.
template <typename Msg, typename Handler>
grpc::ServerWriteReactor<Msg>* start_write_stream(asio::any_io_executor ex, Handler handler) {
	auto pilot = std::make_shared<CallPilot>(ex);
	auto* reactor = new CoroutineWriteReactor<Msg>(std::move(pilot));
	asio::co_spawn(ex, [reactor, handler = std::move(handler)]() -> asio::awaitable<void> {
		grpc::Status status = grpc::Status::OK;
		try {
			status = co_await handler(reactor);
		} catch (const std::exception& error) {
			status = grpc::Status(grpc::StatusCode::INTERNAL, error.what());
		}
		reactor->finish(status);
	}, asio::detached);
	return reactor;
}

// Immediate-fail reactors: a handler that rejects an RPC (bad token, wrong
// server_id) must still return a reactor from the gRPC thread, but Finish may
// only be called after Start* has been armed — for write/bidi/read reactors
// that means deferring the Finish onto the io thread.

template <typename Msg>
class WriteFailureReactor : public grpc::ServerWriteReactor<Msg> {
public:
	WriteFailureReactor() = default;

	void fail(grpc::Status status) { this->Finish(std::move(status)); }

protected:
	void OnDone() override { delete this; }
};

template <typename Req, typename Resp>
class BidiFailureReactor : public grpc::ServerBidiReactor<Req, Resp> {
public:
	BidiFailureReactor() = default;

	void fail(grpc::Status status) { this->Finish(std::move(status)); }

protected:
	void OnDone() override { delete this; }
};

template <typename Req>
class ReadFailureReactor : public grpc::ServerReadReactor<Req> {
public:
	ReadFailureReactor() = default;

	void fail(grpc::Status status) { this->Finish(std::move(status)); }

protected:
	void OnDone() override { delete this; }
};

// Unary rejection can Finish synchronously: the default reactor arms nothing.
inline grpc::ServerUnaryReactor* start_failed_unary_call(grpc::CallbackServerContext* context,
                                                          const grpc::Status& status) {
	auto* reactor = context->DefaultReactor();
	reactor->Finish(status);
	return reactor;
}

template <typename Msg>
grpc::ServerWriteReactor<Msg>* start_failed_write_call(asio::any_io_executor ex,
                                                       const grpc::Status& status) {
	auto* reactor = new WriteFailureReactor<Msg>();
	asio::post(ex, [reactor, status] { reactor->fail(status); });
	return reactor;
}

template <typename Req, typename Resp>
grpc::ServerBidiReactor<Req, Resp>* start_failed_bidi_call(asio::any_io_executor ex,
                                                            const grpc::Status& status) {
	auto* reactor = new BidiFailureReactor<Req, Resp>();
	asio::post(ex, [reactor, status] { reactor->fail(status); });
	return reactor;
}

template <typename Req>
grpc::ServerReadReactor<Req>* start_failed_read_call(asio::any_io_executor ex,
                                                      const grpc::Status& status) {
	auto* reactor = new ReadFailureReactor<Req>();
	asio::post(ex, [reactor, status] { reactor->fail(status); });
	return reactor;
}

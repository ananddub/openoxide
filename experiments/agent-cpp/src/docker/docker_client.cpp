#include "docker/docker_client.h"

#include <asio/post.hpp>
#include <asio/posix/stream_descriptor.hpp>
#include <asio/steady_timer.hpp>
#include <asio/use_awaitable.hpp>

#include <curl/curl.h>

#include <algorithm>
#include <atomic>
#include <chrono>
#include <fmt/format.h>
#include <functional>
#include <mutex>
#include <stdexcept>
#include <string_view>
#include <unordered_map>
#include <unistd.h>
#include <vector>

#include "core/async_util.h"

namespace {

const char* kApiVersion = "v1.41";
constexpr size_t kMaxResponseBytes = 32 * 1024 * 1024;

std::string uri_for(const std::string& path) {
	return fmt::format("/{}{}", kApiVersion, path);
}

} // namespace

namespace detail {

struct Pump;
struct Transfer;

struct Outcome {
	long status = 0;
	CURLcode result = CURLE_OK;
	std::string body;
};

// The shared state of one open stream: the frame mailbox the reader awaits,
// plus the transfer an abort has to tear down.
struct StreamCore : std::enable_shared_from_this<StreamCore> {
	asio::any_io_executor ex;
	std::shared_ptr<Pump> pump;
	std::shared_ptr<Transfer> transfer;
	std::shared_ptr<Mailbox<std::string>> frames;
	std::shared_ptr<Mailbox<Outcome>> opened;
	std::atomic<bool> aborted{false};

	void abort();
};

// One curl_easy transfer plus everything needed to deliver it. Everything
// here is touched only on the io thread (curl calls its callbacks from the
// multi actions the Pump runs there), except the mailboxes, which are
// thread-safe by design.
struct Transfer {
	explicit Transfer(Pump& owner) : pump(owner) {}

	Pump& pump;
	CURL* easy = nullptr;
	std::string uri; // "/v1.41/..." — used in error messages
	std::string url;
	std::string post_body;             // curl does not copy CURLOPT_POSTFIELDS
	struct curl_slist* headers = nullptr;
	bool streaming = false;
	bool dead = false; // completion already delivered
	bool status_seen = false;
	long status = 0;
	std::string buffer; // one-shot body accumulation
	std::function<void(detail::Outcome)> oneshot_done;
	std::shared_ptr<detail::StreamCore> stream; // streaming sink
};

// A curl socket wired into Asio: two dup()ed descriptors, one waited for
// readability and one for writability, so both directions can have a
// pending completion at the same time. curl owns the original fd; the dups
// close with the descriptors. A slot can outlive its waits by one io-loop
// pass (the cancel handler holds it), so it only weakly references the Pump.
struct SocketSlot : std::enable_shared_from_this<SocketSlot> {
	SocketSlot(Pump& owner, curl_socket_t socket, std::shared_ptr<Transfer> session);

	std::weak_ptr<Pump> pump;
	curl_socket_t fd;
	std::shared_ptr<Transfer> transfer;
	asio::posix::stream_descriptor in;
	asio::posix::stream_descriptor out;
	bool want_in = true;
	bool want_out = true;
	bool waiting_in = false;
	bool waiting_out = false;
	bool detached = false; // poll removed or curl closed: waiters no-op

	void arm();
	void arm_in();
	void arm_out();
	void detach();
};

// The single-threaded curl_multi engine: curl hands out socket/timer events,
// Asio waits on them, and completion handlers resume the agent coroutines.
struct Pump : std::enable_shared_from_this<Pump> {
	Pump(asio::any_io_executor ex, std::string socket_path);
	~Pump();

	// Must run on the io thread (all callers are coroutines on it).
	std::shared_ptr<Transfer> begin(const std::string& method, const std::string& path,
	                                const std::string& body,
	                                std::function<void(detail::Outcome)> done,
	                                std::shared_ptr<detail::StreamCore> stream);
	void kill(const std::shared_ptr<Transfer>& transfer);

	void slot_ready(SocketSlot& slot, bool readable);
	void socket_event(CURL* easy, curl_socket_t fd, int what, void* socketp);
	void timer_event(long timeout_ms);

	asio::any_io_executor executor;
	std::string socket_path;
	CURLM* multi = nullptr;
	asio::steady_timer timer;
	std::unordered_map<CURL*, std::shared_ptr<Transfer>> transfers;

	static int socket_cb(CURL* easy, curl_socket_t fd, int what, void* clientp, void* socketp);
	static int timer_cb(CURLM* multi, long timeout_ms, void* clientp);
	static size_t write_cb(char* data, size_t size, size_t count, void* userp);
	static size_t header_cb(char* data, size_t size, size_t count, void* userp);

	void start();
	void process();
	void finish(const std::shared_ptr<Transfer>& transfer, CURLcode result);
	void deliver_bytes(Transfer& transfer, std::string_view bytes);
};

void Pump::slot_ready(SocketSlot& slot, bool readable) {
	if (slot.detached || slot.transfer->dead) {
		return;
	}
	int select_bits = 0;
	if (readable && slot.want_in) {
		select_bits = CURL_CSELECT_IN;
	} else if (!readable && slot.want_out) {
		select_bits = CURL_CSELECT_OUT;
	}
	if (select_bits != 0) {
		int running = 0;
		curl_multi_socket_action(multi, slot.fd, select_bits, &running);
		process();
	}
	if (!slot.detached && !slot.transfer->dead) {
		if (readable && slot.want_in) {
			slot.arm_in();
		} else if (!readable && slot.want_out) {
			slot.arm_out();
		}
	}
}

void Pump::socket_event(CURL* easy, curl_socket_t fd, int what, void* socketp) {
	if (socketp == nullptr) { // CURL_SOCKOPT_NEW: first sight of the socket
		auto it = transfers.find(easy);
		if (it == transfers.end()) {
			return;
		}
		auto slot = std::make_shared<SocketSlot>(*this, fd, it->second);
		curl_multi_assign(multi, fd, slot.get());
		slot->arm();
		return;
	}
	auto& slot = *static_cast<SocketSlot*>(socketp);
	// curl 7.87 calls this CURL_SOCKOPT_CLEANUP (later releases renamed it
	// CURL_SOCKOPT_CLOSED); it is the last callback for the socket.
#ifndef CURL_SOCKOPT_CLEANUP
#define CURL_SOCKOPT_CLEANUP 4
#endif
	if (what == CURL_POLL_REMOVE || what == CURL_SOCKOPT_CLEANUP) {
		slot.detach();
		return;
	}
	switch (what) {
	case CURL_POLL_IN:
		slot.want_in = true;
		slot.want_out = false;
		break;
	case CURL_POLL_OUT:
		slot.want_out = true;
		slot.want_in = false;
		break;
	case CURL_POLL_INOUT:
		slot.want_in = true;
		slot.want_out = true;
		break;
	}
	slot.arm();
}

void Pump::timer_event(long timeout_ms) {
	if (timeout_ms < 0) {
		timer.cancel();
		return;
	}
	timer.expires_after(std::chrono::milliseconds(timeout_ms));
	std::weak_ptr<Pump> weak = shared_from_this();
	timer.async_wait([weak](const asio::error_code& ec) {
		if (ec) {
			return;
		}
		if (auto pump = weak.lock()) {
			int running = 0;
			curl_multi_socket_action(pump->multi, CURL_SOCKET_TIMEOUT, 0, &running);
			pump->process();
		}
	});
}

int Pump::socket_cb(CURL* easy, curl_socket_t fd, int what, void* clientp, void* socketp) {
	static_cast<Pump*>(clientp)->socket_event(easy, fd, what, socketp);
	return 0;
}

int Pump::timer_cb(CURLM*, long timeout_ms, void* clientp) {
	static_cast<Pump*>(clientp)->timer_event(timeout_ms);
	return 0;
}

size_t Pump::write_cb(char* data, size_t size, size_t count, void* userp) {
	auto& transfer = *static_cast<Transfer*>(userp);
	const size_t bytes = size * count;
	transfer.pump.deliver_bytes(transfer, std::string_view(data, bytes));
	// Returning short of the full count aborts with CURLE_WRITE_ERROR, which
	// is how the one-shot response cap (and stream aborts) tear down.
	return transfer.dead ? 0 : bytes;
}

size_t Pump::header_cb(char*, size_t size, size_t count, void* userp) {
	auto& transfer = *static_cast<Transfer*>(userp);
	if (!transfer.status_seen) {
		transfer.status_seen = true;
		long code = 0;
		curl_easy_getinfo(transfer.easy, CURLINFO_RESPONSE_CODE, &code);
		transfer.status = code;
		if (transfer.stream) {
			// Wake open_stream as soon as the status line is known.
			detail::Outcome outcome;
			outcome.status = code;
			transfer.stream->opened->push(outcome);
			transfer.stream->opened->close();
		}
	}
	return size * count;
}

void Pump::deliver_bytes(Transfer& transfer, std::string_view bytes) {
	if (transfer.streaming) {
		if (transfer.stream && transfer.status >= 200 && transfer.status < 300) {
			transfer.stream->frames->push(std::string(bytes));
		}
		return;
	}
	if (transfer.buffer.size() + bytes.size() > kMaxResponseBytes) {
		transfer.dead = true; // makes write_cb return short -> CURLE_WRITE_ERROR
		return;
	}
	transfer.buffer.append(bytes);
}

std::shared_ptr<Transfer> Pump::begin(const std::string& method, const std::string& path,
                                      const std::string& body,
                                      std::function<void(detail::Outcome)> done,
                                      std::shared_ptr<detail::StreamCore> stream) {
	auto transfer = std::make_shared<Transfer>(*this);
	transfer->uri = uri_for(path);
	transfer->url = "http://localhost" + transfer->uri;
	transfer->streaming = (stream != nullptr);
	transfer->oneshot_done = std::move(done);
	transfer->stream = std::move(stream);
	transfer->easy = curl_easy_init();
	if (!transfer->easy) {
		throw std::runtime_error("docker: curl_easy_init failed");
	}
	curl_easy_setopt(transfer->easy, CURLOPT_URL, transfer->url.c_str());
	curl_easy_setopt(transfer->easy, CURLOPT_UNIX_SOCKET_PATH, socket_path.c_str());
	curl_easy_setopt(transfer->easy, CURLOPT_NOSIGNAL, 1L);
	curl_easy_setopt(transfer->easy, CURLOPT_PRIVATE, transfer.get());
	curl_easy_setopt(transfer->easy, CURLOPT_WRITEFUNCTION, &Pump::write_cb);
	curl_easy_setopt(transfer->easy, CURLOPT_WRITEDATA, transfer.get());
	curl_easy_setopt(transfer->easy, CURLOPT_HEADERFUNCTION, &Pump::header_cb);
	curl_easy_setopt(transfer->easy, CURLOPT_HEADERDATA, transfer.get());
	if (method == "POST") {
		transfer->post_body = body;
		curl_easy_setopt(transfer->easy, CURLOPT_POST, 1L);
		curl_easy_setopt(transfer->easy, CURLOPT_POSTFIELDS, transfer->post_body.c_str());
		curl_easy_setopt(transfer->easy, CURLOPT_POSTFIELDSIZE_LARGE,
		                 static_cast<curl_off_t>(transfer->post_body.size()));
		transfer->headers = curl_slist_append(transfer->headers, "Content-Type: application/json");
		curl_easy_setopt(transfer->easy, CURLOPT_HTTPHEADER, transfer->headers);
	}
	transfers.emplace(transfer->easy, transfer);
	curl_multi_add_handle(multi, transfer->easy);
	start();
	return transfer;
}

void Pump::start() {
	// Any pending timeout keeps the loop alive; this kick advances fresh
	// handles and (re)arms the timer through the socket/timer callbacks.
	int running = 0;
	curl_multi_socket_action(multi, CURL_SOCKET_TIMEOUT, 0, &running);
	process();
}

void Pump::process() {
	int messages = 0;
	while (CURLMsg* message = curl_multi_info_read(multi, &messages)) {
		if (message->msg != CURLMSG_DONE) {
			continue;
		}
		auto it = transfers.find(message->easy_handle);
		if (it == transfers.end()) {
			continue;
		}
		finish(it->second, message->data.result);
	}
}

void Pump::finish(const std::shared_ptr<Transfer>& transfer, CURLcode result) {
	const bool already_dead = transfer->dead; // one-shot size cap aborted it
	transfer->dead = true;
	if (!already_dead || transfer->status == 0) {
		long code = 0;
		curl_easy_getinfo(transfer->easy, CURLINFO_RESPONSE_CODE, &code);
		transfer->status = code;
	}
	if (transfer->stream) {
		detail::Outcome outcome;
		outcome.status = transfer->status;
		outcome.result = result;
		transfer->stream->opened->push(outcome);
		transfer->stream->opened->close();
		transfer->stream->frames->close();
	} else if (transfer->oneshot_done) {
		detail::Outcome outcome;
		outcome.status = transfer->status;
		outcome.result = result;
		outcome.body = std::move(transfer->buffer);
		auto done = std::move(transfer->oneshot_done);
		transfer->oneshot_done = nullptr;
		done(std::move(outcome));
	}
	// Ordering: getinfo first, then detach the socket callbacks, then free.
	if (auto it = transfers.find(transfer->easy); it != transfers.end()) {
		transfers.erase(it);
	}
	curl_multi_remove_handle(multi, transfer->easy);
	if (transfer->headers) {
		curl_slist_free_all(transfer->headers);
		transfer->headers = nullptr;
	}
	curl_easy_cleanup(transfer->easy);
	transfer->easy = nullptr;
}

void Pump::kill(const std::shared_ptr<Transfer>& transfer) {
	if (transfer->dead || !transfers.contains(transfer->easy)) {
		return; // completion already delivered (or abort raced ahead of it)
	}
	finish(transfer, CURLE_ABORTED_BY_CALLBACK);
}

Pump::Pump(asio::any_io_executor ex, std::string path)
    : executor(ex), socket_path(std::move(path)), timer(std::move(ex)) {
	static std::once_flag global_init;
	std::call_once(global_init, [] { curl_global_init(CURL_GLOBAL_DEFAULT); });
	multi = curl_multi_init();
	if (!multi) {
		throw std::runtime_error("docker: curl_multi_init failed");
	}
	curl_multi_setopt(multi, CURLMOPT_SOCKETFUNCTION, &Pump::socket_cb);
	curl_multi_setopt(multi, CURLMOPT_SOCKETDATA, this);
	curl_multi_setopt(multi, CURLMOPT_TIMERFUNCTION, &Pump::timer_cb);
	curl_multi_setopt(multi, CURLMOPT_TIMERDATA, this);
}

Pump::~Pump() {
	for (auto& [easy, transfer] : transfers) {
		(void)easy;
		transfer->dead = true;
		curl_multi_remove_handle(multi, transfer->easy);
		curl_easy_cleanup(transfer->easy);
		transfer->easy = nullptr;
	}
	transfers.clear();
	curl_multi_cleanup(multi);
}

SocketSlot::SocketSlot(Pump& owner, curl_socket_t socket, std::shared_ptr<Transfer> session)
    : pump(owner.shared_from_this()), fd(socket), transfer(std::move(session)),
      in(owner.executor, ::dup(socket)), out(owner.executor, ::dup(socket)) {}

void SocketSlot::arm() {
	if (detached) {
		return;
	}
	if (want_in && !waiting_in) {
		arm_in();
	}
	if (want_out && !waiting_out) {
		arm_out();
	}
}

void SocketSlot::arm_in() {
	waiting_in = true;
	in.async_wait(asio::posix::stream_descriptor::wait_read,
	              [this, self = shared_from_this()](const asio::error_code&) {
		              self->waiting_in = false;
		              if (auto pump = self->pump.lock()) {
			              pump->slot_ready(*self, true);
		              }
	              });
}

void SocketSlot::arm_out() {
	waiting_out = true;
	out.async_wait(asio::posix::stream_descriptor::wait_write,
	               [this, self = shared_from_this()](const asio::error_code&) {
		               self->waiting_out = false;
		               if (auto pump = self->pump.lock()) {
			               pump->slot_ready(*self, false);
		               }
	               });
}

void SocketSlot::detach() {
	detached = true;
	in.cancel();
	out.cancel();
}

void StreamCore::abort() {
	if (aborted.exchange(true)) {
		return;
	}
	frames->close();
	opened->close();
	asio::post(ex, [self = shared_from_this()] { self->pump->kill(self->transfer); });
}

} // namespace detail

DockerClient::DockerClient(asio::any_io_executor executor, std::string socket_path)
    : executor_(executor),
      pump_(std::make_shared<detail::Pump>(std::move(executor), std::move(socket_path))) {}

asio::awaitable<std::string> DockerClient::perform(const std::string& method,
                                                   const std::string& path,
                                                   const std::string& body) {
	auto box = std::make_shared<Mailbox<detail::Outcome>>(executor_);
	pump_->begin(method, path, body, [box](detail::Outcome outcome) {
		box->push(std::move(outcome));
	}, nullptr);
	auto outcome = co_await box->next();
	if (!outcome) {
		throw std::runtime_error(fmt::format("docker {}: call interrupted", uri_for(path)));
	}
	if (outcome->result != CURLE_OK) {
		if (outcome->result == CURLE_WRITE_ERROR) {
			throw std::runtime_error("docker response too large");
		}
		throw std::runtime_error(
		    fmt::format("docker socket call to {} failed: {}", uri_for(path),
		                curl_easy_strerror(outcome->result)));
	}
	if (outcome->status < 200 || outcome->status >= 300) {
		throw std::runtime_error(fmt::format("docker {}: status {} {}", uri_for(path),
		                                     outcome->status, outcome->body));
	}
	co_return std::move(outcome->body);
}

asio::awaitable<std::string> DockerClient::get(const std::string& path) {
	co_return co_await perform("GET", path, "");
}

asio::awaitable<std::string> DockerClient::post(const std::string& path,
                                                const std::string& json_body) {
	co_return co_await perform("POST", path, json_body);
}

asio::awaitable<std::unique_ptr<DockerStream>> DockerClient::open_stream(const std::string& path) {
	auto core = std::make_shared<detail::StreamCore>();
	core->ex = executor_;
	core->pump = pump_;
	core->frames = std::make_shared<Mailbox<std::string>>(executor_);
	core->opened = std::make_shared<Mailbox<detail::Outcome>>(executor_);
	core->transfer = pump_->begin("GET", path, "", nullptr, core);
	auto outcome = co_await core->opened->next();
	if (!outcome) {
		throw std::runtime_error(
		    fmt::format("docker stream {} failed: connection closed before headers", uri_for(path)));
	}
	if (outcome->result != CURLE_OK && outcome->status == 0) {
		throw std::runtime_error(
		    fmt::format("docker socket call to {} failed: {}", uri_for(path),
		                curl_easy_strerror(outcome->result)));
	}
	if (outcome->status < 200 || outcome->status >= 300) {
		throw std::runtime_error(fmt::format("docker {}: status {}", uri_for(path), outcome->status));
	}
	co_return std::unique_ptr<DockerStream>(new DockerStream(std::move(core)));
}

DockerStream::DockerStream(std::shared_ptr<detail::StreamCore> core) : core_(std::move(core)) {}

DockerStream::~DockerStream() {
	close();
}

void DockerStream::close() {
	core_->abort();
}

asio::awaitable<std::optional<std::string>> DockerStream::read() {
	std::string frame;
	if (core_->frames->try_pop(frame)) {
		co_return frame;
	}
	if (core_->frames->closed()) {
		co_return std::nullopt;
	}
	co_return co_await core_->frames->next();
}

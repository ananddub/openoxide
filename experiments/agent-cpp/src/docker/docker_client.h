#pragma once

#include <asio/any_io_executor.hpp>
#include <asio/awaitable.hpp>

#include <memory>
#include <optional>
#include <string>

namespace detail {
struct Pump;       // curl_multi multiplexer wired into the Asio executor
struct StreamCore; // transfer state shared with one open DockerStream
struct Outcome;    // status + transport error + body of a finished transfer
}

// HTTP client over the Docker Engine's unix socket, built on libcurl — the
// C++ counterpart of agent/src/utils/docker/transport/api.rs. curl speaks
// HTTP/1.1 (framing, chunked decoding, status parsing); the curl_multi
// sockets and timers are driven from the agent's single Asio io_context, so
// there are no extra threads and nothing pins a thread while it waits on
// the daemon.
class DockerClient {
public:
	DockerClient(asio::any_io_executor executor, std::string socket_path);

	asio::any_io_executor executor() const { return executor_; }

	// GET returning the full body; throws std::runtime_error on transport
	// failure or non-2xx status (message carries the status + detail).
	asio::awaitable<std::string> get(const std::string& path);

	// POST with a JSON body, returning the response body.
	asio::awaitable<std::string> post(const std::string& path, const std::string& json_body);

	// Open a long-lived GET stream (/stats, /events); caller reads frames.
	// Throws on connect failure or non-2xx before returning.
	asio::awaitable<std::unique_ptr<class DockerStream>> open_stream(const std::string& path);

private:
	asio::awaitable<std::string> perform(const std::string& method, const std::string& path,
	                                     const std::string& body);

	asio::any_io_executor executor_;
	std::shared_ptr<detail::Pump> pump_;
};

// A live response body from DockerClient::open_stream. close() from any
// thread aborts any pending read, which is how cancelled RPCs tear a feed
// down.
class DockerStream {
public:
	~DockerStream();
	DockerStream(const DockerStream&) = delete;
	DockerStream& operator=(const DockerStream&) = delete;

	// One decoded body chunk, or nullopt at EOF/error/abort.
	asio::awaitable<std::optional<std::string>> read();

	void close();

private:
	friend class DockerClient;
	explicit DockerStream(std::shared_ptr<detail::StreamCore> core);

	std::shared_ptr<detail::StreamCore> core_;
};

#pragma once

#include <asio/awaitable.hpp>
#include <asio/local/stream_protocol.hpp>
#include <memory>
#include <optional>
#include <string>

// Async HTTP/1.1 client over the Docker Engine's unix socket — the C++
// counterpart of agent/src/utils/docker/transport/api.rs. Every operation is
// an Asio coroutine: one-shot GETs/POSTs use a fresh connection per call,
// long-lived feeds (/stats, /events) own a dedicated stream, and nothing
// pins a thread while it waits on the daemon.
class DockerClient {
public:
	DockerClient(asio::any_io_executor executor, std::string socket_path);

	asio::any_io_executor executor() const { return executor_; }

	// GET returning the full body; throws std::runtime_error on connect,
	// transport or non-2xx status (message carries the status + detail).
	asio::awaitable<std::string> get(const std::string& path);

	// POST with a JSON body, returning the response body.
	asio::awaitable<std::string> post(const std::string& path, const std::string& json_body);

	// Open a long-lived GET stream (/stats, /events); caller reads frames.
	asio::awaitable<std::unique_ptr<class DockerStream>> open_stream(const std::string& path);

private:
	asio::any_io_executor executor_;
	std::string socket_path_;
};

// A live connection after the response headers. close() from the io thread
// aborts any pending read, which is how cancelled RPCs tear a feed down.
class DockerStream {
public:
	DockerStream(asio::local::stream_protocol::socket socket, bool chunked, std::string initial_carry);
	~DockerStream();
	DockerStream(const DockerStream&) = delete;
	DockerStream& operator=(const DockerStream&) = delete;

	// Decoded body bytes (>0), or nullopt at EOF/error/connection closed.
	asio::awaitable<std::optional<std::string>> read();

	void close();

private:
	friend class DockerClient;

	// Pull raw socket bytes into carry_; false on EOF/error.
	asio::awaitable<bool> pull();

	asio::local::stream_protocol::socket socket_;
	bool chunked_;
	std::string carry_; // received bytes not yet handed out
	bool eof_ = false;

	// Chunked framing state.
	enum class ChunkState { Size, Data, DataCrlf, Done } chunk_state_ = ChunkState::Size;
	unsigned long long chunk_remaining_ = 0;
};

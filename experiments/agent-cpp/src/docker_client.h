#pragma once
#include <cstdint>
#include <memory>
#include <string>

// Blocking HTTP/1.1 client over the Docker Engine's unix socket — the C++
// counterpart of agent/src/utils/docker/transport/api.rs. One-shot GETs
// (get_json) use a fresh connection per call; long-lived feeds (/stats,
// /events) and hijacked exec streams own a dedicated Stream instead.
class DockerClient {
public:
	explicit DockerClient(std::string socket_path);

	// GET returning the full body; throws std::runtime_error on connect,
	// transport or non-2xx status (message carries the status + detail).
	std::string get(const std::string& path);

	// POST with a JSON body, returning the response body.
	std::string post(const std::string& path, const std::string& json_body);

	// POST whose 2xx response is a hijacked raw stream (exec attach).
	// The returned Stream carries the raw socket after the headers.
	std::unique_ptr<class DockerStream> open_hijack(const std::string& path, const std::string& json_body);

	// Open a long-lived GET stream (/stats, /events); caller reads chunks.
	std::unique_ptr<class DockerStream> open_stream(const std::string& path);

	const std::string& socket_path() const { return socket_path_; }

private:
	std::string socket_path_;
};

// A live connection after the response headers. read() returns 0 on EOF
// (or when the receive timeout fires so pollers can check cancel flags).
class DockerStream {
public:
	~DockerStream();
	DockerStream(const DockerStream&) = delete;
	DockerStream& operator=(const DockerStream&) = delete;

	// Returns bytes read (>0), 0 on timeout/EOF-without-data, -1 on EOF/error.
	int read(char* buffer, size_t size);
	bool write(const char* buffer, size_t size);
	void close();
	int fd() const { return fd_; }

private:
	friend class DockerClient;

public:
	// Public so the internal stream factory can reach it; not API surface.
	DockerStream(int fd, bool chunked);

private:

	int read_raw(char* buffer, size_t size);
	int read_chunked(char* buffer, size_t size);

	int fd_ = -1;
	bool chunked_ = false;
	std::string carry_; // undecoded chunk framing bytes
	enum class ChunkState { Size, Data, DataCrlf, Done } chunk_state_ = ChunkState::Size;
	unsigned long long chunk_remaining_ = 0;
};

#include "docker_client.h"

#include <algorithm>
#include <cerrno>
#include <cstdio>
#include <cstring>
#include <netinet/in.h>
#include <sstream>
#include <stdexcept>
#include <sys/socket.h>
#include <sys/time.h>
#include <sys/un.h>
#include <unistd.h>

namespace {

const char* kApiVersion = "v1.41";
constexpr size_t kMaxResponseBytes = 32 * 1024 * 1024;

int connect_unix(const std::string& path) {
	sockaddr_un addr{};
	addr.sun_family = AF_UNIX;
	if (path.size() >= sizeof(addr.sun_path)) {
		throw std::runtime_error("docker socket path too long");
	}
	std::strncpy(addr.sun_path, path.c_str(), sizeof(addr.sun_path) - 1);
	const int fd = ::socket(AF_UNIX, SOCK_STREAM, 0);
	if (fd < 0) {
		throw std::runtime_error("docker socket: socket() failed");
	}
	if (::connect(fd, reinterpret_cast<sockaddr*>(&addr), sizeof(addr)) != 0) {
		const int saved = errno;
		::close(fd);
		throw std::runtime_error("docker socket connect to " + path + " failed: " + std::strerror(saved));
	}
	timeval tv{};
	tv.tv_sec = 2;
	::setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
	return fd;
}

std::string lower(std::string value) {
	for (auto& c : value) {
		c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
	}
	return value;
}

// Read until the double CRLF ending the response headers; returns the header
// block and leaves fd positioned at the body.
std::string read_headers(int fd) {
	std::string head;
	char c;
	while (head.size() < 64 * 1024) {
		const ssize_t n = ::read(fd, &c, 1);
		if (n <= 0) {
			throw std::runtime_error("docker: connection closed while reading headers");
		}
		head.push_back(c);
		if (head.ends_with("\r\n\r\n")) {
			break;
		}
	}
	return head;
}

struct ResponseMeta {
	int status = 0;
	bool chunked = false;
	long long content_length = -1;
	bool connection_close = false;
};

ResponseMeta parse_headers(const std::string& head) {
	ResponseMeta meta;
	std::istringstream stream(head);
	std::string line;
	if (std::getline(stream, line) && line.size() >= 12) {
		meta.status = std::atoi(line.c_str() + 9);
	}
	while (std::getline(stream, line)) {
		if (!line.empty() && line.back() == '\r') {
			line.pop_back();
		}
		const auto colon = line.find(':');
		if (colon == std::string::npos) {
			continue;
		}
		const auto name = lower(line.substr(0, colon));
		std::string value = line.substr(colon + 1);
		while (!value.empty() && value.front() == ' ') {
			value.erase(value.begin());
		}
		if (name == "transfer-encoding" && lower(value).find("chunked") != std::string::npos) {
			meta.chunked = true;
		} else if (name == "content-length") {
			meta.content_length = std::atoll(value.c_str());
		} else if (name == "connection" && lower(value).find("close") != std::string::npos) {
			meta.connection_close = true;
		}
	}
	return meta;
}

std::string uri_for(const std::string& path) {
	return std::string("/") + kApiVersion + path;
}

void send_all(int fd, const std::string& data) {
	size_t sent = 0;
	while (sent < data.size()) {
		const ssize_t n = ::write(fd, data.data() + sent, data.size() - sent);
		if (n <= 0) {
			throw std::runtime_error("docker: write failed");
		}
		sent += static_cast<size_t>(n);
	}
}

std::string build_request(const std::string& method, const std::string& path, const std::string& body, bool hijack) {
	std::ostringstream req;
	req << method << " " << uri_for(path) << " HTTP/1.1\r\n"
		<< "Host: docker\r\n"
		<< "Accept: */*\r\n";
	if (hijack) {
		req << "Connection: Upgrade\r\nUpgrade: tcp\r\n";
	}
	if (!body.empty()) {
		req << "Content-Type: application/json\r\nContent-Length: " << body.size() << "\r\n";
	} else if (method == "POST") {
		req << "Content-Length: 0\r\n";
	}
	req << "\r\n" << body;
	return req.str();
}

// Drain a complete one-shot body (Content-Length, chunked, or until close).
std::string read_full_body(int fd, const ResponseMeta& meta) {
	std::string body;
	char buffer[64 * 1024];
	if (meta.chunked) {
		std::string carry;
		while (true) {
			const ssize_t n = ::read(fd, buffer, sizeof(buffer));
			if (n < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
				continue;
			}
			if (n <= 0) {
				break;
			}
			carry.append(buffer, static_cast<size_t>(n));
			// Minimal chunk framing walk.
			while (true) {
				const auto eol = carry.find("\r\n");
				if (eol == std::string::npos) {
					break;
				}
				const long long size = std::strtoll(carry.substr(0, eol).c_str(), nullptr, 16);
				if (size == 0) {
					return body;
				}
				if (carry.size() < eol + 2 + static_cast<size_t>(size) + 2) {
					break;
				}
				body.append(carry, eol + 2, static_cast<size_t>(size));
				carry.erase(0, eol + 2 + static_cast<size_t>(size) + 2);
				if (body.size() > kMaxResponseBytes) {
					throw std::runtime_error("docker response too large");
				}
			}
		}
		return body;
	}
	if (meta.content_length >= 0) {
		body.reserve(static_cast<size_t>(meta.content_length));
		long long remaining = meta.content_length;
		while (remaining > 0) {
			const ssize_t n = ::read(fd, buffer, sizeof(buffer));
			if (n < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
				continue;
			}
			if (n <= 0) {
				break;
			}
			body.append(buffer, static_cast<size_t>(n));
			remaining -= n;
		}
		return body;
	}
	while (true) {
		const ssize_t n = ::read(fd, buffer, sizeof(buffer));
		if (n < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
			continue;
		}
		if (n <= 0) {
			break;
		}
		body.append(buffer, static_cast<size_t>(n));
		if (body.size() > kMaxResponseBytes) {
			throw std::runtime_error("docker response too large");
		}
	}
	return body;
}

std::unique_ptr<DockerStream> make_stream(int fd, const ResponseMeta& meta, const std::string& path) {
	if (meta.status < 200 || meta.status >= 300) {
		::close(fd);
		throw std::runtime_error("docker " + uri_for(path) + ": status " + std::to_string(meta.status));
	}
	return std::unique_ptr<DockerStream>(new DockerStream(fd, meta.chunked));
}

} // namespace

DockerClient::DockerClient(std::string socket_path) : socket_path_(std::move(socket_path)) {}

std::string DockerClient::get(const std::string& path) {
	const int fd = connect_unix(socket_path_);
	std::string meta_error;
	try {
		send_all(fd, build_request("GET", path, "", false));
		const auto meta = parse_headers(read_headers(fd));
		auto body = read_full_body(fd, meta);
		::close(fd);
		if (meta.status < 200 || meta.status >= 300) {
			throw std::runtime_error("docker " + uri_for(path) + ": status " + std::to_string(meta.status) + " " + body);
		}
		return body;
	} catch (...) {
		::close(fd);
		throw;
	}
}

std::string DockerClient::post(const std::string& path, const std::string& json_body) {
	const int fd = connect_unix(socket_path_);
	try {
		send_all(fd, build_request("POST", path, json_body, false));
		const auto meta = parse_headers(read_headers(fd));
		auto body = read_full_body(fd, meta);
		::close(fd);
		if (meta.status < 200 || meta.status >= 300) {
			throw std::runtime_error("docker " + uri_for(path) + ": status " + std::to_string(meta.status) + " " + body);
		}
		return body;
	} catch (...) {
		::close(fd);
		throw;
	}
}

std::unique_ptr<DockerStream> DockerClient::open_stream(const std::string& path) {
	const int fd = connect_unix(socket_path_);
	try {
		send_all(fd, build_request("GET", path, "", false));
		const auto meta = parse_headers(read_headers(fd));
		return make_stream(fd, meta, path);
	} catch (...) {
		::close(fd);
		throw;
	}
}

std::unique_ptr<DockerStream> DockerClient::open_hijack(const std::string& path, const std::string& json_body) {
	const int fd = connect_unix(socket_path_);
	try {
		send_all(fd, build_request("POST", path, json_body, true));
		// A hijacked exec stream answers 200 with headers, then raw bytes.
		const auto meta = parse_headers(read_headers(fd));
		return make_stream(fd, meta, path);
	} catch (...) {
		::close(fd);
		throw;
	}
}

DockerStream::DockerStream(int fd, bool chunked) : fd_(fd), chunked_(chunked) {}

DockerStream::~DockerStream() {
	close();
}

void DockerStream::close() {
	if (fd_ >= 0) {
		::close(fd_);
		fd_ = -1;
	}
}

int DockerStream::read(char* buffer, size_t size) {
	if (fd_ < 0) {
		return -1;
	}
	if (!chunked_) {
		return read_raw(buffer, size);
	}
	return read_chunked(buffer, size);
}

int DockerStream::read_raw(char* buffer, size_t size) {
	const ssize_t n = ::read(fd_, buffer, size);
	if (n < 0) {
		if (errno == EAGAIN || errno == EWOULDBLOCK || errno == EINTR) {
			return 0;
		}
		return -1;
	}
	if (n == 0) {
		return -1;
	}
	return static_cast<int>(n);
}

namespace {
// Pull one raw socket read into the carry buffer.
// Returns 1 on bytes, 0 on timeout, -1 on EOF/error.
int pull(int fd, std::string& carry) {
	char raw[64 * 1024];
	const ssize_t n = ::read(fd, raw, sizeof(raw));
	if (n < 0) {
		if (errno == EAGAIN || errno == EWOULDBLOCK || errno == EINTR) {
			return 0;
		}
		return -1;
	}
	if (n == 0) {
		return -1;
	}
	carry.append(raw, static_cast<size_t>(n));
	return 1;
}
} // namespace

int DockerStream::read_chunked(char* buffer, size_t size) {
	int produced = 0;
	while (produced < static_cast<int>(size)) {
		if (chunk_state_ == ChunkState::Done) {
			return produced > 0 ? produced : -1;
		}
		if (chunk_state_ == ChunkState::Size) {
			const auto pos = carry_.find("\r\n");
			if (pos == std::string::npos) {
				const int r = pull(fd_, carry_);
				if (r <= 0) {
					if (r < 0) {
						chunk_state_ = ChunkState::Done;
					}
					return produced > 0 ? produced : (r < 0 ? -1 : 0);
				}
				continue;
			}
			unsigned long long v = 0;
			try {
				v = std::stoull(carry_.substr(0, pos), nullptr, 16);
			} catch (...) {
				return -1;
			}
			carry_.erase(0, pos + 2);
			if (v == 0) {
				chunk_state_ = ChunkState::Done;
				continue;
			}
			chunk_remaining_ = v;
			chunk_state_ = ChunkState::Data;
			continue;
		}
		if (chunk_state_ == ChunkState::Data) {
			if (carry_.empty()) {
				const int r = pull(fd_, carry_);
				if (r <= 0) {
					if (r < 0) {
						chunk_state_ = ChunkState::Done;
					}
					return produced > 0 ? produced : (r < 0 ? -1 : 0);
				}
			}
			const size_t take = std::min({size - static_cast<size_t>(produced),
			                          static_cast<size_t>(chunk_remaining_), carry_.size()});
			std::memcpy(buffer + produced, carry_.data(), take);
			carry_.erase(0, take);
			chunk_remaining_ -= take;
			produced += static_cast<int>(take);
			if (chunk_remaining_ == 0) {
				chunk_state_ = ChunkState::DataCrlf;
			}
			continue;
		}
		// DataCrlf: consume the trailing CRLF of the finished chunk.
		while (carry_.size() < 2) {
			const int r = pull(fd_, carry_);
			if (r <= 0) {
				if (r < 0) {
					chunk_state_ = ChunkState::Done;
				}
				return produced > 0 ? produced : (r < 0 ? -1 : 0);
			}
		}
		carry_.erase(0, 2);
		chunk_state_ = ChunkState::Size;
	}
	return produced;
}

bool DockerStream::write(const char* buffer, size_t size) {
	if (fd_ < 0) {
		return false;
	}
	size_t sent = 0;
	while (sent < size) {
		const ssize_t n = ::write(fd_, buffer + sent, size - sent);
		if (n <= 0) {
			if (errno == EINTR) {
				continue;
			}
			return false;
		}
		sent += static_cast<size_t>(n);
	}
	return true;
}

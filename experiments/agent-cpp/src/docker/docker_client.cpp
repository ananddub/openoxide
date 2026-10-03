#include "docker/docker_client.h"

#include <asio/connect.hpp>
#include <asio/read_until.hpp>
#include <asio/redirect_error.hpp>
#include <asio/write.hpp>

#include <algorithm>
#include <cctype>
#include <cstdlib>
#include <fmt/format.h>
#include <sstream>
#include <stdexcept>

#include "core/async_util.h"

namespace {

const char* kApiVersion = "v1.41";
constexpr size_t kMaxResponseBytes = 32 * 1024 * 1024;
constexpr size_t kReadChunk = 64 * 1024;

std::string lower(std::string value) {
	for (auto& c : value) {
		c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
	}
	return value;
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
	return fmt::format("/{}{}", kApiVersion, path);
}

std::string build_request(const std::string& method, const std::string& path,
                          const std::string& body) {
	std::string req = fmt::format("{} {} HTTP/1.1\r\nHost: docker\r\nAccept: */*\r\n", method, uri_for(path));
	if (!body.empty()) {
		req += fmt::format("Content-Type: application/json\r\nContent-Length: {}\r\n", body.size());
	} else if (method == "POST") {
		req += "Content-Length: 0\r\n";
	}
	req += "\r\n";
	req += body;
	return req;
}

// One connected docker API session: request sent, response headers parsed.
struct Session {
	asio::local::stream_protocol::socket socket;
	ResponseMeta meta;
	std::string head_carry; // body bytes delivered with the headers
};

asio::awaitable<Session> exchange(asio::any_io_executor ex, const std::string& socket_path,
                                  const std::string& request) {
	asio::local::stream_protocol::socket socket{ex};
	asio::local::stream_protocol::endpoint endpoint(socket_path);
	asio::error_code ec;
	co_await socket.async_connect(endpoint, asio::redirect_error(asio::use_awaitable, ec));
	if (ec) {
		throw std::runtime_error(
		    fmt::format("docker socket connect to {} failed: {}", socket_path, ec.message()));
	}
	co_await asio::async_write(socket, asio::buffer(request),
	                           asio::redirect_error(asio::use_awaitable, ec));
	if (ec) {
		throw std::runtime_error("docker: write failed");
	}
	std::string head;
	co_await asio::async_read_until(socket, asio::dynamic_buffer(head), "\r\n\r\n",
	                                asio::redirect_error(asio::use_awaitable, ec));
	if (ec && head.find("\r\n\r\n") == std::string::npos) {
		throw std::runtime_error("docker: connection closed while reading headers");
	}
	// dynamic_buffer consumed everything the kernel delivered at once, so
	// bytes past the header boundary are body, already removed from the
	// socket. Hand them to the body reader instead of losing them.
	std::string head_carry;
	const auto boundary = head.find("\r\n\r\n");
	if (boundary != std::string::npos) {
		head_carry = head.substr(boundary + 4);
		head.resize(boundary);
	} else {
		head_carry = std::move(head);
		head.clear();
	}
	co_return Session{std::move(socket), parse_headers(head), std::move(head_carry)};
}

// Drain a complete one-shot body (Content-Length, chunked, or until close).
asio::awaitable<std::string> read_full_body(Session& session) {
	std::string body;
	std::string carry = std::move(session.head_carry);
	char raw[kReadChunk];
	const auto pull = [&]() -> asio::awaitable<long long> {
		asio::error_code ec;
		const size_t n = co_await session.socket.async_read_some(
			asio::buffer(raw, sizeof(raw)), asio::redirect_error(asio::use_awaitable, ec));
		if (ec) {
			co_return -1;
		}
		carry.append(raw, n);
		co_return static_cast<long long>(n);
	};

	if (session.meta.chunked) {
		while (true) {
			const auto eol = carry.find("\r\n");
			if (eol == std::string::npos) {
				if (co_await pull() < 0) {
					break;
				}
				continue;
			}
			const long long size = std::strtoll(carry.substr(0, eol).c_str(), nullptr, 16);
			if (size == 0) {
				break;
			}
			if (carry.size() < eol + 2 + static_cast<size_t>(size) + 2) {
				if (co_await pull() < 0) {
					break;
				}
				continue;
			}
			body.append(carry, eol + 2, static_cast<size_t>(size));
			carry.erase(0, static_cast<size_t>(eol + 2 + size + 2));
			if (body.size() > kMaxResponseBytes) {
				throw std::runtime_error("docker response too large");
			}
		}
		co_return body;
	}
	if (session.meta.content_length >= 0) {
		body.reserve(static_cast<size_t>(
			std::min<long long>(session.meta.content_length, static_cast<long long>(kMaxResponseBytes))));
		long long remaining = session.meta.content_length;
		while (remaining > 0) {
			if (carry.empty()) {
				if (co_await pull() < 0) {
					break;
				}
			}
			const size_t take = std::min(carry.size(), static_cast<size_t>(remaining));
			body.append(carry, 0, take);
			carry.erase(0, take);
			remaining -= static_cast<long long>(take);
		}
		co_return body;
	}
	while (true) {
		if (carry.empty()) {
			if (co_await pull() < 0) {
				break;
			}
		}
		body += carry;
		carry.clear();
		if (body.size() > kMaxResponseBytes) {
			throw std::runtime_error("docker response too large");
		}
	}
	co_return body;
}

void require_ok(const ResponseMeta& meta, const std::string& path, const std::string& body) {
	if (meta.status < 200 || meta.status >= 300) {
		throw std::runtime_error(fmt::format("docker {}: status {} {}", uri_for(path), meta.status, body));
	}
}

} // namespace

DockerClient::DockerClient(asio::any_io_executor executor, std::string socket_path)
    : executor_(std::move(executor)), socket_path_(std::move(socket_path)) {}

asio::awaitable<std::string> DockerClient::get(const std::string& path) {
	auto session = co_await exchange(executor_, socket_path_, build_request("GET", path, ""));
	auto body = co_await read_full_body(session);
	require_ok(session.meta, path, body);
	co_return body;
}

asio::awaitable<std::string> DockerClient::post(const std::string& path, const std::string& json_body) {
	auto session = co_await exchange(executor_, socket_path_, build_request("POST", path, json_body));
	auto body = co_await read_full_body(session);
	require_ok(session.meta, path, body);
	co_return body;
}

asio::awaitable<std::unique_ptr<DockerStream>> DockerClient::open_stream(const std::string& path) {
	auto session = co_await exchange(executor_, socket_path_, build_request("GET", path, ""));
	if (session.meta.status < 200 || session.meta.status >= 300) {
		throw std::runtime_error(
		    fmt::format("docker {}: status {}", uri_for(path), session.meta.status));
	}
	co_return std::make_unique<DockerStream>(std::move(session.socket), session.meta.chunked,
	                                         std::move(session.head_carry));
}

DockerStream::DockerStream(asio::local::stream_protocol::socket socket, bool chunked,
                           std::string initial_carry)
    : socket_(std::move(socket)), chunked_(chunked), carry_(std::move(initial_carry)) {}

DockerStream::~DockerStream() {
	close();
}

void DockerStream::close() {
	asio::error_code ec;
	socket_.close(ec);
}

asio::awaitable<bool> DockerStream::pull() {
	if (eof_) {
		co_return false;
	}
	char raw[kReadChunk];
	asio::error_code ec;
	const size_t n = co_await socket_.async_read_some(asio::buffer(raw, sizeof(raw)),
	                                                  asio::redirect_error(asio::use_awaitable, ec));
	if (ec) {
		eof_ = true;
		co_return false;
	}
	carry_.append(raw, n);
	co_return true;
}

asio::awaitable<std::optional<std::string>> DockerStream::read() {
	if (!chunked_) {
		if (carry_.empty() && !eof_) {
			if (!co_await pull() && carry_.empty()) {
				co_return std::nullopt;
			}
		}
		if (carry_.empty()) {
			co_return std::nullopt;
		}
		co_return std::move(carry_);
	}

	std::string out;
	while (!out.empty() || !eof_) {
		if (chunk_state_ == ChunkState::Size) {
			const auto pos = carry_.find("\r\n");
			if (pos == std::string::npos) {
				if (!co_await pull()) {
					break;
				}
				continue;
			}
			unsigned long long v = 0;
			try {
				v = std::stoull(carry_.substr(0, pos), nullptr, 16);
			} catch (...) {
				eof_ = true;
				break;
			}
			carry_.erase(0, pos + 2);
			if (v == 0) {
				chunk_state_ = ChunkState::Done;
				break;
			}
			chunk_remaining_ = v;
			chunk_state_ = ChunkState::Data;
			continue;
		}
		if (chunk_state_ == ChunkState::Data) {
			if (carry_.empty()) {
				if (!co_await pull()) {
					break;
				}
				continue;
			}
			const size_t take = std::min(carry_.size(), static_cast<size_t>(chunk_remaining_));
			out.append(carry_, 0, take);
			carry_.erase(0, take);
			chunk_remaining_ -= take;
			if (chunk_remaining_ == 0) {
				chunk_state_ = ChunkState::DataCrlf;
			}
			if (!out.empty()) {
				co_return out;
			}
			continue;
		}
		if (chunk_state_ == ChunkState::DataCrlf) {
			if (carry_.size() < 2) {
				if (!co_await pull()) {
					break;
				}
				continue;
			}
			carry_.erase(0, 2);
			chunk_state_ = ChunkState::Size;
			continue;
		}
		break; // Done
	}
	if (out.empty()) {
		eof_ = true;
	}
	co_return out.empty() ? std::nullopt : std::optional<std::string>{std::move(out)};
}

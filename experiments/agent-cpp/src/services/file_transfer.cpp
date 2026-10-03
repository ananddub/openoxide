#include "services/file_transfer.h"

#include <fmt/format.h>
#include <openssl/evp.h>

#include <algorithm>
#include <array>
#include <cctype>
#include <cerrno>
#include <fstream>
#include <malloc.h>
#include <random>
#include <stdexcept>
#include <sys/stat.h>
#include <unistd.h>

#include "agent.grpc.pb.h"

namespace {

constexpr char kAgentVersion[] = "0.4.0-cpp";

// Validation/IO failures inside the upload coroutine carry their wire status.
struct UploadFail {
	grpc::Status status;
};

std::string random_suffix() {
	static std::random_device device;
	static std::mt19937_64 engine(device());
	static std::uniform_int_distribution<unsigned long long> dist;
	return fmt::format("{:016x}", dist(engine));
}

// Mirrors validate_destination(): absolute, no '..', inside one upload root.
std::string validate_destination(const std::string& raw, const std::vector<std::string>& roots) {
	if (raw.empty() || raw.front() != '/') {
		throw UploadFail{grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "destination must be absolute")};
	}
	std::istringstream stream(raw);
	std::string part;
	while (std::getline(stream, part, '/')) {
		if (part == "..") {
			throw UploadFail{grpc::Status(grpc::StatusCode::PERMISSION_DENIED,
			                              "destination traversal is not allowed")};
		}
	}
	const bool inside = std::any_of(roots.begin(), roots.end(),
	                                [&raw](const std::string& root) {
		                                if (raw.size() < root.size()) {
			                                return false;
		                                }
		                                if (raw.compare(0, root.size(), root) != 0) {
			                                return false;
		                                }
		                                return raw.size() == root.size() || raw[root.size()] == '/';
	                                });
	if (!inside) {
		throw UploadFail{grpc::Status(grpc::StatusCode::PERMISSION_DENIED,
		                              "destination is outside upload roots")};
	}
	return raw;
}

bool create_parent_dirs(const std::string& path) {
	std::string current;
	size_t pos = 0;
	while ((pos = path.find('/', pos + 1)) != std::string::npos) {
		current = path.substr(0, pos);
		if (mkdir(current.c_str(), 0755) != 0 && errno != EEXIST) {
			return false;
		}
	}
	return true;
}

grpc::Status internal_error(const std::string& message) {
	return grpc::Status(grpc::StatusCode::INTERNAL, message);
}

} // namespace

agent::AgentInfoResponse FileTransferService::agent_info() const {
	agent::AgentInfoResponse info;
	info.set_protocol_version("3");
	info.set_agent_version(kAgentVersion);
	info.add_capabilities("file_upload");
	info.set_max_upload_bytes(config_.max_upload_bytes);
	for (const auto& root : config_.upload_roots) {
		info.add_upload_roots(root);
	}
	return info;
}

grpc::ServerReadReactor<agent::FileUploadMessage>* FileTransferService::start(
    grpc::CallbackServerContext* /*context*/, agent::FileUploadResult* response) {
	auto pilot = std::make_shared<CallPilot>(ex_);
	auto box = std::make_shared<UploadReactor::Box>(ex_);
	auto* reactor = new UploadReactor(pilot, box);
	asio::co_spawn(ex_,
	               [this, reactor, box, response, pilot]() -> asio::awaitable<void> {
		               grpc::Status status = grpc::Status::OK;
		               try {
			               status = co_await upload_call(box, pilot, response);
		               } catch (const UploadFail& fail) {
			               status = fail.status;
		               } catch (const std::exception& error) {
			               status = internal_error(error.what());
		               }
		               reactor->finish(status);
	               },
	               asio::detached);
	return reactor;
}

// Port of file_transfer.rs upload(): every read is bounded by
// upload_timeout_seconds (deadline_exceeded), end-of-stream marks completion,
// and the temp file is installed only when size + SHA-256 match exactly.
asio::awaitable<grpc::Status> FileTransferService::upload_call(
    std::shared_ptr<UploadReactor::Box> box, std::shared_ptr<CallPilot> pilot,
    agent::FileUploadResult* response) {
	const std::chrono::milliseconds budget(config_.upload_timeout_seconds * 1000ULL);

	auto first = co_await box->next_within(budget);
	if (!first) {
		if (box->closed()) {
			co_return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "upload stream is empty");
		}
		co_return grpc::Status(grpc::StatusCode::DEADLINE_EXCEEDED, "upload timed out");
	}
	if (!(*first)->has_header()) {
		co_return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "upload must start with a header message");
	}
	const agent::FileUploadHeader header = (*first)->header();
	if (header.server_id() <= 0 || (config_.server_id != -1 && header.server_id() != config_.server_id)) {
		co_return grpc::Status(grpc::StatusCode::NOT_FOUND, "upload server_id does not belong to this agent");
	}

	std::string destination;
	try {
		destination = validate_destination(header.destination(), config_.upload_roots);
	} catch (const UploadFail& fail) {
		co_return fail.status;
	}
	if (header.total_size() == 0 || header.total_size() > config_.max_upload_bytes) {
		co_return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "invalid upload size");
	}
	std::string expected = header.checksum_sha256();
	std::transform(expected.begin(), expected.end(), expected.begin(),
	               [](unsigned char c) { return std::tolower(c); });
	expected.erase(std::remove_if(expected.begin(), expected.end(),
	                              [](unsigned char c) { return c == ' ' || c == '\n' || c == '\t'; }),
	               expected.end());
	if (expected.size() != 64 ||
	    !std::all_of(expected.begin(), expected.end(),
	                 [](unsigned char c) { return std::isxdigit(c) != 0; })) {
		co_return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "invalid SHA-256 checksum");
	}

	const auto parent = destination.substr(0, destination.find_last_of('/'));
	if (parent.empty()) {
		co_return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "destination has no parent");
	}
	if (!create_parent_dirs(parent)) {
		co_return internal_error("could not create destination directory");
	}
	const auto file_name = destination.substr(destination.find_last_of('/') + 1);
	const std::string temp = fmt::format("{}/.{}.upload-{}", parent, file_name, random_suffix());

	EVP_MD_CTX* hasher = EVP_MD_CTX_new();
	if (!hasher || EVP_DigestInit_ex(hasher, EVP_sha256(), nullptr) != 1) {
		if (hasher) {
			EVP_MD_CTX_free(hasher);
		}
		co_return internal_error("sha256 context init failed");
	}

	grpc::Status status = grpc::Status::OK;
	bool written_open = false;
	unsigned long long written = 0;
	std::ofstream file;
	file.open(temp, std::ios::binary);
	if (file) {
		written_open = true;
	} else {
		status = internal_error("could not create upload temp file");
	}

	while (status.ok()) {
		auto chunk = co_await box->next_within(budget);
		if (!chunk) {
			if (box->closed()) {
				break; // end-of-stream marks completion
			}
			status = grpc::Status(grpc::StatusCode::DEADLINE_EXCEEDED, "upload timed out");
			break;
		}
		if (pilot->cancelled()) {
			status = grpc::Status(grpc::StatusCode::CANCELLED, "upload cancelled");
			break;
		}
		if (!(*chunk)->has_data()) {
			status = grpc::Status(grpc::StatusCode::INVALID_ARGUMENT,
			                      "upload metadata must not change mid-stream");
			break;
		}
		const std::string& data = (*chunk)->data();
		if (written + data.size() > header.total_size()) {
			status = grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "upload exceeds declared size");
			break;
		}
		file.write(data.data(), static_cast<std::streamsize>(data.size()));
		EVP_DigestUpdate(hasher, data.data(), data.size());
		written += data.size();
		if (!file.good()) {
			status = internal_error("upload write failed");
			break;
		}
	}
	file.flush();
	file.close();

	unsigned char digest[EVP_MAX_MD_SIZE];
	unsigned int digest_length = 0;
	EVP_DigestFinal_ex(hasher, digest, &digest_length);
	EVP_MD_CTX_free(hasher);

	std::string actual;
	static const char* kHex = "0123456789abcdef";
	for (unsigned int i = 0; i < digest_length; ++i) {
		actual.push_back(kHex[digest[i] >> 4]);
		actual.push_back(kHex[digest[i] & 0xF]);
	}

	if (status.ok() && written == header.total_size() && actual == expected && written_open) {
		if (rename(temp.c_str(), destination.c_str()) == 0) {
			response->set_bytes_written(written);
			response->set_destination(destination);
		} else {
			status = internal_error("could not install uploaded file");
		}
	} else if (status.ok()) {
		status = grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "upload checksum or size mismatch");
	}
	if (!status.ok()) {
		remove(temp.c_str());
	}
	// Chunk buffers are freed already; return the now-empty heap pages.
	malloc_trim(0);
	co_return status;
}

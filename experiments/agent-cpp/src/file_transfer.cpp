#include "file_transfer.h"

#include <openssl/evp.h>

#include <algorithm>
#include <array>
#include <cctype>
#include <cerrno>
#include <cstdio>
#include <fstream>
#include <malloc.h>
#include <random>
#include <sstream>
#include <stdexcept>
#include <sys/stat.h>
#include <unistd.h>

#include "agent.grpc.pb.h"

namespace {

constexpr char kAgentVersion[] = "0.4.0-cpp";

std::string random_suffix() {
	static std::random_device device;
	static std::mt19937_64 engine(device());
	std::uniform_int_distribution<unsigned long long> dist;
	std::array<char, 17> buffer{};
	snprintf(buffer.data(), buffer.size(), "%016llx", dist(engine));
	return std::string(buffer.data());
}

// Mirrors validate_destination(): absolute, no '..', inside one upload root.
std::string validate_destination(const std::string& raw, const std::vector<std::string>& roots) {
	if (raw.empty() || raw.front() != '/') {
		throw std::runtime_error("destination must be absolute");
	}
	std::istringstream stream(raw);
	std::string part;
	while (std::getline(stream, part, '/')) {
		if (part == "..") {
			throw std::runtime_error("destination traversal is not allowed");
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
		throw std::runtime_error("destination is outside upload roots");
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

agent::FileUploadResult FileTransferService::upload(grpc::ServerContext* context,
                                                    grpc::ServerReader<agent::FileUploadMessage>* reader) {
	const auto& config = config_;
	agent::FileUploadMessage first;
	if (!reader->Read(&first)) {
		throw std::runtime_error("upload stream is empty");
	}
	if (!first.has_header()) {
		throw std::runtime_error("upload must start with a header message");
	}
	const auto& header = first.header();
	if (header.server_id() <= 0 || (config.server_id != -1 && header.server_id() != config.server_id)) {
		throw std::runtime_error("upload server_id does not belong to this agent");
	}
	const std::string destination = validate_destination(header.destination(), config.upload_roots);
	if (header.total_size() == 0 || header.total_size() > config.max_upload_bytes) {
		throw std::runtime_error("invalid upload size");
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
		throw std::runtime_error("invalid SHA-256 checksum");
	}

	const auto parent = destination.substr(0, destination.find_last_of('/'));
	if (parent.empty()) {
		throw std::runtime_error("destination has no parent");
	}
	if (!create_parent_dirs(parent)) {
		throw std::runtime_error("could not create destination directory");
	}
	const auto file_name = destination.substr(destination.find_last_of('/') + 1);
	const std::string temp = parent + "/." + file_name + ".upload-" + random_suffix();

	EVP_MD_CTX* hasher = EVP_MD_CTX_new();
	if (!hasher || EVP_DigestInit_ex(hasher, EVP_sha256(), nullptr) != 1) {
		if (hasher) {
			EVP_MD_CTX_free(hasher);
		}
		throw std::runtime_error("sha256 context init failed");
	}

	agent::FileUploadResult result;
	bool ok = false;
	{
		std::ofstream file(temp, std::ios::binary);
		if (!file) {
			EVP_MD_CTX_free(hasher);
			throw std::runtime_error("could not create upload temp file");
		}
		unsigned long long written = 0;
		ok = true;
		agent::FileUploadMessage chunk;
		while (reader->Read(&chunk)) {
			if (!chunk.has_data()) {
				ok = false; // metadata must not change mid-stream
				break;
			}
			const auto& data = chunk.data();
			if (written + data.size() > header.total_size()) {
				ok = false; // exceeds declared size
				break;
			}
			file.write(data.data(), static_cast<std::streamsize>(data.size()));
			EVP_DigestUpdate(hasher, data.data(), data.size());
			written += data.size();
			if (context->IsCancelled()) {
				ok = false;
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

		if (ok && written == header.total_size() && actual == expected) {
			if (rename(temp.c_str(), destination.c_str()) == 0) {
				result.set_bytes_written(written);
				result.set_destination(destination);
			} else {
				ok = false;
			}
		} else {
			ok = false;
		}
	}
	if (!ok) {
		remove(temp.c_str());
		throw std::runtime_error("upload checksum or size mismatch");
	}
	// Chunk buffers are freed already; return the now-empty heap pages.
	malloc_trim(0);
	return result;
}

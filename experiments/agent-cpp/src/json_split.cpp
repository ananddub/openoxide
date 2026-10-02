#include "json_split.h"

#include <istream>
#include <sstream>

// One document per push-loop: std::istream >> nlohmann::json consumes exactly
// the first JSON value, so tellg() marks where the next document starts.
// The packaged parser does all JSON work.
std::vector<nlohmann::json> JsonDocumentScanner::push(const char* data, size_t size) {
	std::vector<nlohmann::json> docs;
	buffer_.append(data, size);

	while (true) {
		const auto first = buffer_.find_first_not_of(" \t\r\n");
		if (first == std::string::npos) {
			buffer_.clear();
			break;
		}
		if (first > 0) {
			buffer_.erase(0, first);
		}

		std::istringstream stream(buffer_);
		nlohmann::json value;
		try {
			stream >> value;
		} catch (const nlohmann::json::exception&) {
			// Malformed or incomplete document: keep bytes, wait for more.
			if (value.is_discarded()) {
				buffer_.clear();
			}
			break;
		}
		if (value.is_discarded()) {
			buffer_.clear();
			break;
		}

		auto consumed = static_cast<size_t>(stream.tellg());
		if (consumed == 0 || consumed > buffer_.size()) {
			// Extraction did not report a position; fall back to a strict
			// single-document parse of the whole buffer.
			nlohmann::json whole = nlohmann::json::parse(buffer_, nullptr, false);
			if (whole.is_discarded()) {
				if (buffer_.size() > 16 * 1024 * 1024) {
					buffer_.clear();
				}
				break;
			}
			docs.push_back(std::move(whole));
			buffer_.clear();
			break;
		}
		docs.push_back(std::move(value));
		buffer_.erase(0, consumed);

		if (buffer_.size() > 16 * 1024 * 1024) {
			buffer_.clear(); // runaway framing safety
			break;
		}
	}
	return docs;
}

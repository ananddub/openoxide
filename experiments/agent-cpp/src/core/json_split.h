#pragma once
#include <nlohmann/json.hpp>
#include <string>
#include <vector>

// Splits Docker's concatenated-JSON stream bodies (stats documents arrive
// back-to-back with no delimiter). All JSON scanning/parsing is delegated to
// the packaged nlohmann SAX API — no hand-rolled parser here.
class JsonDocumentScanner {
public:
	// Feed a raw chunk; returns every complete document parsed so far.
	std::vector<nlohmann::json> push(const char* data, size_t size);

private:
	std::string buffer_;
};

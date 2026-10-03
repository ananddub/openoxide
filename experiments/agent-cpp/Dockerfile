# Build context must be the repo root (the agent compiles the shared
# proto/agent.proto — the same wire contract the Rust agent serves):
#   docker build -f experiments/agent-cpp/Dockerfile -t openoxide-agent-cpp .
FROM debian:bookworm-slim AS build
# This network's middlebox corrupts plain-HTTP .deb downloads ("Hash Sum
# mismatch"), so pin apt to HTTPS mirrors — but the slim base ships no CA
# store, so bootstrap ca-certificates over HTTP first, then switch. The
# retry/sequential Acquire options keep BuildKit's fetches deterministic too.
RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates \
	&& sed -i 's|http://deb.debian.org|https://deb.debian.org|g; s|http://security.debian.org|https://deb.debian.org|g' \
		/etc/apt/sources.list.d/debian.sources \
	&& printf 'Acquire::Retries "3";\nAcquire::http::Pipeline-Depth "0";\n' \
		> /etc/apt/apt.conf.d/90ci-fetch \
	&& apt-get update \
	&& apt-get install -y --no-install-recommends \
		build-essential \
		cmake \
		pkg-config \
		libgrpc++-dev \
		libprotobuf-dev \
		protobuf-compiler \
		protobuf-compiler-grpc \
		nlohmann-json3-dev \
		libfmt-dev \
		libasio-dev \
		libcurl4-openssl-dev \
		libconcurrentqueue-dev \
		libssl-dev \
	&& rm -rf /var/lib/apt/lists/*
COPY proto/agent.proto /src/proto/agent.proto
COPY experiments/agent-cpp /src/experiments/agent-cpp
WORKDIR /build
RUN cmake -DCMAKE_BUILD_TYPE=MinSizeRel /src/experiments/agent-cpp \
	&& make -j"$(nproc)" \
	&& strip --strip-all openoxide-agent-cpp

FROM debian:bookworm-slim
# Same HTTP -> CA bootstrap -> HTTPS sequence as the build stage.
RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates \
	&& sed -i 's|http://deb.debian.org|https://deb.debian.org|g; s|http://security.debian.org|https://deb.debian.org|g' \
		/etc/apt/sources.list.d/debian.sources \
	&& printf 'Acquire::Retries "3";\nAcquire::http::Pipeline-Depth "0";\n' \
		> /etc/apt/apt.conf.d/90ci-fetch \
	&& apt-get update \
	&& apt-get install -y --no-install-recommends \
		libgrpc++1.51 \
		libprotobuf32 \
		libfmt9 \
		libssl3 \
		libcurl4 \
		docker.io \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=build /build/openoxide-agent-cpp /usr/local/bin/openoxide-agent-cpp
EXPOSE 50051
# Mount the host Docker socket at /var/run/docker.sock when running.
CMD ["/usr/local/bin/openoxide-agent-cpp"]

# Build context must be the repo root (the agent compiles the shared
# proto/agent.proto — the same wire contract the Rust agent serves):
#   docker build -f experiments/agent-cpp/Dockerfile -t openoxide-agent-cpp .
FROM debian:bookworm-slim AS build
RUN apt-get update && apt-get install -y --no-install-recommends \
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
		libssl-dev
COPY proto/agent.proto /src/proto/agent.proto
COPY experiments/agent-cpp /src/experiments/agent-cpp
WORKDIR /build
RUN cmake -DCMAKE_BUILD_TYPE=MinSizeRel /src/experiments/agent-cpp \
	&& make -j"$(nproc)" \
	&& strip --strip-all openoxide-agent-cpp

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
		libgrpc++1.51 \
		libprotobuf32 \
		libfmt9 \
		libssl3 \
		libcurl4 \
		ca-certificates \
		docker.io \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=build /build/openoxide-agent-cpp /usr/local/bin/openoxide-agent-cpp
EXPOSE 50051
# Mount the host Docker socket at /var/run/docker.sock when running.
CMD ["/usr/local/bin/openoxide-agent-cpp"]

# Dev container for the experimental C++ agent, scoped to this folder only.
# The workspace (experiments/agent-cpp) is mounted by VS Code; the build's
# .proto files are bind-mounted read-only at /proto-src (see devcontainer.json).
FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
		build-essential \
		cmake \
		ninja-build \
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
		gdb \
		valgrind \
		strace \
		ltrace \
		cppcheck \
		clangd \
		ca-certificates \
		curl \
		git \
		procps \
		docker.io \
	&& rm -rf /var/lib/apt/lists/*

# grpcurl for RPC smoke tests inside the container
RUN curl -sL https://github.com/fullstorydev/grpcurl/releases/download/v1.9.1/grpcurl_1.9.1_linux_x86_64.tar.gz \
	| tar xz -C /usr/local/bin grpcurl

# uid 1000 matches the host user, so builds in the mounted workspace keep
# sane ownership instead of leaving root-owned artifacts behind.
RUN useradd -m -u 1000 -s /bin/bash vscode

# sudo + a few extra tools as a small top layer (keeps the big apt layer cached).
RUN apt-get update && apt-get install -y --no-install-recommends sudo gdu \
	&& rm -rf /var/lib/apt/lists/* \
	&& echo 'vscode ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/vscode \
	&& chmod 440 /etc/sudoers.d/vscode

# Root-mode entrypoint hook: joins vscode to the host docker.sock group at
# container start (devcontainer.json sets containerUser=root, remoteUser=vscode).
COPY .devcontainer/devcontainer-entrypoint.sh /usr/local/bin/devcontainer-entrypoint.sh

WORKDIR /workspaces/agent-cpp
USER vscode
ENTRYPOINT ["/usr/local/bin/devcontainer-entrypoint.sh"]
# Keep the container alive; devcontainer.json sets overrideCommand=false so
# this CMD (after the root entrypoint hook) is what actually runs.
CMD ["/bin/sh", "-c", "while :; do sleep 3600; done"]

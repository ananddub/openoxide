# C++ agent experiments — one-stop entry points.
# Inside the dev container (`make compile` etc.) builds run directly with the
# preinstalled toolchain. Outside it, the same targets wrap Docker so a bare
# host still needs no gRPC toolchain. `make dev-image` explicitly builds the
# dev toolchain image with docker.

SELF        := $(abspath $(lastword $(MAKEFILE_LIST)))
HERE        := $(dir $(SELF))
ROOT        := $(abspath $(HERE)../..)

IMAGE       := openoxide-agent-cpp
DEV_IMAGE   := agent-cpp-dev
BINARY      := $(HERE)build/openoxide-agent-cpp

# Mirror the VS Code devcontainer mount layout exactly (repo root at
# /workspaces/openoxide), so the CMake cache in ./build stays valid for both.
DEV_RUN     := docker run --rm -u $(shell id -u):$(shell id -g) \
	-v $(ROOT):/workspaces/openoxide \
	-v $(ROOT)/proto:/proto-src:ro \
	-w /workspaces/openoxide/experiments/agent-cpp $(DEV_IMAGE)

IN_DEV_CONTAINER := $(shell test -f /.dockerenv && echo 1)

.DEFAULT_GOAL := help
.PHONY: help dev-image compile image run shell size clean

help: ## List all targets
	@grep -hE '^[a-z-]+:.*?## ' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2}'

ifeq ($(IN_DEV_CONTAINER),1)

dev-image: ## No-op inside the dev container
	@echo "inside the dev container — toolchain is already installed, no image needed"

compile: ## Direct cmake+ninja build (detected: inside dev container)
	cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=MinSizeRel
	ninja -C build
	strip --strip-all build/openoxide-agent-cpp
	@ls -l $(BINARY)

shell: ## No-op inside the dev container
	@echo "you are already inside the dev container"

else

dev-image: ## Build the dev toolchain image from .devcontainer/Dockerfile
	docker build -f $(HERE).devcontainer/Dockerfile -t $(DEV_IMAGE) $(HERE)

compile: ## Configure + build + strip inside the dev image
	$(DEV_RUN) sh -c 'cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=MinSizeRel && ninja -C build && strip --strip-all build/openoxide-agent-cpp'
	@ls -l $(BINARY)

shell: ## Interactive bash inside the dev image (toolchain + grpcurl ready)
	$(DEV_RUN) -it bash

endif

image: ## Build the production multi-stage agent image
	docker build -f $(HERE)Dockerfile -t $(IMAGE) $(ROOT)

run: ## Run the agent image against the host Docker socket (needs METRICS_TOKEN)
	docker run --rm -it \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-e SERVER_ID=$${SERVER_ID:-1} \
		-e METRICS_TOKEN=$${METRICS_TOKEN:-devtoken} \
		-e GRPC_PORT=$${GRPC_PORT:-50051} \
		-p 50051:50051 $(IMAGE)

size: ## Show binary and image sizes
	@ls -l $(BINARY) 2>/dev/null || echo "no binary — run 'make compile' first"
	@docker image inspect $(IMAGE) --format 'agent image: {{.Size}} bytes' 2>/dev/null || true

clean: ## Remove local build artifacts
	rm -rf $(HERE)build

# agent-cpp — experimental C++ port of the Rust `agent/`

A from-scratch C++20 implementation of the OpenOxide monitoring agent, speaking
the **exact same gRPC contract** as the production Rust agent: it compiles the
repo-root [`proto/agent.proto`](../../proto/agent.proto), so the existing panel
can talk to either implementation unchanged.

This is an experiment folder — it is not part of the Cargo workspace and nothing
in `src/`, `agent/` or `web/` depends on it.

## What is replicated (full parity)

| Rust source | C++ counterpart |
|---|---|
| `core/config.rs` (env vars + validation) | `src/config.{h,cpp}` |
| `core/middleware.rs` (`x-metrics-token` auth) | `AgentServiceImpl::authenticate` |
| `api/agent.rs` (9 RPCs, `check_server_id`) | `src/agent_service.{h,cpp}` |
| `app/grpc.rs` (TLS / mTLS with wait-for-provisioning) | `src/main.cpp` |
| `telemetry/collector/system` (5 s host sampling loop) | `src/collector.{h,cpp}` (reads `/proc`, `statvfs`) |
| `utils/docker/transport/api.rs` (unix-socket HTTP) | `src/docker_client.{h,cpp}` |
| `utils/docker/query/types.rs` (stats formulas) | `src/docker_stats.{h,cpp}` |
| `services/monitoring/hub.rs` (StatsHub / EventsHub) | `src/hubs.{h,cpp}` |
| `services/monitoring/metrics.rs` / `state.rs` / `stats.rs` / `lifecycle.rs` | RPCs in `agent_service.cpp` |
| `services/monitoring/shell.rs` (PTY shell, ExecTask, page-cache reclaim) | `src/shell_service.{h,cpp}` |
| `services/file_transfer.rs` (SHA-256 streamed upload) | `src/file_transfer.{h,cpp}` |

Behavioral details kept identical: `docker stats --format {{json .}}`-shaped
`StatsFrame` lines, `/v1.41` API prefix, short/full container-id tolerant
matching, 15 s watch heartbeat + 25 ms event coalescing, upload roots/traversal
rules, NOT_FOUND on server_id mismatch, `memory.reclaim` after ExecTask.

Simplifications vs. Rust (documented deviations):
- JSON parsing/frames always go through the packaged `nlohmann::json` — no
  hand-written parser.
- Host-escape shell uses `nsenter -t 1` directly (when present) instead of
  spawning a privileged helper container from the agent's own image.
- Single Unix-socket HTTP calls are opened per request (Rust pools one
  persistent connection).

## Build (Docker — the host needs no gRPC toolchain)

Everything is wrapped in a `Makefile` — run it from this folder:

```sh
make compile   # stripped binary in ./build (built inside the dev image)
make image     # production openoxide-agent-cpp image
make run       # run the image against the host docker.sock
make shell     # interactive dev shell (cmake, grpcurl, gdb, valgrind)
make size      # binary + image sizes
make help      # all targets
```

## Dev container (VS Code)

Open **only this folder** (`experiments/agent-cpp`) in VS Code → "Reopen in
Container". The dev container is scoped to the C++ project: the workspace mount
is this folder, and just the repo's `proto/` directory is bind-mounted
read-only at `/proto-src` for codegen (CMake auto-falls back to it). Runs as
`vscode` (uid 1000) so build artifacts in the mounted folder stay host-owned.

Preinstalled: cmake/ninja, gRPC/protobuf dev headers, clangd, gdb, valgrind,
cppcheck, docker CLI + `docker.sock`, and `grpcurl`.

```sh
cd /workspaces/agent-cpp
cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=MinSizeRel && ninja -C build
```

## Actual RAM (private, non-reclaimable)

Idle ≈ **2.3 MB** anonymous (`Pss_Anon` in `/proc/1/smaps_rollup`); `docker
stats` shows ~5–6 MiB cgroup (that also counts reclaimable mapped library
pages). Measuring:

```sh
docker exec <agent> sh -c 'grep -E "^(Pss_Anon|Private_Dirty)" /proc/1/smaps_rollup'
valgrind --tool=massif ./build/openoxide-agent-cpp   # allocation breakdown
```

Tuning knobs already in place: `MinSizeRel` + `strip` (554 KB binary),
`NUM_CQS=2`, `malloc_trim(0)` after ExecTask/Upload (glibc otherwise keeps
freed heap pages mapped), cgroup page-cache reclaim after deploys. The floor
is the libgrpc++/protobuf runtime itself (~2 MB: arenas, CQs, poller stacks) —
going below that means a statically trimmed gRPC build, not app-level tuning.

## Run

```sh
docker run --rm -p 50051:50051 \
	-v /var/run/docker.sock:/var/run/docker.sock \
	-e SERVER_ID=1 -e GRPC_PORT=50051 -e METRICS_TOKEN=secret \
	openoxide-agent-cpp
```

Same env contract as the Rust agent: `SERVER_ID`, `GRPC_PORT`, `METRICS_TOKEN`,
`DOCKER_SOCKET`, `MAX_UPLOAD_BYTES`, `UPLOAD_TIMEOUT_SECONDS`, `UPLOAD_ROOTS`,
`AGENT_TLS_CERT`/`AGENT_TLS_KEY`/`AGENT_TLS_CLIENT_CA`.

## Smoke test (grpcurl)

```sh
docker run --rm -p 50051:50051 -v /var/run/docker.sock:/var/run/docker.sock \
	-e SERVER_ID=1 -e GRPC_PORT=50051 -e METRICS_TOKEN=secret \
	-v $PWD/proto:/proto \
	openoxide-agent-cpp &

grpcurl -plaintext -H 'x-metrics-token: secret' \
	-d '{"server_id": 1}' \
	-import-proto proto/agent.proto \
	localhost:50051 agent.AgentService/GetAgentInfo
```

## Layout

```
Makefile          make compile / image / run / shell / size / clean
Dockerfile        production multi-stage image (repo-root build context)
.devcontainer/    VS Code dev container (same toolchain as `make compile`)
build/            generated binary (gitignored)
src/
  main.cpp          server bootstrap, TLS, signals
  config.*          env config + validation
  agent_service.*   all 9 AgentService RPCs
  collector.*       /proc host sampler (background thread)
  docker_client.*   HTTP/1.1 over docker.sock (+ chunked streams, hijack)
  docker_stats.*    stats doc, docker-CLI line rendering, byte formatting
  hubs.*            StatsHub / EventsHub shared Docker feeds + subscribers
  json_split.*      streamed JSON-document framing via nlohmann parser
  shell_service.*   PTY Shell + ExecTask
  file_transfer.*   UploadFile (SHA-256, atomic install)
```

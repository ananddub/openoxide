#pragma once

#include <memory>

// Installs a two-worker (+one timer) EventEngine as gRPC's default factory.
// gRPC's built-in engine sizes its thread pool to hardware_concurrency (32
// threads on a big dev host), and those stacks dominate the agent's RSS.
// Must be called before any gRPC object is created.
void InstallTinyEventEngine();

# CMake build for the experimental C++ port of the openoxide agent.
# Mirrors agent/ (Rust): same proto/agent.proto, same AgentService surface.
#
# Build inside Docker (see Dockerfile) — host needs no gRPC toolchain:
#   docker build -t openoxide-agent-cpp experiments/agent-cpp
cmake_minimum_required(VERSION 3.22)
project(openoxide_agent_cpp CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# GCC 12 keeps C++20 coroutines behind a flag.
if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
	add_compile_options(-fcoroutines)
endif()

# The single source of truth for the wire contract is the repo-root proto.
# The dev container scopes the workspace to this folder and bind-mounts the
# proto dir at /proto-src, so fall back to that when the repo is not present.
get_filename_component(REPO_ROOT "${CMAKE_CURRENT_SOURCE_DIR}" DIRECTORY)
get_filename_component(REPO_ROOT "${REPO_ROOT}" DIRECTORY)
set(AGENT_PROTO "${REPO_ROOT}/proto/agent.proto")
if(NOT EXISTS "${AGENT_PROTO}" AND EXISTS "/proto-src/agent.proto")
	set(AGENT_PROTO "/proto-src/agent.proto")
endif()
if(NOT EXISTS "${AGENT_PROTO}")
	message(FATAL_ERROR "proto/agent.proto not found at ${AGENT_PROTO} or /proto-src/agent.proto")
endif()
get_filename_component(AGENT_PROTO_DIR "${AGENT_PROTO}" DIRECTORY)

find_package(Protobuf REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(CURL REQUIRED)

# gRPC: prefer the CMake config package, fall back to pkg-config (Debian).
find_package(gRPC CONFIG QUIET)
if(gRPC_FOUND)
	set(GRPC_LIBS gRPC::grpc++)
	find_program(GRPC_CPP_PLUGIN grpc_cpp_plugin REQUIRED)
else()
	find_package(PkgConfig REQUIRED)
	pkg_check_modules(GRPC REQUIRED grpc++)
	find_program(GRPC_CPP_PLUGIN grpc_cpp_plugin REQUIRED)
endif()

set(PROTO_GEN "${CMAKE_CURRENT_BINARY_DIR}/gen")
file(MAKE_DIRECTORY ${PROTO_GEN})

add_custom_command(
	OUTPUT ${PROTO_GEN}/agent.pb.cc ${PROTO_GEN}/agent.pb.h
	COMMAND ${Protobuf_PROTOC_EXECUTABLE}
		--proto_path=${AGENT_PROTO_DIR}
		--cpp_out=${PROTO_GEN}
		${AGENT_PROTO}
	DEPENDS ${AGENT_PROTO}
	VERBATIM)

add_custom_command(
	OUTPUT ${PROTO_GEN}/agent.grpc.pb.cc ${PROTO_GEN}/agent.grpc.pb.h
	COMMAND ${Protobuf_PROTOC_EXECUTABLE}
		--proto_path=${AGENT_PROTO_DIR}
		--grpc_out=${PROTO_GEN}
		--plugin=protoc-gen-grpc=${GRPC_CPP_PLUGIN}
		${AGENT_PROTO}
	DEPENDS ${AGENT_PROTO} ${PROTO_GEN}/agent.pb.h
	VERBATIM)

add_library(agent_proto STATIC
	${PROTO_GEN}/agent.pb.cc
	${PROTO_GEN}/agent.grpc.pb.cc)
target_include_directories(agent_proto PUBLIC ${PROTO_GEN})
if(gRPC_FOUND)
	target_link_libraries(agent_proto PUBLIC ${GRPC_LIBS} protobuf::libprotobuf)
else()
	target_link_libraries(agent_proto PUBLIC ${GRPC_LIBRARIES} protobuf::libprotobuf)
	target_link_directories(agent_proto PUBLIC ${GRPC_LIBRARY_DIRS})
	target_include_directories(agent_proto PUBLIC ${GRPC_INCLUDE_DIRS})
endif()

add_executable(openoxide-agent-cpp
	src/main.cpp
	src/core/config.cpp
	src/core/tiny_event_engine.cpp
	src/core/json_split.cpp
	src/docker/docker_client.cpp
	src/docker/docker_stats.cpp
	src/docker/hubs.cpp
	src/telemetry/collector.cpp
	src/services/file_transfer.cpp
	src/services/shell_service.cpp
	src/services/agent_service.cpp)
# Project headers are included src-rooted ("core/config.h"), so src itself
# is an include directory.
target_include_directories(openoxide-agent-cpp PRIVATE src)
target_link_libraries(openoxide-agent-cpp PRIVATE
	agent_proto
	OpenSSL::Crypto
	CURL::libcurl)
find_package(nlohmann_json QUIET)
if(nlohmann_json_FOUND)
	target_link_libraries(openoxide-agent-cpp PRIVATE nlohmann_json::nlohmann_json)
endif()
find_package(fmt QUIET)
if(fmt_FOUND)
	target_link_libraries(openoxide-agent-cpp PRIVATE fmt::fmt)
endif()

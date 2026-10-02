#include "shell_service.h"

#include <fcntl.h>
#include <grp.h>
#include <pty.h>
#include <signal.h>
#include <sys/ioctl.h>
#include <sys/select.h>
#include <sys/wait.h>
#include <termios.h>
#include <unistd.h>

#include <algorithm>
#include <atomic>
#include <cstdlib>
#include <cstring>
#include <mutex>
#include <nlohmann/json.hpp>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

#include <malloc.h>

#include "agent.grpc.pb.h"
#include "hubs.h"

namespace {

template <typename Writer>
void send_output(Writer* writer, agent::ShellMessage::PayloadCase kind, const std::string& data) {
	agent::ShellMessage message;
	if (kind == agent::ShellMessage::kStdout) {
		*message.mutable_stdout()->mutable_data() = data;
	} else {
		*message.mutable_stderr()->mutable_data() = data;
	}
	writer->Write(message);
}

template <typename Writer>
void send_exit(Writer* writer, int code) {
	agent::ShellMessage message;
	message.mutable_exit()->set_exit_code(code);
	writer->Write(message);
}

template <typename Writer>
void send_shell_error(Writer* writer, const std::string& message) {
	agent::ShellMessage out;
	out.mutable_error()->set_message(message);
	writer->Write(out);
}

// Spawn a process attached to a PTY master. `host` requests a privileged
// helper escape (mirrors the nsenter path); otherwise `argv` runs directly.
struct PtySession {
	int master = -1;
	pid_t child = -1;
};

PtySession spawn_pty(const std::vector<std::string>& argv, const std::string& cwd) {
	PtySession session;
	int master = -1;
	const pid_t child = forkpty(&master, nullptr, nullptr, nullptr);
	if (child < 0) {
		return session;
	}
	if (child == 0) {
		if (!cwd.empty()) {
			if (chdir(cwd.c_str()) != 0) {
				_exit(127);
			}
		}
		setenv("TERM", "xterm-256color", 1);
		std::vector<char*> args;
		args.reserve(argv.size() + 1);
		for (const auto& arg : argv) {
			args.push_back(const_cast<char*>(arg.c_str()));
		}
		args.push_back(nullptr);
		execvp(args[0], args.data());
		_exit(127);
	}
	session.master = master;
	session.child = child;
	return session;
}

void pty_resize(int master, uint32_t cols, uint32_t rows) {
	winsize size{};
	size.ws_col = static_cast<unsigned short>(cols == 0 ? 80 : cols);
	size.ws_row = static_cast<unsigned short>(rows == 0 ? 24 : rows);
	ioctl(master, TIOCSWINSZ, &size);
}

int pty_close(PtySession& session) {
	if (session.master >= 0) {
		close(session.master);
		session.master = -1;
	}
	int status = 0;
	if (session.child > 0) {
		waitpid(session.child, &status, 0);
		session.child = -1;
	}
	if (WIFEXITED(status)) {
		return WEXITSTATUS(status);
	}
	if (WIFSIGNALED(status)) {
		return 128 + WTERMSIG(status);
	}
	return 0;
}

} // namespace

void reclaim_own_page_cache() {
	static const char* paths[] = {"/sys/fs/cgroup/memory.reclaim", "/sys/fs/cgroup/memory/memory.reclaim"};
	for (const char* path : paths) {
		if (access(path, W_OK) == 0) {
			int fd = open(path, O_WRONLY);
			if (fd >= 0) {
				const char* value = "536870912";
				ssize_t written = write(fd, value, strlen(value));
				(void)written;
				close(fd);
			}
			return;
		}
	}
	// glibc keeps freed heap pages mapped (RSS) for reuse; hand them back so
	// the deploy task's buffers do not linger after completion.
	malloc_trim(0);
}

grpc::Status ShellService::Open(grpc::ServerContext* context,
                                grpc::ServerReaderWriter<agent::ShellMessage, agent::ShellMessage>* stream) {
	agent::ShellMessage first;
	if (!stream->Read(&first) || !first.has_open()) {
		return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "missing shell open message");
	}
	const agent::ShellOpen open = first.open();
	if (open.server_id() <= 0 || (config_.server_id != -1 && open.server_id() != config_.server_id)) {
		return grpc::Status(grpc::StatusCode::NOT_FOUND, "shell server_id does not match this agent");
	}

	std::vector<std::string> argv;
	const std::string shell = open.shell().empty() ? "sh" : open.shell();

	if (open.host()) {
		// Host escape: nsenter into PID 1's namespaces when available, which
		// requires the agent to share the host PID namespace (docker-compose
		// mounts it in production). Falls back to a direct host shell.
		if (access("/usr/bin/nsenter", X_OK) == 0) {
			argv = {"nsenter", "-t", "1", "-m", "-u", "-i", "-n", "-p", "--", shell};
		} else {
			argv = {shell};
		}
	} else if (!open.container().empty()) {
		// Container shell via the Docker CLI, mirroring open_container():
		// resolve the requested name against the live container list first —
		// exact compose-service label match wins, then name substring hits.
		std::string target = open.container();
		try {
			const auto containers = ContainerSummary::list(docker_, true);
			std::string search = target;
			std::transform(search.begin(), search.end(), search.begin(),
			               [](unsigned char c) { return std::tolower(c); });
			std::string clean = search;
			if (clean.ends_with("_db")) {
				clean.resize(clean.size() - 3);
			} else if (clean.ends_with("-db")) {
				clean.resize(clean.size() - 3);
			}
			const ContainerSummary* found = nullptr;
			for (const auto& container : containers) {
				if (container.label("com.docker.compose.service") == search ||
				    [&] {
					    std::string value = container.label("com.docker.compose.service");
					    std::transform(value.begin(), value.end(), value.begin(),
					                   [](unsigned char c) { return std::tolower(c); });
					    return !value.empty() && value == search;
				    }()) {
					found = &container;
					break;
				}
			}
			if (found == nullptr) {
				for (const auto& container : containers) {
					std::string name = container.name;
					std::transform(name.begin(), name.end(), name.begin(),
					               [](unsigned char c) { return std::tolower(c); });
					const bool hit = name.find(search) != std::string::npos ||
						name.rfind(search, 0) == 0 ||
						name.find(search + "_") != std::string::npos ||
						name.find(search + ".") != std::string::npos ||
						(!clean.empty() &&
						 (name.find(clean) != std::string::npos || name.rfind(clean, 0) == 0));
					if (hit) {
						found = &container;
						break;
					}
				}
			}
			if (found != nullptr) {
				target = found->id.empty() ? found->name : found->id;
			}
		} catch (...) {
			// Docker unreachable: fall back to the raw name, `docker exec`
			// will report the error on the stream.
		}
		argv = {"docker", "exec", "-i", "-t", target, shell};
	} else {
		argv = {shell};
	}

	PtySession session = spawn_pty(argv, "");
	if (session.master < 0) {
		send_shell_error(stream, "could not start shell");
		return grpc::Status::OK;
	}
	pty_resize(session.master, open.cols(), open.rows());

	std::atomic<bool> stop{false};

	// Writer/pty-read pump runs on this thread; an input pump thread feeds stdin.
	std::thread input([stream, &session, &stop, open, this] {
		agent::ShellMessage message;
		while (!stop && stream->Read(&message)) {
			if (message.has_input()) {
				const std::string& data = message.input().data();
				if (!data.empty()) {
					ssize_t written = write(session.master, data.data(), data.size());
					if (written < 0) {
						break;
					}
				}
			} else if (message.has_resize()) {
				pty_resize(session.master, message.resize().cols(), message.resize().rows());
			} else if (message.has_close()) {
				break;
			}
		}
		stop = true;
	});

	char buffer[4096];
	while (!stop && !context->IsCancelled()) {
		fd_set reads;
		FD_ZERO(&reads);
		FD_SET(session.master, &reads);
		timeval timeout{0, 200000};
		const int ready = select(session.master + 1, &reads, nullptr, nullptr, &timeout);
		if (ready > 0 && FD_ISSET(session.master, &reads)) {
			const ssize_t n = read(session.master, buffer, sizeof(buffer));
			if (n > 0) {
				send_output(stream, agent::ShellMessage::kStdout, std::string(buffer, static_cast<size_t>(n)));
			} else if (n == 0) {
				break;
			} else if (errno != EAGAIN && errno != EWOULDBLOCK) {
				break;
			}
		}
		// Reap when the child has exited even if the master still buffers data.
		if (session.child > 0) {
			int status = 0;
			const pid_t done = waitpid(session.child, &status, WNOHANG);
			if (done == session.child) {
				session.child = -1;
				int code = 0;
				if (WIFEXITED(status)) {
					code = WEXITSTATUS(status);
				} else if (WIFSIGNALED(status)) {
					code = 128 + WTERMSIG(status);
				}
				send_exit(stream, code);
				stop = true;
				break;
			}
		}
	}

	stop = true;
	if (session.child > 0) {
		kill(session.child, SIGTERM);
	}
	pty_close(session);
	if (input.joinable()) {
		input.join();
	}
	return grpc::Status::OK;
}

grpc::Status ShellService::ExecTask(grpc::ServerContext* context, const agent::ExecTaskRequest& request,
                                    grpc::ServerWriter<agent::ExecTaskOutput>* writer) {
	if (request.server_id() <= 0 || (config_.server_id != -1 && request.server_id() != config_.server_id)) {
		return grpc::Status(grpc::StatusCode::NOT_FOUND, "task server_id does not match this agent");
	}
	if (request.command().empty()) {
		return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "task command cannot be empty");
	}

	std::vector<std::string> argv;
	if (request.container().empty()) {
		argv = {"sh", "-lc", request.command()};
	} else {
		argv = {"docker", "exec", "-i"};
		if (!request.working_directory().empty()) {
			argv.push_back("-w");
			argv.push_back(request.working_directory());
		}
		argv.push_back(request.container());
		argv.push_back("sh");
		argv.push_back("-lc");
		argv.push_back(request.command());
	}

	// Use pipes (no PTY) so stdout and stderr are separated, like the Rust path.
	int out_pipe[2];
	int err_pipe[2];
	int in_pipe[2];
	if (pipe(out_pipe) != 0 || pipe(err_pipe) != 0) {
		return grpc::Status(grpc::StatusCode::INTERNAL, "could not create task pipes");
	}
	if (!request.stdin().empty() && pipe(in_pipe) != 0) {
		return grpc::Status(grpc::StatusCode::INTERNAL, "could not create task stdin pipe");
	}

	const int has_stdin = !request.stdin().empty();
	const std::string command = request.command();

	pid_t child = fork();
	if (child < 0) {
		return grpc::Status(grpc::StatusCode::INTERNAL, "fork failed");
	}
	if (child == 0) {
		dup2(out_pipe[1], STDOUT_FILENO);
		dup2(err_pipe[1], STDERR_FILENO);
		if (has_stdin) {
			dup2(in_pipe[0], STDIN_FILENO);
		} else {
			int null_fd = open("/dev/null", O_RDONLY);
			dup2(null_fd, STDIN_FILENO);
			if (null_fd > 2) {
				close(null_fd);
			}
		}
		close(out_pipe[0]);
		close(out_pipe[1]);
		close(err_pipe[0]);
		close(err_pipe[1]);
		if (has_stdin) {
			close(in_pipe[0]);
			close(in_pipe[1]);
		}
		if (!request.container().empty() && !request.working_directory().empty()) {
			// docker exec -w handles the working dir; host cwd only applies when
			// running directly.
		} else if (request.container().empty() && !request.working_directory().empty()) {
			if (chdir(request.working_directory().c_str()) != 0) {
				_exit(127);
			}
		}
		std::vector<char*> args;
		for (const auto& arg : argv) {
			args.push_back(const_cast<char*>(arg.c_str()));
		}
		args.push_back(nullptr);
		execvp(args[0], args.data());
		_exit(127);
	}

	close(out_pipe[1]);
	close(err_pipe[1]);
	if (has_stdin) {
		close(in_pipe[0]);
	}

	if (has_stdin) {
		std::thread([in_pipe, data = request.stdin()] {
			ssize_t written = write(in_pipe[1], data.data(), data.size());
			(void)written;
			close(in_pipe[1]);
		}).detach();
	}

	std::atomic<bool> stop{false};
	// gRPC's sync writer is not safe for concurrent Write: stdout and stderr
	// pumps serialize through this mutex, like the Rust mpsc funnel.
	std::mutex write_mutex;
	const uint32_t timeout_seconds = request.timeout_seconds();
	const auto deadline = std::chrono::steady_clock::now() +
		std::chrono::seconds(timeout_seconds == 0 ? 31536000 : timeout_seconds);

	auto pump = [&](int fd, agent::ExecTaskOutputKind kind) {
		char buffer[4096];
		ssize_t n;
		while (!stop && (n = read(fd, buffer, sizeof(buffer))) > 0) {
			agent::ExecTaskOutput output;
			output.set_kind(kind);
			output.set_data(buffer, static_cast<size_t>(n));
			std::lock_guard<std::mutex> guard(write_mutex);
			writer->Write(output);
		}
		close(fd);
	};

	std::thread out_thread(pump, out_pipe[0], agent::EXEC_TASK_OUTPUT_STDOUT);
	std::thread err_thread(pump, err_pipe[0], agent::EXEC_TASK_OUTPUT_STDERR);

	int status = 0;
	bool timed_out = false;
	while (true) {
		const pid_t done = waitpid(child, &status, WNOHANG);
		if (done == child) {
			break;
		}
		if (context->IsCancelled() || std::chrono::steady_clock::now() > deadline) {
			timed_out = true;
			kill(child, SIGKILL);
			waitpid(child, &status, 0);
			break;
		}
		if (done < 0) {
			break;
		}
		std::this_thread::sleep_for(std::chrono::milliseconds(20));
	}

	out_thread.join();
	err_thread.join();
	stop = true;

	if (timed_out) {
		agent::ExecTaskOutput output;
		output.set_kind(agent::EXEC_TASK_OUTPUT_ERROR);
		output.set_error("task timed out after " + std::to_string(timeout_seconds) + " seconds");
		writer->Write(output);
	}

	agent::ExecTaskOutput exit_output;
	exit_output.set_kind(agent::EXEC_TASK_OUTPUT_EXIT);
	if (WIFEXITED(status)) {
		exit_output.set_exit_code(WEXITSTATUS(status));
	} else if (WIFSIGNALED(status)) {
		exit_output.set_exit_code(128 + WTERMSIG(status));
	} else {
		exit_output.set_exit_code(-1);
	}
	writer->Write(exit_output);

	reclaim_own_page_cache();
	return grpc::Status::OK;
}

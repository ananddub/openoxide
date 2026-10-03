#include "services/shell_service.h"

#include <fmt/format.h>
#include <malloc.h>

#include <asio/redirect_error.hpp>
#include <asio/posix/stream_descriptor.hpp>
#include <asio/steady_timer.hpp>
#include <asio/use_awaitable.hpp>
#include <asio/write.hpp>

#include <fcntl.h>
#include <pty.h>
#include <signal.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include <termios.h>
#include <unistd.h>

#include <algorithm>
#include <array>
#include <chrono>
#include <cstdlib>
#include <cstring>
#include <nlohmann/json.hpp>
#include <vector>

#include "docker/hubs.h"

namespace {

constexpr auto kPoll = std::chrono::milliseconds(50);

} // namespace

void reclaim_own_page_cache() {
	static const char* paths[] = {"/sys/fs/cgroup/memory.reclaim", "/sys/fs/cgroup/memory/memory.reclaim"};
	for (const char* path : paths) {
		if (access(path, W_OK) == 0) {
			int fd = ::open(path, O_WRONLY);
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

namespace {

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

int exit_code(int status) {
	if (WIFEXITED(status)) {
		return WEXITSTATUS(status);
	}
	if (WIFSIGNALED(status)) {
		return 128 + WTERMSIG(status);
	}
	return 0;
}

} // namespace

// Shared per-shell state. Everything is touched from the io thread only: the
// cancel hook re-posts onto it before calling abort().
struct ShellState {
	explicit ShellState(asio::any_io_executor ex) : master(ex), join(ex) {}

	asio::any_io_executor ex;
	std::shared_ptr<Mailbox<std::shared_ptr<agent::ShellMessage>>> box;
	int child = -1;
	asio::posix::stream_descriptor master;
	asio::steady_timer join;
	bool waiting = false;
	int live = 0;
	bool stopped = false;

	void abort() {
		stopped = true;
		if (child > 0) {
			kill(child, SIGTERM);
		}
		master.cancel();
		if (box) {
			box->close();
		}
	}

	void leave() {
		if (--live <= 0 && waiting) {
			join.cancel();
		}
	}
};

namespace {

// Runs st->abort() + st->leave() when a loop coroutine unwinds, whichever way
// it ends — the joiner in run_shell depends on exactly two leaves.
struct LoopExit {
	ShellState& state;
	~LoopExit() {
		state.abort();
		state.leave();
	}
};

agent::ShellMessage stdout_frame(std::string data) {
	agent::ShellMessage message;
	*message.mutable_stdout()->mutable_data() = std::move(data);
	return message;
}

agent::ShellMessage exit_frame(int code) {
	agent::ShellMessage message;
	message.mutable_exit()->set_exit_code(code);
	return message;
}

agent::ShellMessage error_frame(const std::string& text) {
	agent::ShellMessage message;
	message.mutable_error()->set_message(text);
	return message;
}

} // namespace

grpc::ServerBidiReactor<agent::ShellMessage, agent::ShellMessage>* ShellService::open(
    grpc::CallbackServerContext* /*context*/) {
	auto pilot = std::make_shared<CallPilot>(ex_);
	auto box = std::make_shared<ShellReactor::Box>(ex_);
	auto state = std::make_shared<ShellState>(ex_);
	state->box = box;
	auto* reactor = new ShellReactor(pilot, box, [state, ex = ex_] {
		// OnCancel runs on an EventEngine thread; teardown is io-thread work.
		asio::post(ex, [state] { state->abort(); });
	});
	asio::co_spawn(
	    ex_,
	    [this, reactor, state]() -> asio::awaitable<void> {
		    grpc::Status status = grpc::Status::OK;
		    try {
			    status = co_await run_shell(reactor, state);
		    } catch (const std::exception& error) {
			    status = grpc::Status(grpc::StatusCode::INTERNAL, error.what());
		    }
		    reactor->finish(status);
	    },
	    asio::detached);
	return reactor;
}

asio::awaitable<grpc::Status> ShellService::run_shell(ShellReactor* reactor,
                                                      std::shared_ptr<ShellState> state) {
	agent::ShellOpen open;
	{
		auto first = co_await state->box->next();
		if (!first || !(*first)->has_open()) {
			co_return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "missing shell open message");
		}
		open = (*first)->open();
	}
	if (open.server_id() <= 0 || (config_.server_id != -1 && open.server_id() != config_.server_id)) {
		co_return grpc::Status(grpc::StatusCode::NOT_FOUND, "shell server_id does not match this agent");
	}

	std::vector<std::string> argv;
	const std::string shell = open.shell().empty() ? "sh" : open.shell();
	if (open.host()) {
		// Host escape: nsenter into PID 1's namespaces when the agent shares
		// the host PID namespace; otherwise a direct host shell.
		if (access("/usr/bin/nsenter", X_OK) == 0) {
			argv = {"nsenter", "-t", "1", "-m", "-u", "-i", "-n", "-p", "--", shell};
		} else {
			argv = {shell};
		}
	} else if (!open.container().empty()) {
		const std::string target = co_await resolve_container(open.container());
		argv = {"docker", "exec", "-i", "-t", target, shell};
	} else {
		argv = {shell};
	}

	PtySession session = spawn_pty(argv, "");
	if (session.master < 0) {
		co_await reactor->write(error_frame("could not start shell"));
		co_return grpc::Status::OK;
	}
	state->child = session.child;
	std::error_code ec;
	state->master.assign(session.master, ec);
	if (ec) {
		close(session.master);
		kill(session.child, SIGKILL);
		waitpid(session.child, nullptr, 0);
		co_await reactor->write(error_frame("could not start shell"));
		co_return grpc::Status::OK;
	}
	pty_resize(session.master, open.cols(), open.rows());

	state->live = 2;
	asio::co_spawn(ex_, shell_output_loop(reactor, state), asio::detached);
	asio::co_spawn(ex_, shell_input_loop(reactor, state), asio::detached);

	state->waiting = true;
	while (state->live > 0) {
		state->join.expires_after(std::chrono::hours(1));
		try {
			co_await state->join.async_wait(asio::use_awaitable);
		} catch (const asio::system_error&) {
		}
	}

	// Reap for good (sync parity: the child is SIGTERMed, then SIGKILLed).
	if (state->child > 0) {
		kill(state->child, SIGTERM);
		int status = 0;
		pid_t done = 0;
		asio::steady_timer poll(ex_);
		for (int i = 0; i < 40; ++i) {
			done = waitpid(state->child, &status, WNOHANG);
			if (done == state->child) {
				break;
			}
			poll.expires_after(kPoll);
			co_await poll.async_wait(asio::redirect_error(asio::use_awaitable, ec));
		}
		if (done != state->child) {
			kill(state->child, SIGKILL);
			waitpid(state->child, &status, 0);
		}
		state->child = -1;
	}
	state->master.close();
	co_return grpc::Status::OK;
}

// pty -> client. Mirrors the sync select loop: readable events, WNOHANG reap
// with an exit frame, EIO/EOF ends the session.
asio::awaitable<void> ShellService::shell_output_loop(ShellReactor* reactor,
                                                      std::shared_ptr<ShellState> state) {
	LoopExit exit_guard{*state};
	std::array<char, 4096> buffer{};
	while (!state->stopped && !reactor->cancelled()) {
		asio::error_code ec;
		co_await state->master.async_wait(asio::posix::stream_descriptor::wait_read,
		                                  asio::redirect_error(asio::use_awaitable, ec));
		if (ec) {
			break;
		}
		const ssize_t n = ::read(state->master.native_handle(), buffer.data(), buffer.size());
		if (n > 0) {
			if (!co_await reactor->write(stdout_frame(std::string(buffer.data(), static_cast<size_t>(n))))) {
				break;
			}
		} else if (n < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
			continue;
		} else {
			break; // EOF/EIO: the child is gone or the master was closed
		}
		if (state->child > 0) {
			int status = 0;
			const pid_t done = waitpid(state->child, &status, WNOHANG);
			if (done == state->child) {
				state->child = -1;
				co_await reactor->write(exit_frame(exit_code(status)));
				break;
			}
		}
	}
}

// client -> pty: stdin bytes and window resizes; close (or stream end) stops
// the session, which LoopExit turns into a child kill.
asio::awaitable<void> ShellService::shell_input_loop(ShellReactor* /*reactor*/,
                                                     std::shared_ptr<ShellState> state) {
	LoopExit exit_guard{*state};
	while (!state->stopped) {
		auto item = co_await state->box->next();
		if (!item) {
			break;
		}
		const agent::ShellMessage& message = **item;
		if (message.has_input()) {
			const std::string& data = message.input().data();
			if (!data.empty() && ::write(state->master.native_handle(), data.data(), data.size()) < 0) {
				break;
			}
		} else if (message.has_resize()) {
			pty_resize(state->master.native_handle(), message.resize().cols(), message.resize().rows());
		} else if (message.has_close()) {
			break;
		}
	}
}

// open_container(): exact compose-service label match wins, then name
// substring hits (including the "_db"/"-db" cleanup). When Docker is
// unreachable the raw name is used and `docker exec` reports the error.
asio::awaitable<std::string> ShellService::resolve_container(const std::string& requested) {
	std::vector<ContainerSummary> containers;
	try {
		containers = co_await ContainerSummary::list(docker_, true);
	} catch (const std::exception&) {
		co_return requested;
	}
	std::string search = requested;
	std::transform(search.begin(), search.end(), search.begin(),
	               [](unsigned char c) { return std::tolower(c); });
	std::string clean = search;
	if (clean.ends_with("_db") || clean.ends_with("-db")) {
		clean.resize(clean.size() - 3);
	}
	const ContainerSummary* found = nullptr;
	for (const auto& container : containers) {
		std::string label = container.label("com.docker.compose.service");
		std::transform(label.begin(), label.end(), label.begin(),
		               [](unsigned char c) { return std::tolower(c); });
		if (!label.empty() && label == search) {
			found = &container;
			break;
		}
	}
	if (found == nullptr) {
		for (const auto& container : containers) {
			std::string name = container.name;
			std::transform(name.begin(), name.end(), name.begin(),
			               [](unsigned char c) { return std::tolower(c); });
			const bool hit = name.find(search) != std::string::npos || name.rfind(search, 0) == 0 ||
			                 name.find(search + "_") != std::string::npos ||
			                 name.find(search + ".") != std::string::npos ||
			                 (!clean.empty() && (name.find(clean) != std::string::npos ||
			                                      name.rfind(clean, 0) == 0));
			if (hit) {
				found = &container;
				break;
			}
		}
	}
	if (found == nullptr) {
		co_return requested;
	}
	co_return found->id.empty() ? found->name : found->id;
}

grpc::ServerWriteReactor<agent::ExecTaskOutput>* ShellService::start_exec_task(
    grpc::CallbackServerContext* /*context*/, const agent::ExecTaskRequest* request) {
	return start_write_stream<agent::ExecTaskOutput>(
	    ex_, [this, request](auto* reactor) { return run_exec_task(*request, reactor); });
}

namespace {

struct ExecState {
	std::atomic<bool> stopped{false};
	int pumps_total = 0;
	std::atomic<int> pumps_done{0}; // pumps finish on the io thread via leave()
};

// One pipe-fd pump: readable -> read -> mailbox. Runs on the io thread.
asio::awaitable<void> pump_pipe(asio::any_io_executor ex, int fd, agent::ExecTaskOutputKind kind,
                                std::shared_ptr<Mailbox<agent::ExecTaskOutput>> box,
                                std::shared_ptr<ExecState> state, int pumps_to_wait) {
	struct PumpExit {
		std::shared_ptr<ExecState> state;
		int total;
		~PumpExit() {
			if (++state->pumps_done >= total) {
				state->stopped = true; // unblock the sibling pump's waits
			}
		}
	} exit_guard{state, pumps_to_wait};
	asio::posix::stream_descriptor pipe(ex, fd);
	std::array<char, 4096> buffer{};
	while (!state->stopped) {
		asio::error_code ec;
		co_await pipe.async_wait(asio::posix::stream_descriptor::wait_read,
		                         asio::redirect_error(asio::use_awaitable, ec));
		if (ec) {
			break;
		}
		const ssize_t n = ::read(pipe.native_handle(), buffer.data(), buffer.size());
		if (n > 0) {
			agent::ExecTaskOutput output;
			output.set_kind(kind);
			output.set_data(buffer.data(), static_cast<size_t>(n));
			box->push(std::move(output));
		} else if (n == 0) {
			break;
		} else if (errno != EAGAIN && errno != EWOULDBLOCK) {
			break;
		}
	}
}

asio::awaitable<void> feed_stdin(asio::any_io_executor ex, int fd, std::string data,
                                 std::shared_ptr<ExecState> state, int pumps_to_wait) {
	struct PumpExit {
		std::shared_ptr<ExecState> state;
		int total;
		~PumpExit() {
			if (++state->pumps_done >= total) {
				state->stopped = true;
			}
		}
	} exit_guard{state, pumps_to_wait};
	asio::posix::stream_descriptor pipe(ex, fd);
	asio::error_code ec;
	co_await asio::async_write(pipe, asio::buffer(data), asio::redirect_error(asio::use_awaitable, ec));
	pipe.close(); // EOF for the child
}

} // namespace

asio::awaitable<grpc::Status> ShellService::run_exec_task(agent::ExecTaskRequest request,
                                                          TaskReactor* reactor) {
	if (request.server_id() <= 0 || (config_.server_id != -1 && request.server_id() != config_.server_id)) {
		co_return grpc::Status(grpc::StatusCode::NOT_FOUND, "task server_id does not match this agent");
	}
	if (request.command().empty()) {
		co_return grpc::Status(grpc::StatusCode::INVALID_ARGUMENT, "task command cannot be empty");
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

	// Pipes (no PTY) keep stdout and stderr separated, like the Rust path.
	std::array<int, 2> out_pipe{};
	std::array<int, 2> err_pipe{};
	std::array<int, 2> in_pipe{};
	if (pipe(out_pipe.data()) != 0 || pipe(err_pipe.data()) != 0) {
		co_return grpc::Status(grpc::StatusCode::INTERNAL, "could not create task pipes");
	}
	const bool has_stdin = !request.stdin().empty();
	if (has_stdin && pipe(in_pipe.data()) != 0) {
		close(out_pipe[0]);
		close(out_pipe[1]);
		close(err_pipe[0]);
		close(err_pipe[1]);
		co_return grpc::Status(grpc::StatusCode::INTERNAL, "could not create task stdin pipe");
	}

	const pid_t child = fork();
	if (child < 0) {
		co_return grpc::Status(grpc::StatusCode::INTERNAL, "fork failed");
	}
	if (child == 0) {
		dup2(out_pipe[1], STDOUT_FILENO);
		dup2(err_pipe[1], STDERR_FILENO);
		if (has_stdin) {
			dup2(in_pipe[0], STDIN_FILENO);
		} else {
			int null_fd = ::open("/dev/null", O_RDONLY);
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
		if (request.container().empty() && !request.working_directory().empty()) {
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

	const auto box = std::make_shared<Mailbox<agent::ExecTaskOutput>>(ex_);
	auto state = std::make_shared<ExecState>();
	state->pumps_total = has_stdin ? 3 : 2;
	asio::co_spawn(ex_, pump_pipe(ex_, out_pipe[0], agent::EXEC_TASK_OUTPUT_STDOUT, box, state, state->pumps_total),
	               asio::detached);
	asio::co_spawn(ex_, pump_pipe(ex_, err_pipe[0], agent::EXEC_TASK_OUTPUT_STDERR, box, state, state->pumps_total),
	               asio::detached);
	if (has_stdin) {
		asio::co_spawn(ex_, feed_stdin(ex_, in_pipe[1], request.stdin(), state, state->pumps_total),
		               asio::detached);
	} else {
		close(in_pipe[1]);
	}

	const uint32_t timeout_seconds = request.timeout_seconds();
	const auto deadline = std::chrono::steady_clock::now() +
		std::chrono::seconds(timeout_seconds == 0 ? 31536000 : timeout_seconds);
	bool child_done = false;
	bool timed_out = false;
	int status = 0;
	asio::error_code ec;

	// The mailbox poll doubles as the 50 ms waitpid tick; every output frame
	// goes through this coroutine, so the reactor sees one writer at a time.
	while (!child_done || state->pumps_done < state->pumps_total) {
		auto item = co_await box->next_within(kPoll);
		if (item && !co_await reactor->write(std::move(*item))) {
			state->stopped = true; // client gone: stop pumping, kill below
		}
		if (!child_done) {
			const pid_t done = waitpid(child, &status, WNOHANG);
			if (done == child || done < 0) {
				child_done = true;
			} else if (state->stopped || reactor->cancelled()) {
				kill(child, SIGKILL);
				waitpid(child, &status, 0);
				child_done = true;
			} else if (std::chrono::steady_clock::now() > deadline) {
				timed_out = true;
				kill(child, SIGKILL);
				waitpid(child, &status, 0);
				child_done = true;
			}
		}
	}

	// Last frames the pumps pushed just before finishing.
	{
		agent::ExecTaskOutput output;
		while (box->try_pop(output)) {
			co_await reactor->write(std::move(output));
		}
	}

	if (timed_out) {
		agent::ExecTaskOutput output;
		output.set_kind(agent::EXEC_TASK_OUTPUT_ERROR);
		output.set_error(fmt::format("task timed out after {} seconds", timeout_seconds));
		co_await reactor->write(std::move(output));
	}
	agent::ExecTaskOutput exit_output;
	exit_output.set_kind(agent::EXEC_TASK_OUTPUT_EXIT);
	exit_output.set_exit_code(child_done && (WIFEXITED(status) || WIFSIGNALED(status))
	                              ? exit_code(status)
	                              : -1);
	co_await reactor->write(std::move(exit_output));

	reclaim_own_page_cache();
	co_return grpc::Status::OK;
}

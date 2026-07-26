use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use zeroize::Zeroize;

pub struct SshAgentSession {
	pub socket_path: PathBuf,
	pub pid: u32,
}

impl SshAgentSession {
	pub async fn start_and_add_key(private_key: &str) -> Result<Self, String> {
		let output = Command::new("ssh-agent")
			.arg("-s")
			.output()
			.await
			.map_err(|e| format!("failed to execute ssh-agent: {e}"))?;

		if !output.status.success() {
			return Err("ssh-agent failed to start".into());
		}

		let out_str = String::from_utf8_lossy(&output.stdout);
		let mut socket_path: Option<PathBuf> = None;
		let mut pid: Option<u32> = None;

		for line in out_str.lines() {
			if line.starts_with("SSH_AUTH_SOCK=") {
				if let Some(val) = line.split(';').next() {
					let path_str = val.trim_start_matches("SSH_AUTH_SOCK=");
					socket_path = Some(PathBuf::from(path_str));
				}
			} else if line.starts_with("SSH_AGENT_PID=") {
				if let Some(val) = line.split(';').next() {
					let pid_str = val.trim_start_matches("SSH_AGENT_PID=");
					if let Ok(parsed_pid) = pid_str.parse::<u32>() {
						pid = Some(parsed_pid);
					}
				}
			}
		}

		let socket_path = socket_path.ok_or_else(|| "could not parse SSH_AUTH_SOCK from ssh-agent output".to_string())?;
		let pid = pid.ok_or_else(|| "could not parse SSH_AGENT_PID from ssh-agent output".to_string())?;

		let mut key_buf = private_key.to_string();
		if !key_buf.ends_with('\n') {
			key_buf.push('\n');
		}

		// Pin RAM memory pages with mlock so OS cannot write key bytes to swap disk
		#[cfg(unix)]
		unsafe {
			let ptr = key_buf.as_ptr() as *const std::ffi::c_void;
			let len = key_buf.len();
			if len > 0 {
				let _ = libc::mlock(ptr, len);
			}
		}

		let mut add_cmd = Command::new("ssh-add");
		add_cmd.arg("-");
		add_cmd.env("SSH_AUTH_SOCK", &socket_path);
		add_cmd.stdin(std::process::Stdio::piped());
		add_cmd.stdout(std::process::Stdio::piped());
		add_cmd.stderr(std::process::Stdio::piped());

		let mut child = add_cmd.spawn().map_err(|e| {
			key_buf.zeroize();
			format!("failed to spawn ssh-add: {e}")
		})?;

		if let Some(mut stdin) = child.stdin.take() {
			let write_res = stdin.write_all(key_buf.as_bytes()).await;
			let flush_res = stdin.flush().await;

			#[cfg(unix)]
			unsafe {
				let ptr = key_buf.as_ptr() as *const std::ffi::c_void;
				let len = key_buf.len();
				if len > 0 {
					let _ = libc::munlock(ptr, len);
				}
			}

			key_buf.zeroize();

			write_res.map_err(|e| format!("failed to write key to ssh-add stdin: {e}"))?;
			flush_res.map_err(|e| format!("failed to flush ssh-add stdin: {e}"))?;
		} else {
			key_buf.zeroize();
		}

		let add_output = child.wait_with_output().await.map_err(|e| format!("failed to wait on ssh-add: {e}"))?;
		if !add_output.status.success() {
			let err_msg = String::from_utf8_lossy(&add_output.stderr);
			return Err(format!("ssh-add failed to load private key from stdin: {}", err_msg.trim()));
		}

		Ok(Self { socket_path, pid })
	}
}

impl Drop for SshAgentSession {
	fn drop(&mut self) {
		let pid = self.pid;
		let socket_path = self.socket_path.clone();
		tokio::spawn(async move {
			// Graceful ssh-agent shutdown (-k)
			let _ = Command::new("ssh-agent")
				.arg("-k")
				.env("SSH_AUTH_SOCK", &socket_path)
				.env("SSH_AGENT_PID", pid.to_string())
				.output()
				.await;
			let _ = Command::new("kill").arg(pid.to_string()).output().await;
		});
	}
}

// Clean up any stale orphaned /tmp/ssh-* socket folders on server boot-up
pub fn sweep_orphaned_agent_sockets() {
	if let Ok(entries) = std::fs::read_dir("/tmp") {
		for entry in entries.flatten() {
			let path = entry.path();
			if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
				if name.starts_with("ssh-") && path.is_dir() {
					let _ = std::fs::remove_dir_all(&path);
				}
			}
		}
	}
}

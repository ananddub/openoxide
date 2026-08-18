use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use crate::exec::{ExecResult, SshAuth, SshHostKey};
use super::russh_client::{RusshSession, connect_russh};

static SESSION_POOL: OnceLock<Mutex<HashMap<String, Arc<RusshSession>>>> = OnceLock::new();

fn session_pool() -> &'static Mutex<HashMap<String, Arc<RusshSession>>> {
    SESSION_POOL.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn evict_session(host: &str, port: u16, username: &str) {
    let key = format!("{username}@{host}:{port}");
    if let Ok(mut pool) = session_pool().lock() {
        pool.remove(&key);
    }
}

pub async fn get_or_connect_session(
    host: &str,
    port: u16,
    username: &str,
    auth: &SshAuth,
    host_key: &SshHostKey,
    connect_timeout: Duration,
) -> ExecResult<Arc<RusshSession>> {
    let key = format!("{username}@{host}:{port}");

    // 1. Try to reuse an active pooled session
    {
        let pool = session_pool().lock().unwrap();
        if let Some(session) = pool.get(&key) {
            if !session.is_closed() {
                return Ok(session.clone());
            }
        }
    }

    // 2. Connect new session if none exists or previous was closed
    let session = Arc::new(connect_russh(host, port, username, auth, host_key, connect_timeout).await?);

    // 3. Store active session handle in pool for multiplexing
    {
        let mut pool = session_pool().lock().unwrap();
        pool.insert(key, session.clone());
    }

    Ok(session)
}

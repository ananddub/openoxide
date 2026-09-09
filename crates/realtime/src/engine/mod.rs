use std::ffi::{c_char, c_int, c_void, CStr};
use std::collections::HashMap;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use tokio::io::unix::AsyncFd;
use tokio::sync::broadcast;

use crate::c_ffi::sqlite3_realtime_attach_if_needed;

use crate::c_ffi::{
    sqlite3_realtime_cleanup, sqlite3_realtime_drain, sqlite3_realtime_init, RealtimeOp,
    RealtimeRawEvent,
};

struct DrainContext {
    events: Vec<RealtimeRawEvent>,
}

/// Maps a live sqlite3 pointer to its attached handle so idempotent
/// re-attaches reuse the existing reactor instead of spawning a duplicate.
/// Weak refs: when the last pool-side holder drops (connection replaced),
/// the stale entry disappears automatically.
static HANDLES: std::sync::LazyLock<std::sync::Mutex<HashMap<usize, std::sync::Weak<RealtimeHandle>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

fn register_handle(ptr: usize, handle: &Arc<RealtimeHandle>) {
    HANDLES
        .lock()
        .expect("handle registry poisoned")
        .insert(ptr, Arc::downgrade(handle));
}

fn lookup_handle(ptr: usize) -> Option<Arc<RealtimeHandle>> {
    let mut map = HANDLES.lock().expect("handle registry poisoned");
    // Prune dead entries while we're here so a replaced connection's old
    // sqlite3 pointer reuse can't collide with a stale weak.
    map.retain(|_, weak| weak.upgrade().is_some());
    map.get(&ptr).and_then(|weak| weak.upgrade())
}

extern "C" fn drain_gateway(
    op: c_int,
    z_db: *const c_char,
    z_table: *const c_char,
    i_rowid: i64,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let ctx = unsafe { &mut *(user_data as *mut DrainContext) };

    let db_str = if !z_db.is_null() {
        unsafe { CStr::from_ptr(z_db).to_string_lossy().into_owned() }
    } else {
        String::new()
    };

    let table_str = if !z_table.is_null() {
        unsafe { CStr::from_ptr(z_table).to_string_lossy().into_owned() }
    } else {
        String::new()
    };

    ctx.events.push(RealtimeRawEvent {
        op: RealtimeOp::from(op),
        db: db_str,
        table: table_str,
        rowid: i_rowid,
    });
}

pub struct RealtimeHandle {
    db_handle: *mut c_void,
    event_fd: RawFd,
    bus: broadcast::Sender<RealtimeRawEvent>,
    _reactor_task: tokio::task::JoinHandle<()>,
}

unsafe impl Send for RealtimeHandle {}
unsafe impl Sync for RealtimeHandle {}

impl RealtimeHandle {
    /// Attach the realtime C engine to an active SQLite raw handle (*mut sqlite3)
    pub fn attach(raw_sqlite_ptr: *mut c_void) -> Result<Arc<Self>, String> {
        let mut out_fd: c_int = -1;
        let rc = unsafe { sqlite3_realtime_init(raw_sqlite_ptr, &mut out_fd) };
        if rc != 0 {
            return Err(format!("sqlite3_realtime_init failed with code {}", rc));
        }

        let event_fd = out_fd as RawFd;
        let (bus, _) = broadcast::channel(1024);

        let bus_clone = bus.clone();
        let db_ptr_usize = raw_sqlite_ptr as usize;

        // The eventfd can be stale when the reactor starts (sqlx replaced the
        // connection between attach and spawn, leaving the fd invalid).
        // Register with retries: on failure, cleanup + fresh init gives a
        // new eventfd which we register until AsyncFd accepts it.
        let reactor_task = tokio::spawn(async move {
            let mut current_fd = event_fd;
            // Bounded retries: each pass cleans up the stale context and
            // re-inits, which yields a fresh eventfd (often the same fd
            // number after fd reuse — still valid to register because the
            // cleanup closed the old one). Give up after a few passes to
            // avoid a hot loop on a genuinely broken fd.
            let async_fd = loop {
                match AsyncFd::new(current_fd) {
                    Ok(fd) => break fd,
                    Err(error) => {
                        if current_fd != event_fd {
                            // A re-init'd fd also failed — the connection
                            // itself is broken; don't spin forever.
                            tracing::error!(
                                error = %error,
                                fd = current_fd,
                                "re-initialized eventfd also failed, reactor exiting"
                            );
                            return;
                        }
                        tracing::warn!(
                            error = %error,
                            db_ptr = db_ptr_usize,
                            "eventfd registration failed, re-initializing realtime hooks"
                        );
                        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                        let db_ptr = db_ptr_usize as *mut c_void;
                        unsafe { sqlite3_realtime_cleanup(db_ptr) };
                        let mut fresh_fd: c_int = -1;
                        let rc = unsafe { sqlite3_realtime_init(db_ptr, &mut fresh_fd) };
                        if rc != 0 {
                            tracing::error!(rc, "realtime re-init failed, reactor exiting");
                            return;
                        }
                        current_fd = fresh_fd;
                    }
                }
            };

            loop {
                let mut guard = match async_fd.readable().await {
                    Ok(g) => g,
                    Err(_) => break,
                };

                let mut drain_ctx = DrainContext {
                    events: Vec::with_capacity(32),
                };

                let db_ptr = db_ptr_usize as *mut c_void;
                unsafe {
                    sqlite3_realtime_drain(
                        db_ptr,
                        Some(drain_gateway),
                        &mut drain_ctx as *mut DrainContext as *mut c_void,
                    );
                }

                for ev in drain_ctx.events {
                    let _ = bus_clone.send(ev);
                }

                guard.clear_ready();
            }
        });

        let handle = Arc::new(Self {
            db_handle: raw_sqlite_ptr,
            event_fd,
            bus,
            _reactor_task: reactor_task,
        });
        register_handle(raw_sqlite_ptr as usize, &handle);
        Ok(handle)
    }

    pub fn get_fd(&self) -> RawFd {
        self.event_fd
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RealtimeRawEvent> {
        self.bus.subscribe()
    }

    /// Idempotent attach for pooled connections: attaches the CDC hooks if
    /// the connection doesn't carry them yet and returns a live handle —
    /// existing for an already-attached connection, fresh otherwise. Pool
    /// layers that re-open connections call this from their connect hook so
    /// every physical connection captures changes.
    pub fn attach_if_needed(raw_sqlite_ptr: *mut c_void) -> Result<Arc<Self>, String> {
        // A live handle for this exact connection is reused as-is.
        if let Some(existing) = lookup_handle(raw_sqlite_ptr as usize) {
            return Ok(existing);
        }

        // Stale C-side context from a replaced connection that reused this
        // pointer: clean it up before initializing fresh.
        unsafe { sqlite3_realtime_cleanup(raw_sqlite_ptr) };

        Self::attach(raw_sqlite_ptr)
    }

    /// Return the raw `sqlite3*` handle as a `usize` for safe cross-thread transfer.
    /// Used by `LiveQueryRegistry::register()` to run `EXPLAIN QUERY PLAN` on the
    /// same connection that owns the realtime hooks.
    ///
    /// # Safety
    /// The caller must ensure this pointer is only dereferenced on the thread that
    /// owns the underlying SQLite connection (i.e. inside `analyze_query_tables`).
    pub fn db_ptr(&self) -> usize {
        self.db_handle as usize
    }
}

impl Drop for RealtimeHandle {
    fn drop(&mut self) {
        unsafe {
            sqlite3_realtime_cleanup(self.db_handle);
        }
    }
}

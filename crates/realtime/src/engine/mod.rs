use std::ffi::{c_char, c_int, c_void, CStr};
use std::os::unix::io::RawFd;
use std::sync::Arc;
use tokio::io::unix::AsyncFd;
use tokio::sync::broadcast;

use crate::c_ffi::{
    sqlite3_realtime_cleanup, sqlite3_realtime_drain, sqlite3_realtime_init, RealtimeOp,
    RealtimeRawEvent,
};

struct DrainContext {
    events: Vec<RealtimeRawEvent>,
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

        let reactor_task = tokio::spawn(async move {
            let async_fd = match AsyncFd::new(event_fd) {
                Ok(fd) => fd,
                Err(e) => {
                    tracing::error!("Failed to register eventfd in tokio epoll: {}", e);
                    return;
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

        Ok(Arc::new(Self {
            db_handle: raw_sqlite_ptr,
            event_fd,
            bus,
            _reactor_task: reactor_task,
        }))
    }

    pub fn get_fd(&self) -> RawFd {
        self.event_fd
    }

    pub fn subscribe(&self) -> broadcast::Receiver<RealtimeRawEvent> {
        self.bus.subscribe()
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

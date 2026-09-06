use std::ffi::{c_char, c_int, c_void};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealtimeOp {
    Insert = 18,
    Delete = 9,
    Update = 23,
    Unknown = 0,
}

impl From<c_int> for RealtimeOp {
    fn from(op: c_int) -> Self {
        match op {
            18 => RealtimeOp::Insert,
            9 => RealtimeOp::Delete,
            23 => RealtimeOp::Update,
            _ => RealtimeOp::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeRawEvent {
    pub op: RealtimeOp,
    pub db: String,
    pub table: String,
    pub rowid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedChangeEvent<T> {
    pub op: RealtimeOp,
    pub table: String,
    pub rowid: i64,
    pub data: Option<T>,
}

unsafe extern "C" {
    pub fn sqlite3_realtime_init(db: *mut c_void, p_out_fd: *mut c_int) -> c_int;
    pub fn sqlite3_realtime_attach_if_needed(
        db: *mut c_void,
        p_out_fd: *mut c_int,
        p_already_attached: *mut c_int,
    ) -> c_int;
    pub fn sqlite3_realtime_get_fd(db: *mut c_void) -> c_int;
    pub fn sqlite3_realtime_drain(
        db: *mut c_void,
        cb: Option<extern "C" fn(c_int, *const c_char, *const c_char, i64, *mut c_void)>,
        p_user_data: *mut c_void,
    ) -> c_int;
    pub fn sqlite3_realtime_cleanup(db: *mut c_void) -> c_int;
    pub fn sqlite3_realtime_analyze_query(
        db: *mut c_void,
        sql: *const c_char,
        cb: Option<extern "C" fn(*const c_char, *mut c_void)>,
        p_user: *mut c_void,
    ) -> c_int;
}

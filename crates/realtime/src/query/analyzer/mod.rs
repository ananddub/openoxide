use std::collections::HashSet;
use std::ffi::{c_char, c_void, CStr, CString};

use crate::c_ffi::sqlite3_realtime_analyze_query;

struct TableCollector {
    tables: HashSet<String>,
}

extern "C" fn table_callback(z_table: *const c_char, p_user: *mut c_void) {
    if z_table.is_null() || p_user.is_null() {
        return;
    }
    let collector = unsafe { &mut *(p_user as *mut TableCollector) };
    let table_name = unsafe { CStr::from_ptr(z_table).to_string_lossy().into_owned() };
    if !table_name.is_empty() {
        collector.tables.insert(table_name);
    }
}

/// Analyzes an arbitrary SQL query using SQLite's native `EXPLAIN QUERY PLAN` engine.
/// Returns the exact set of dependent table names discovered by SQLite's optimizer.
pub fn analyze_query_tables(raw_sqlite_ptr: *mut c_void, sql: &str) -> HashSet<String> {
    if raw_sqlite_ptr.is_null() || sql.trim().is_empty() {
        return HashSet::new();
    }

    let Ok(c_sql) = CString::new(sql) else {
        return HashSet::new();
    };

    let mut collector = TableCollector {
        tables: HashSet::new(),
    };

    unsafe {
        sqlite3_realtime_analyze_query(
            raw_sqlite_ptr,
            c_sql.as_ptr(),
            Some(table_callback),
            &mut collector as *mut TableCollector as *mut c_void,
        );
    }

    collector.tables
}

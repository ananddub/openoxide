use std::sync::{Arc, Mutex};
use sqlx::sqlite::{PreupdateHookResult, SqliteOperation};

use crate::db::reactive::bus::publish_when_visible;
use crate::db::reactive::org_index::OrgColumnIndex;
use crate::db::reactive::scope::REQUEST_CHANGES;
use crate::db::reactive::types::{DbChangeEvent, DbChangeOperation};

pub fn install_hooks(handle: &mut sqlx::sqlite::LockedSqliteHandle<'_>) {
    let registry = OrgColumnIndex::global();
    let pending = Arc::new(Mutex::new(Vec::<DbChangeEvent>::new()));
    let runtime = tokio::runtime::Handle::current();

    let preupdate_pending = Arc::clone(&pending);
    handle.set_preupdate_hook(move |result| {
        if let Ok(mut changes) = preupdate_pending.lock() {
            changes.push(change_event(result, registry));
        }
    });

    let commit_pending = Arc::clone(&pending);
    let commit_runtime = runtime.clone();
    handle.set_commit_hook(move || {
        let changes = commit_pending
            .lock()
            .map(|mut changes| std::mem::take(&mut *changes))
            .unwrap_or_default();

        if !changes.is_empty() {
            let queued = REQUEST_CHANGES
                .try_with(|request_changes| {
                    if let Ok(mut request_changes) = request_changes.lock() {
                        request_changes.extend(changes.iter().cloned());
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false);
            if !queued {
                commit_runtime.spawn(publish_when_visible(changes));
            }
        }

        true
    });

    handle.set_rollback_hook(move || {
        if let Ok(mut changes) = pending.lock() {
            changes.clear();
        }
    });
}

fn change_event(result: PreupdateHookResult<'_>, registry: &OrgColumnIndex) -> DbChangeEvent {
    DbChangeEvent {
        table: result.table.to_owned(),
        organization_id: registry.organization_id(&result),
        old_row_id: result.get_old_row_id().ok(),
        new_row_id: result.get_new_row_id().ok(),
        operation: match result.operation {
            SqliteOperation::Insert => DbChangeOperation::Insert,
            SqliteOperation::Update => DbChangeOperation::Update,
            SqliteOperation::Delete => DbChangeOperation::Delete,
            _ => DbChangeOperation::Unknown,
        },
    }
}

use std::collections::HashSet;

use crate::api::connect::generated::openoxide::api::v1::WatchedTable;

/// A batch of table-level notifications destined for one organization.
/// Unknown tables are widened to a full resync (all tables) so schema drift
/// degrades to over-invalidation, never to missed updates.
#[derive(Clone, Debug)]
pub struct TableChangeBatch {
    pub organization_id: i64,
    pub tables: Vec<WatchedTable>,
}

impl TableChangeBatch {
    pub fn all_tables(organization_id: i64) -> Self {
        Self {
            organization_id,
            tables: Vec::new(),
        }
    }
}

/// Collapses consecutive change events into minimal table sets. A burst of
/// commits (a deploy writes the same tables many times) becomes one batch
/// listing each table once.
#[derive(Debug)]
pub struct TableBatcher;

impl TableBatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn batch(
        &self,
        organization_id: i64,
        tables: impl IntoIterator<Item = Option<WatchedTable>>,
        all_tables: &[WatchedTable],
    ) -> Option<TableChangeBatch> {
        let mut resolved: HashSet<WatchedTable> = HashSet::new();
        let mut full_resync = false;
        for table in tables {
            match table {
                Some(table) => {
                    resolved.insert(table);
                }
                // Unmapped table: safest interpretation is that everything
                // the client might show is stale.
                None => full_resync = true,
            }
        }
        if full_resync {
            return Some(TableChangeBatch {
                organization_id,
                tables: all_tables.to_vec(),
            });
        }
        if resolved.is_empty() {
            return None;
        }
        Some(TableChangeBatch {
            organization_id,
            tables: resolved.into_iter().collect(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbChangeOperation {
    Insert,
    Update,
    Delete,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbChangeEvent {
    pub table: String,
    pub organization_id: Option<i64>,
    pub old_row_id: Option<i64>,
    pub new_row_id: Option<i64>,
    pub operation: DbChangeOperation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbChangeSet {
    pub changes: Vec<DbChangeEvent>,
}

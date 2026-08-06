#[derive(Debug, Clone)]
pub struct GlobalSearchOptions {
    pub query: String,
    pub limit: i64,
}

#[derive(Debug, Clone, serde::Serialize, poem_openapi::Object)]
pub struct GlobalResourceDto {
    pub resource_type: String,
    pub id: String,
    pub name: String,
    pub status: Option<String>,
}

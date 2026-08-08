use serde::Serialize;

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DependencyStatusDto {
    pub name: String,
    pub healthy: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct SystemHealthDto {
    pub healthy: bool,
    pub dependencies: Vec<DependencyStatusDto>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ConfigTestDto {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

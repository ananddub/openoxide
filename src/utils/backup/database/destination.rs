use crate::utils::rclone::RcloneTarget;

#[derive(Debug, Clone)]
pub struct S3Destination {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub provider: Option<String>,
}

impl S3Destination {
    pub fn to_rclone_target(&self, path: &str) -> RcloneTarget {
        RcloneTarget::S3 {
            provider: self.provider.clone().unwrap_or_else(|| "AWS".to_string()),
            access_key_id: self.access_key.clone(),
            secret_access_key: self.secret_key.clone(),
            bucket: self.bucket.clone(),
            region: self.region.clone(),
            endpoint: self.endpoint.clone(),
            path: path.to_string(),
            force_path_style: true,
            no_check_bucket: true,
        }
    }
}

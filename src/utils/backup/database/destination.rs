use crate::utils::rclone::RcloneTarget;
use os::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum DestinationProvider {
        default = S3;

        S3 => "S3",
        Aws => "AWS",
        R2 => "R2",
        CloudflareR2 => "CLOUDFLARE_R2",
        Backblaze => "BACKBLAZE",
        B2 => "B2",
        Gcs => "GCS",
        Gcp => "GCP",
        Google => "GOOGLE",
        DoSpaces => "DO_SPACES",
        DigitalOcean => "DIGITALOCEAN",
        Spaces => "SPACES",
    }
}

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
        let provider = DestinationProvider::from(self.provider.as_deref().unwrap_or_default());
        let rclone_provider = match provider {
            DestinationProvider::S3 | DestinationProvider::Aws
                if self.endpoint.trim().is_empty() =>
            {
                "AWS"
            }
            DestinationProvider::R2 | DestinationProvider::CloudflareR2 => "Cloudflare",
            DestinationProvider::DoSpaces
            | DestinationProvider::DigitalOcean
            | DestinationProvider::Spaces => "DigitalOcean",
            DestinationProvider::Backblaze
            | DestinationProvider::B2
            | DestinationProvider::Gcs
            | DestinationProvider::Gcp
            | DestinationProvider::Google => "Other",
            DestinationProvider::S3 | DestinationProvider::Aws => "Other",
        };
        RcloneTarget::S3 {
            provider: rclone_provider.to_string(),
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

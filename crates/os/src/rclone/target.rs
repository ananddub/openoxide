use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RcloneTarget {
    Local {
        path: String,
    },
    S3 {
        provider: String,
        access_key_id: String,
        secret_access_key: String,
        bucket: String,
        region: String,
        endpoint: String,
        path: String,
        force_path_style: bool,
        no_check_bucket: bool,
    },
    Sftp {
        host: String,
        port: Option<u16>,
        user: String,
        pass: Option<String>,
        key_file: Option<String>,
        key_use_agent: bool,
        path: String,
    },
    Ftp {
        host: String,
        port: Option<u16>,
        user: String,
        pass: String,
        path: String,
        tls: bool,
    },
    B2 {
        account_id: String,
        application_key: String,
        bucket: String,
        path: String,
    },
    Gcs {
        service_account_credentials: Option<String>,
        bucket: String,
        path: String,
        project_number: Option<String>,
    },
    AzureBlob {
        account: String,
        key: String,
        container: String,
        path: String,
    },
    Webdav {
        url: String,
        user: String,
        pass: String,
        vendor: Option<String>,
        path: String,
    },
    Dropbox {
        token: String,
        path: String,
    },
    GoogleDrive {
        client_id: Option<String>,
        client_secret: Option<String>,
        token: String,
        path: String,
    },
}

impl RcloneTarget {
    pub fn compile(&self, prefix: &str) -> (String, HashMap<String, String>) {
        let mut envs = HashMap::new();
        match self {
            Self::Local { path } => (path.clone(), envs),
            Self::S3 {
                provider,
                access_key_id,
                secret_access_key,
                bucket,
                region,
                endpoint,
                path,
                force_path_style,
                no_check_bucket,
            } => {
                let name = format!("{}_s3", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "s3".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_PROVIDER", name.to_uppercase()),
                    provider.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_ACCESS_KEY_ID", name.to_uppercase()),
                    access_key_id.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_SECRET_ACCESS_KEY", name.to_uppercase()),
                    secret_access_key.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_REGION", name.to_uppercase()),
                    region.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_ENDPOINT", name.to_uppercase()),
                    endpoint.clone(),
                );
                if *force_path_style {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_FORCE_PATH_STYLE", name.to_uppercase()),
                        "true".to_string(),
                    );
                }
                if *no_check_bucket {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_NO_CHECK_BUCKET", name.to_uppercase()),
                        "true".to_string(),
                    );
                }
                (
                    format!("{}:{}/{}", name, bucket, path.trim_start_matches('/')),
                    envs,
                )
            }
            Self::Sftp {
                host,
                port,
                user,
                pass,
                key_file,
                key_use_agent,
                path,
            } => {
                let name = format!("{}_sftp", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "sftp".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_HOST", name.to_uppercase()),
                    host.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_USER", name.to_uppercase()),
                    user.clone(),
                );
                if let Some(p) = port {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_PORT", name.to_uppercase()),
                        p.to_string(),
                    );
                }
                if let Some(ps) = pass {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_PASS", name.to_uppercase()),
                        obscure_password(ps),
                    );
                }
                if let Some(kf) = key_file {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_KEY_FILE", name.to_uppercase()),
                        kf.clone(),
                    );
                }
                if *key_use_agent {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_KEY_USE_AGENT", name.to_uppercase()),
                        "true".to_string(),
                    );
                }
                (format!("{}:{}", name, path), envs)
            }
            Self::Ftp {
                host,
                port,
                user,
                pass,
                path,
                tls,
            } => {
                let name = format!("{}_ftp", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "ftp".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_HOST", name.to_uppercase()),
                    host.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_USER", name.to_uppercase()),
                    user.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_PASS", name.to_uppercase()),
                    obscure_password(pass),
                );
                if let Some(p) = port {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_PORT", name.to_uppercase()),
                        p.to_string(),
                    );
                }
                if *tls {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_TLS", name.to_uppercase()),
                        "true".to_string(),
                    );
                }
                (format!("{}:{}", name, path), envs)
            }
            Self::B2 {
                account_id,
                application_key,
                bucket,
                path,
            } => {
                let name = format!("{}_b2", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "b2".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_ACCOUNT", name.to_uppercase()),
                    account_id.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_KEY", name.to_uppercase()),
                    application_key.clone(),
                );
                (
                    format!("{}:{}/{}", name, bucket, path.trim_start_matches('/')),
                    envs,
                )
            }
            Self::Gcs {
                service_account_credentials,
                bucket,
                path,
                project_number,
            } => {
                let name = format!("{}_gcs", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "google cloud storage".to_string(),
                );
                if let Some(cred) = service_account_credentials {
                    envs.insert(
                        format!(
                            "RCLONE_CONFIG_{}_SERVICE_ACCOUNT_CREDENTIALS",
                            name.to_uppercase()
                        ),
                        cred.clone(),
                    );
                }
                if let Some(pn) = project_number {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_PROJECT_NUMBER", name.to_uppercase()),
                        pn.clone(),
                    );
                }
                (
                    format!("{}:{}/{}", name, bucket, path.trim_start_matches('/')),
                    envs,
                )
            }
            Self::AzureBlob {
                account,
                key,
                container,
                path,
            } => {
                let name = format!("{}_azureblob", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "azureblob".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_ACCOUNT", name.to_uppercase()),
                    account.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_KEY", name.to_uppercase()),
                    key.clone(),
                );
                (
                    format!("{}:{}/{}", name, container, path.trim_start_matches('/')),
                    envs,
                )
            }
            Self::Webdav {
                url,
                user,
                pass,
                vendor,
                path,
            } => {
                let name = format!("{}_webdav", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "webdav".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_URL", name.to_uppercase()),
                    url.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_USER", name.to_uppercase()),
                    user.clone(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_PASS", name.to_uppercase()),
                    obscure_password(pass),
                );
                if let Some(v) = vendor {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_VENDOR", name.to_uppercase()),
                        v.clone(),
                    );
                }
                (format!("{}:{}", name, path), envs)
            }
            Self::Dropbox { token, path } => {
                let name = format!("{}_dropbox", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "dropbox".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TOKEN", name.to_uppercase()),
                    token.clone(),
                );
                (format!("{}:{}", name, path), envs)
            }
            Self::GoogleDrive {
                client_id,
                client_secret,
                token,
                path,
            } => {
                let name = format!("{}_drive", prefix);
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TYPE", name.to_uppercase()),
                    "drive".to_string(),
                );
                envs.insert(
                    format!("RCLONE_CONFIG_{}_TOKEN", name.to_uppercase()),
                    token.clone(),
                );
                if let Some(cid) = client_id {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_CLIENT_ID", name.to_uppercase()),
                        cid.clone(),
                    );
                }
                if let Some(cs) = client_secret {
                    envs.insert(
                        format!("RCLONE_CONFIG_{}_CLIENT_SECRET", name.to_uppercase()),
                        cs.clone(),
                    );
                }
                (format!("{}:{}", name, path), envs)
            }
        }
    }
}

fn obscure_password(password: &str) -> String {
    if let Ok(output) = std::process::Command::new("rclone")
        .args(&["obscure", password])
        .output()
    {
        if output.status.success() {
            let obscured = String::from_utf8_lossy(&output.stdout);
            return obscured.trim().to_string();
        }
    }
    password.to_string()
}

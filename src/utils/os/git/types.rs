use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitStatusEntry {
    pub index_status: char,
    pub worktree_status: char,
    pub path: String,
    pub original_path: Option<String>,
}

impl GitStatusEntry {
    pub(crate) fn parse(line: &str) -> Option<Self> {
        let mut chars = line.chars();
        let index_status = chars.next()?;
        let worktree_status = chars.next()?;
        if chars.next()? != ' ' {
            return None;
        }
        Some(Self {
            index_status,
            worktree_status,
            path: chars.collect(),
            original_path: None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitBranch {
    pub name: String,
    pub current: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GitAuth {
    Token(String),
    SshKey(String),
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let n = match chunk.len() {
            3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32),
            2 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8),
            1 => (chunk[0] as u32) << 16,
            _ => unreachable!(),
        };
        result.push(ALPHABET[((n >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((n >> 12) & 0x3F) as usize] as char);
        result.push(if chunk.len() > 1 {
            ALPHABET[((n >> 6) & 0x3F) as usize] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            ALPHABET[(n & 0x3F) as usize] as char
        } else {
            '='
        });
    }
    result
}

impl GitAuth {
    pub fn to_config(&self) -> (String, String) {
        match self {
            Self::Token(token) => {
                if token.contains(':') {
                    // Basic Auth (e.g. Bitbucket App Password)
                    let encoded = base64_encode(token.as_bytes());
                    (
                        "http.extraHeader".to_string(),
                        format!("Authorization: Basic {}", encoded),
                    )
                } else {
                    // Bearer Token (e.g. GitHub/GitLab token)
                    (
                        "http.extraHeader".to_string(),
                        format!("Authorization: Bearer {}", token),
                    )
                }
            }
            Self::SshKey(key_path) => {
                // Safely quote the SSH key path to prevent shell injection or spaces issues
                let escaped_key_path = key_path.replace("'", "'\\''");
                (
                    "core.sshCommand".to_string(),
                    format!(
                        "ssh -i '{}' -o StrictHostKeyChecking=accept-new",
                        escaped_key_path
                    ),
                )
            }
        }
    }
}

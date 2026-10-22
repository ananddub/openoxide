use std::fmt;

#[derive(Debug)]
pub enum TokenError {
    Expired,
    Invalid,
    WrongType,
    EncodingFailed,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::Expired => write!(f, "token expired"),
            TokenError::Invalid => write!(f, "invalid token"),
            TokenError::WrongType => write!(f, "wrong token type"),
            TokenError::EncodingFailed => write!(f, "failed to encode token"),
        }
    }
}

impl std::error::Error for TokenError {}

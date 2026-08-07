use crate::exec::{ExecError, ExecResult};

macro_rules! validated_u16 {
    ($name:ident, $default:expr, $min:expr, $max:expr, $message:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(u16);
        impl $name {
            pub fn new(value: u16) -> ExecResult<Self> {
                if ($min..=$max).contains(&value) {
                    Ok(Self(value))
                } else {
                    Err(command_error($message))
                }
            }
            pub const fn get(self) -> u16 {
                self.0
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self($default)
            }
        }
    };
}

validated_u16!(
    DigPort,
    53,
    1,
    u16::MAX,
    "dig port must be between 1 and 65535"
);
validated_u16!(
    DigTimeout,
    3,
    1,
    u16::MAX,
    "dig timeout must be greater than zero"
);
validated_u16!(
    DigTries,
    2,
    1,
    u16::MAX,
    "dig tries must be greater than zero"
);
validated_u16!(
    DigEdnsBufferSize,
    1232,
    512,
    u16::MAX,
    "EDNS buffer size must be between 512 and 65535"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigEdnsVersion(u8);
impl DigEdnsVersion {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
    pub const fn get(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsName(String);

impl DnsName {
    pub fn new(value: impl Into<String>) -> ExecResult<Self> {
        let value = value.into().trim_end_matches('.').to_ascii_lowercase();
        validate_dns_name(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigRecordType {
    #[default]
    A,
    Aaaa,
    Caa,
    Cname,
    Mx,
    Naptr,
    Ns,
    Ptr,
    Soa,
    Srv,
    Txt,
    Any,
}

impl DigRecordType {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::Aaaa => "AAAA",
            Self::Caa => "CAA",
            Self::Cname => "CNAME",
            Self::Mx => "MX",
            Self::Naptr => "NAPTR",
            Self::Ns => "NS",
            Self::Ptr => "PTR",
            Self::Soa => "SOA",
            Self::Srv => "SRV",
            Self::Txt => "TXT",
            Self::Any => "ANY",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigTransport {
    #[default]
    Udp,
    Tcp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigAddressFamily {
    Any,
    Ipv4,
    Ipv6,
}

impl Default for DigAddressFamily {
    fn default() -> Self {
        Self::Any
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DigQueryClass {
    #[default]
    In,
    Ch,
    Hs,
    Any,
}
impl DigQueryClass {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::In => "IN",
            Self::Ch => "CH",
            Self::Hs => "HS",
            Self::Any => "ANY",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DigSection {
    Question,
    Answer,
    Authority,
    Additional,
    Comments,
    Stats,
}

impl DigSection {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Question => "question",
            Self::Answer => "answer",
            Self::Authority => "authority",
            Self::Additional => "additional",
            Self::Comments => "comments",
            Self::Stats => "stats",
        }
    }
}

pub(super) fn command_error(message: impl Into<String>) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.into(),
    }
}

fn validate_dns_name(value: &str) -> ExecResult<()> {
    if value.is_empty()
        || value.len() > 253
        || value.split('.').any(|label| {
            label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '-')
        })
    {
        Err(command_error("invalid DNS name"))
    } else {
        Ok(())
    }
}

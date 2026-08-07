use std::fmt;

/// Docker target platform for multi-arch images.
///
/// ```
/// Platform::LinuxAmd64   // "linux/amd64"
/// Platform::LinuxArm64   // "linux/arm64"
/// Platform::custom("windows/amd64")
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Platform {
    LinuxAmd64,
    LinuxArm64,
    /// Linux ARMv7.
    LinuxArmV7,
    LinuxS390x,
    LinuxPpc64le,
    Custom(String),
}

impl Platform {
    pub fn custom(s: impl Into<String>) -> Self {
        Self::Custom(s.into())
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::LinuxAmd64 => "linux/amd64",
            Self::LinuxArm64 => "linux/arm64",
            Self::LinuxArmV7 => "linux/arm/v7",
            Self::LinuxS390x => "linux/s390x",
            Self::LinuxPpc64le => "linux/ppc64le",
            Self::Custom(s) => s.as_str(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn amd64() {
        assert_eq!(Platform::LinuxAmd64.to_string(), "linux/amd64");
    }
    #[test]
    fn arm64() {
        assert_eq!(Platform::LinuxArm64.to_string(), "linux/arm64");
    }
    #[test]
    fn armv7() {
        assert_eq!(Platform::LinuxArmV7.to_string(), "linux/arm/v7");
    }
    #[test]
    fn custom() {
        assert_eq!(
            Platform::custom("windows/amd64").to_string(),
            "windows/amd64"
        );
    }
}

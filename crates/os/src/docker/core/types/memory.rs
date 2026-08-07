use std::fmt;

/// Strongly-typed memory value. Internally stored as bytes.
///
/// ```
/// Memory::mb(512)  // "512m"
/// Memory::gb(2)    // "2g"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Memory(u64);

impl Memory {
    pub fn bytes(n: u64) -> Self {
        Self(n)
    }
    pub fn kb(n: u64) -> Self {
        Self(n * 1_024)
    }
    pub fn mb(n: u64) -> Self {
        Self(n * 1_024 * 1_024)
    }
    pub fn gb(n: u64) -> Self {
        Self(n * 1_024 * 1_024 * 1_024)
    }
    pub fn as_bytes(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const GB: u64 = 1_024 * 1_024 * 1_024;
        const MB: u64 = 1_024 * 1_024;
        const KB: u64 = 1_024;
        if self.0 % GB == 0 {
            write!(f, "{}g", self.0 / GB)
        } else if self.0 % MB == 0 {
            write!(f, "{}m", self.0 / MB)
        } else if self.0 % KB == 0 {
            write!(f, "{}k", self.0 / KB)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_mb() {
        assert_eq!(Memory::mb(512).to_string(), "512m");
    }
    #[test]
    fn display_gb() {
        assert_eq!(Memory::gb(2).to_string(), "2g");
    }
    #[test]
    fn display_kb() {
        assert_eq!(Memory::kb(4).to_string(), "4k");
    }
    #[test]
    fn display_bytes() {
        assert_eq!(Memory::bytes(100).to_string(), "100");
    }
    #[test]
    fn ordering() {
        assert!(Memory::gb(1) > Memory::mb(512));
    }
}

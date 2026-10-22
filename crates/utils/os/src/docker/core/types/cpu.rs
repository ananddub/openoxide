use std::fmt;

/// Strongly-typed CPU fractional core count.
///
/// ```
/// use oxos::docker::core::Cpu;
///
/// assert_eq!(Cpu::cores(0.5).to_string(), "0.50");
/// assert_eq!(Cpu::milliscores(500).to_string(), "0.50");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cpu(f64);

impl Cpu {
    /// Millicores — e.g. `500` = 0.5 cores.
    pub fn milliscores(m: u32) -> Self {
        Self(m as f64 / 1000.0)
    }
    pub fn cores(m: f64) -> Self {
        Self(m)
    }
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn millis() {
        assert_eq!(Cpu::milliscores(500).to_string(), "0.50");
    }
}

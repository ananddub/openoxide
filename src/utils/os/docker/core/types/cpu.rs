use std::fmt;

/// Strongly-typed CPU fractional core count.
///
/// ```
/// Cpu::new(0.5)     // "0.50"
/// Cpu::millis(500)  // "0.50"
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cpu(f64);

impl Cpu {
    /// `cores` — e.g. `0.5`, `2.0`.
    // pub fn new(cores: f64) -> Self { Self(cores) }
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
    // #[test] fn display_half() { assert_eq!(Cpu::new(0.5).to_string(), "0.50"); }
    #[test]
    fn millis() {
        assert_eq!(Cpu::milliscores(500).to_string(), "0.50");
    }
    // #[test] fn whole()        { assert_eq!(Cpu::new(2.0).to_string(), "2.00"); }
}

use super::error::AlertParseError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operator {
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
    Equal,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GreaterThan => "GT",
            Self::GreaterOrEqual => "GTE",
            Self::LessThan => "LT",
            Self::LessOrEqual => "LTE",
            Self::Equal => "EQ",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::GreaterThan => ">",
            Self::GreaterOrEqual => ">=",
            Self::LessThan => "<",
            Self::LessOrEqual => "<=",
            Self::Equal => "==",
        }
    }

    pub fn compare(&self, value: f64, threshold: f64) -> bool {
        match self {
            Self::GreaterThan => value > threshold,
            Self::GreaterOrEqual => value >= threshold,
            Self::LessThan => value < threshold,
            Self::LessOrEqual => value <= threshold,
            Self::Equal => (value - threshold).abs() < f64::EPSILON.max(threshold.abs() * 1e-9),
        }
    }
}

impl FromStr for Operator {
    type Err = AlertParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim().to_ascii_uppercase().as_str() {
            ">" | "GT" | "GREATER_THAN" => Ok(Self::GreaterThan),
            ">=" | "GTE" | "GREATER_OR_EQUAL" => Ok(Self::GreaterOrEqual),
            "<" | "LT" | "LESS_THAN" => Ok(Self::LessThan),
            "<=" | "LTE" | "LESS_OR_EQUAL" => Ok(Self::LessOrEqual),
            "==" | "=" | "EQ" | "EQUAL" => Ok(Self::Equal),
            other => Err(AlertParseError::Operator(other.to_owned())),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

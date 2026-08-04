use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

use super::AlertParseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertEventState {
    Firing,
    Resolved,
    NoData,
}

impl AlertEventState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Firing => "FIRING",
            Self::Resolved => "RESOLVED",
            Self::NoData => "NO_DATA",
        }
    }
}

impl fmt::Display for AlertEventState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AlertEventState {
    type Err = AlertParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_uppercase().as_str() {
            "FIRING" => Ok(Self::Firing),
            "RESOLVED" => Ok(Self::Resolved),
            "NO_DATA" => Ok(Self::NoData),
            other => Err(AlertParseError::EventState(other.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_states_round_trip_through_the_database_spelling() {
        for state in [
            AlertEventState::Firing,
            AlertEventState::Resolved,
            AlertEventState::NoData,
        ] {
            assert_eq!(state.as_str().parse::<AlertEventState>().unwrap(), state);
        }
    }

    #[test]
    fn unknown_database_state_is_rejected() {
        assert!("ACKNOWLEDGED".parse::<AlertEventState>().is_err());
    }
}

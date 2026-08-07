use os::string_enum;
use serde::{Deserialize, Serialize};

use super::AlertParseError;

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum AlertEventState {
        default = Firing;

        Firing => "FIRING",
        Resolved => "RESOLVED",
        NoData => "NO_DATA",
    }
}

use std::str::FromStr;

impl FromStr for AlertEventState {
    type Err = AlertParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_str(value).ok_or_else(|| AlertParseError::EventState(value.to_owned()))
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

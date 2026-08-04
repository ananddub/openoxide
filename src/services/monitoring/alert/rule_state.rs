use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "lowercase")]
#[oai(rename_all = "lowercase")]
pub enum RuleState {
    Enabled,
    Disabled,
}

impl RuleState {
    pub fn from_flag(flag: i32) -> Self {
        if flag == 0 {
            Self::Disabled
        } else {
            Self::Enabled
        }
    }

    pub fn as_flag(&self) -> i32 {
        match self {
            Self::Enabled => 1,
            Self::Disabled => 0,
        }
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
}

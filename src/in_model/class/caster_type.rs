use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CasterType {
    Full,
    Half,
    Artificer,
    Third,
    None,
}

impl Default for CasterType {
    fn default() -> Self {
        Self::None
    }
}

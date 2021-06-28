use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum CasterType {
    Full,
    Half,
    Artificer,
    None,
}

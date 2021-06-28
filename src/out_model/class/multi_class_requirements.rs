use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum MulticlassRequirementKey {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MultiClassRequirements {
    Value(MulticlassRequirementKey, u8),
    And(Vec<MultiClassRequirements>),
    Or(Vec<MultiClassRequirements>),
}

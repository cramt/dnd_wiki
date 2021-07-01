use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MulticlassRequirementKey {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

impl MultiClassRequirements {
    pub fn and_vec(self) -> Vec<Self> {
        match self {
            Self::Value(a, b) => vec![Self::Value(a, b)],
            Self::And(v) => v,
            Self::Or(v) => vec![Self::Or(v)],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MultiClassRequirements {
    Value(MulticlassRequirementKey, u8),
    And(Vec<MultiClassRequirements>),
    Or(Vec<MultiClassRequirements>),
}

impl ToString for MulticlassRequirementKey {
    fn to_string(&self) -> String {
        match self {
            Self::Str => "Strength",
            Self::Dex => "Dexterity",
            Self::Con => "Constitution",
            Self::Int => "Intelligence",
            Self::Wis => "Wisdom",
            Self::Cha => "Charisma",
        }
        .to_string()
    }
}

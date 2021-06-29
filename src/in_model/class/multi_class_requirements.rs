use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
pub enum MulticlassRequirementKey {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

const FIELDS: &[&str] = &["str", "dex", "con", "int", "wis", "cha"];

impl FromStr for MulticlassRequirementKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s.to_lowercase()[..3];
        match s {
            "str" => Ok(Self::Str),
            "dex" => Ok(Self::Dex),
            "con" => Ok(Self::Con),
            "int" => Ok(Self::Int),
            "wis" => Ok(Self::Wis),
            "cha" => Ok(Self::Cha),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum MultiClassRequirements {
    Value(MulticlassRequirementKey, u8),
    And(Vec<MultiClassRequirements>),
    Or(Vec<MultiClassRequirements>),
}

struct MultiClassRequirementsVisitor;

impl<'de> Visitor<'de> for MultiClassRequirementsVisitor {
    type Value = MultiClassRequirements;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("standard multi class requirements")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
    where
        A: MapAccess<'de>,
    {
        let mut v = Vec::new();
        while let Some((key, value)) = map.next_entry::<String, u8>()? {
            let key = key
                .parse::<MulticlassRequirementKey>()
                .map_err(|_| serde::de::Error::unknown_field(key.as_str(), FIELDS))?;
            v.push(MultiClassRequirements::Value(key, value))
        }
        Ok(MultiClassRequirements::And(v))
    }
}

impl<'de> Deserialize<'de> for MultiClassRequirements {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MultiClassRequirementsVisitor)
    }
}

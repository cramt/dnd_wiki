use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::str::FromStr;
use serde::ser::SerializeMap;

#[derive(Debug)]
pub enum MulticlassRequirementKey {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

const FIELDS: &'static [&'static str] = &["str", "dex", "con", "int", "wis", "cha"];

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

#[derive(Debug)]
pub enum MultiClassRequirements {
    Value(MulticlassRequirementKey, u8),
    And(Vec<MultiClassRequirements>),
    Or(Vec<MultiClassRequirements>),
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

impl Serialize for MultiClassRequirements {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match self {
            MultiClassRequirements::Value(_, _) => unimplemented!(),
            MultiClassRequirements::And(x) =>  {
                let mut map = serializer.serialize_map(Some(x.len()))?;

                for entry in x{
                    match entry {
                        MultiClassRequirements::Value(a, b) => map.serialize_entry(&a.to_string(), b)?,
                        MultiClassRequirements::And(_) => unimplemented!(),
                        MultiClassRequirements::Or(_) => unimplemented!(),
                    }
                }

                map.end()
            }
            MultiClassRequirements::Or(_) => unimplemented!(),
        }
    }
}

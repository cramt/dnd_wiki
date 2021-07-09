use std::collections::HashMap;

use serde::{de::Visitor, Deserialize};
use shoulda::Shoulda;

use super::subrace::Subrace;

#[derive(Debug, Shoulda)]
pub struct Race {
    pub name: String,
    pub category: String,
    pub flavor_text: Option<String>,
    pub features: HashMap<String, String>,
    pub subraces: Vec<Subrace>,
}

struct RaceVisitor;

impl<'de> Visitor<'de> for RaceVisitor {
    type Value = Race;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("standard race")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut features = HashMap::new();
        let mut subraces = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            if key == "subraces" {
                subraces = map.next_value::<Vec<Subrace>>()?;
            } else {
                features.insert(key, map.next_value::<String>()?);
            }
        }
        let name = features
            .remove("name")
            .ok_or_else(|| serde::de::Error::custom("no name field"))?;
        let category = features
            .remove("category")
            .ok_or_else(|| serde::de::Error::custom("no category field"))?;
        let flavor_text = features.remove("flavor_text");
        Ok(Race {
            name,
            category,
            flavor_text,
            features,
            subraces,
        })
    }
}

impl<'de> Deserialize<'de> for Race {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RaceVisitor)
    }
}

use std::collections::HashMap;

use serde::{de::Visitor, Deserialize};
use shoulda::Shoulda;

#[derive(Debug, Shoulda)]
pub struct Subrace {
    pub name: String,
    pub flavor_text: Option<String>,
    pub features: HashMap<String, String>,
}

struct SubraceVisitor;

impl<'de> Visitor<'de> for SubraceVisitor {
    type Value = Subrace;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("standard race")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut features = HashMap::new();
        while let Some((key, val)) = map.next_entry::<String, String>()? {
            features.insert(key, val);
        }
        let name = features
            .remove("name")
            .ok_or_else(|| serde::de::Error::custom("no name field"))?;
        let flavor_text = features.remove("flavor_text");
        Ok(Subrace {
            name,
            flavor_text,
            features,
        })
    }
}

impl<'de> Deserialize<'de> for Subrace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(SubraceVisitor)
    }
}

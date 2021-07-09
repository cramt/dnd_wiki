use serde::{de::Visitor, Deserialize};
use shoulda::Shoulda;

#[derive(Debug, Shoulda)]
pub struct RaceCategory {
    pub name: String,
    pub body: Option<String>,
}

struct RaceCategoryVisitor;

impl<'de> Visitor<'de> for RaceCategoryVisitor {
    type Value = RaceCategory;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("standard race category")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RaceCategory {
            body: None,
            name: v,
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut name = None;
        let mut body = None;
        for _ in 0..2 {
            let (k, v) = map
                .next_entry::<String, Option<String>>()?
                .ok_or_else(|| serde::de::Error::custom("map not atleast 2 long"))?;
            if k == "body" {
                body = v;
            } else {
                name = Some(k);
            }
        }
        let name = name.ok_or_else(|| serde::de::Error::custom("no name found"))?;
        Ok(RaceCategory { name, body })
    }
}

impl<'de> Deserialize<'de> for RaceCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(RaceCategoryVisitor)
    }
}

use serde::{de::Visitor, Deserialize};

#[derive(Debug)]
pub enum PrerequisiteEntry {
    Leaf(String),
    Or(Vec<PrerequisiteEntry>),
}

struct PrerequisiteEntryVisitor;

impl<'de> Visitor<'de> for PrerequisiteEntryVisitor {
    type Value = PrerequisiteEntry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("prerequisite entry")
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
        Ok(PrerequisiteEntry::Leaf(v))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let (key, val) = map
            .next_entry::<String, Vec<String>>()?
            .ok_or_else(|| serde::de::Error::custom("not of size 1"))?;
        if key == "or" {
            Ok(PrerequisiteEntry::Or(
                val.into_iter()
                    .map(PrerequisiteEntry::Leaf)
                    .collect(),
            ))
        } else {
            Err(serde::de::Error::custom("key not \"or\""))
        }
    }
}

impl<'de> Deserialize<'de> for PrerequisiteEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(PrerequisiteEntryVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct Prerequisites(pub Vec<PrerequisiteEntry>);

impl Default for Prerequisites {
    fn default() -> Self {
        Self(Vec::new())
    }
}
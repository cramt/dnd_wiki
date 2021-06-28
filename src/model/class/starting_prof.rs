use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum StartingProfEntry {
    StrictSet(Vec<String>, u8),
    ChooseSet(u8, Vec<String>, u8),
    Empty,
}

impl Default for StartingProfEntry {
    fn default() -> Self {
        StartingProfEntry::Empty
    }
}

struct StartingProfEntryVisitor;

const FIELDS: &'static [&'static str] = &["amount", "list", "any"];

impl<'de> Visitor<'de> for StartingProfEntryVisitor {
    type Value = StartingProfEntry;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("standard starting proficiency formatting")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Self::Value::Empty)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Self::Value::Empty)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut entries = Vec::new();
        let mut anys = 0;

        while let Some(s) = seq.next_element::<String>()? {
            if s == "any" {
                anys += 1;
            } else {
                entries.push(s.into())
            }
        }

        Ok(StartingProfEntry::StrictSet(entries, anys))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error>
    where
        A: MapAccess<'de>,
    {
        let mut amount = None;
        let mut list = None;
        let mut any = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_ref() {
                "amount" => {
                    if amount.is_some() {
                        return Err(serde::de::Error::duplicate_field("amount"));
                    }
                    amount = Some(map.next_value::<u8>()?)
                }
                "list" => {
                    if list.is_some() {
                        return Err(serde::de::Error::duplicate_field("list"));
                    }
                    list = Some(map.next_value::<Vec<String>>()?)
                }
                "any" => {
                    if any.is_some() {
                        return Err(serde::de::Error::duplicate_field("any"));
                    }
                    any = Some(map.next_value::<u8>()?)
                }
                _unknown => return Err(serde::de::Error::unknown_field(_unknown, FIELDS)),
            }
        }
        let any = any.unwrap_or(0);
        match (amount, list) {
            (Some(amount), Some(list)) => Ok(StartingProfEntry::ChooseSet(amount, list, any)),
            (None, None) | (None, Some(_)) => Err(serde::de::Error::missing_field("amount")),
            (Some(_), None) => Err(serde::de::Error::missing_field("list")),
        }
    }
}

impl<'de> Deserialize<'de> for StartingProfEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StartingProfEntryVisitor)
    }
}

impl Serialize for StartingProfEntry {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            StartingProfEntry::StrictSet(a, b) => {
                let mut seq = serializer.serialize_seq(Some(a.len() + *b as usize))?;

                for entry in a {
                    seq.serialize_element(entry)?;
                }

                for _ in 0..*b {
                    seq.serialize_element("any")?;
                }

                seq.end()
            }
            StartingProfEntry::ChooseSet(a, b, c) => {
                let mut map = serializer.serialize_map(Some(3))?;

                map.serialize_entry("amount", a)?;
                map.serialize_entry("list", b)?;
                map.serialize_entry("any", c)?;

                map.end()
            }
            StartingProfEntry::Empty => serializer.serialize_none(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StartingProf {
    #[serde(default)]
    armor: StartingProfEntry,
    #[serde(default)]
    weapons: StartingProfEntry,
    #[serde(default)]
    tools: StartingProfEntry,
    #[serde(default)]
    saving_throws: StartingProfEntry,
    #[serde(default)]
    skills: StartingProfEntry,
}

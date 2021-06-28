use core::fmt;
use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Serialize)]
pub struct Equipment(Vec<Vec<String>>);

#[derive(Debug)]
pub enum EquipmentEntry {
    Choice(Vec<String>),
    Specific(String),
}

struct EquipmentEntryVisitor;

impl<'de> Visitor<'de> for EquipmentEntryVisitor {
    type Value = EquipmentEntry;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("standard equipment entry")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(EquipmentEntry::Specific(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        if let Some(el) = seq.next_element::<String>()? {
            v.push(el)
        }
        Ok(EquipmentEntry::Choice(v))
    }
}

impl<'de> Deserialize<'de> for EquipmentEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EquipmentEntryVisitor)
    }
}

impl Serialize for EquipmentEntry {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            EquipmentEntry::Choice(v) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                for x in v {
                    seq.serialize_element(x)?
                }
                seq.end()
            }
            EquipmentEntry::Specific(a) => serializer.serialize_str(a),
        }
    }
}

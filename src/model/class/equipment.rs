use serde::de::{SeqAccess, Visitor, Error};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use serde::ser::SerializeSeq;
use core::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Equipment(Vec<EquipmentEntry>);

#[derive(Debug)]
pub enum EquipmentEntry {
    Choice(String, String),
    Specific(String),
}

struct EquipmentEntryVisitor;

impl<'de> Visitor<'de> for EquipmentEntryVisitor {
    type Value = EquipmentEntry;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("standard equipment entry")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: Error, {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where
        E: Error, {
        Ok(EquipmentEntry::Specific(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error> where
        A: SeqAccess<'de>, {
        let a = seq.next_element::<String>()?.ok_or_else(|| serde::de::Error::custom("not length 2"))?;
        let b = seq.next_element::<String>()?.ok_or_else(|| serde::de::Error::custom("not length 2"))?;
        Ok(EquipmentEntry::Choice(a, b))
    }
}

impl<'de> Deserialize<'de> for EquipmentEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_any(EquipmentEntryVisitor)
    }
}

impl Serialize for EquipmentEntry {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match self {
            EquipmentEntry::Choice(a, b) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element(a)?;
                seq.serialize_element(b)?;
                seq.end()
            }
            EquipmentEntry::Specific(a) => serializer.serialize_str(a)
        }
    }
}

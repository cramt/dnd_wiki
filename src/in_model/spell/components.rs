use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Debug)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<String>,
}

impl FromStr for Components {
    type Err = ();

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(V)?,?\s?(S)?,?\s?(M\s\([^\)]+\))?$").unwrap());
        match REGEX.captures(v) {
            None => Err(()),
            Some(captures) => Ok(Components {
                verbal: captures.get(1).is_some(),
                somatic: captures.get(2).is_some(),
                material: captures.get(3).map(|x| {
                    let str = x.as_str();
                    str[3..(str.len() - 1)].to_string()
                }),
            }),
        }
    }
}

struct ComponentsVisitor;

impl<'de> Visitor<'de> for ComponentsVisitor {
    type Value = Components;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("standard spell components")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.parse()
            .map_err(|_| E::custom("failed to parse spell components"))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v.as_str())
    }
}

impl<'de> Deserialize<'de> for Components {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ComponentsVisitor)
    }
}

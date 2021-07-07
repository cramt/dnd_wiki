use serde::{Deserialize, Serialize};

use crate::text_utils::proper_noun;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PrerequisiteEntry {
    Leaf(String),
    Or(Vec<PrerequisiteEntry>),
}

impl PrerequisiteEntry {
    pub fn exists(&self) -> bool {
        match self {
            PrerequisiteEntry::Leaf(x) => !x.is_empty(),
            PrerequisiteEntry::Or(x) => !x.is_empty(),
        }
    }
}

impl ToString for PrerequisiteEntry {
    fn to_string(&self) -> String {
        match self {
            PrerequisiteEntry::Leaf(x) => proper_noun(x.to_string()),
            PrerequisiteEntry::Or(x) => x
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" or "),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Prerequisites(pub Vec<PrerequisiteEntry>);

impl Prerequisites {
    pub fn exists(&self) -> bool {
        self.0.iter().fold(false, |acc, x| acc || x.exists())
    }
}

impl ToString for Prerequisites {
    fn to_string(&self) -> String {
        self.0.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(", ")
    }
}

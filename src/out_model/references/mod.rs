use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::text_utils::file_name_sanitize;

use super::index::Index;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct References(HashMap<String, ReferenceEntry>);

impl Deref for References {
    type Target = HashMap<String, ReferenceEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for References {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for References {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl From<HashMap<String, ReferenceEntry>> for References {
    fn from(a: HashMap<String, ReferenceEntry>) -> Self {
        Self(a)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceEntry {
    pub link: String,
    pub children: References,
}

impl From<&Index> for References {
    fn from(index: &Index) -> Self {
        let mut inner = Self::default();
        inner.insert(
            "spells".to_string(),
            ReferenceEntry {
                link: "spells/index.html".to_string(),
                children: Self::from(
                    index
                        .spells
                        .iter()
                        .map(|x| {
                            (
                                x.name.to_string(),
                                ReferenceEntry {
                                    link: format!("{}.html", file_name_sanitize(&x.name)),
                                    children: References::default(),
                                },
                            )
                        })
                        .collect::<HashMap<_, _>>(),
                ),
            },
        );
        for class in &index.classes {
            inner.insert(
                class.name.to_string(),
                ReferenceEntry {
                    link: format!("classes/{}/index.html", file_name_sanitize(&class.name)),
                    children: Self::from(
                        class
                            .subclasses
                            .entries
                            .iter()
                            .map(|x| {
                                (
                                    x.name.to_string(),
                                    ReferenceEntry {
                                        link: format!("{}.html", file_name_sanitize(&x.name)),
                                        children: Self::default(),
                                    },
                                )
                            })
                            .collect::<HashMap<_, _>>(),
                    ),
                },
            );
        }
        inner
    }
}

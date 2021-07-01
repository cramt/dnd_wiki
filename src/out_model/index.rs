use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::handlebars_engine as engine;
use crate::out_model::class::Class;
use crate::out_model::spell::Spell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    pub classes: Vec<Class>,
    pub spells: Vec<Spell>,
    pub style: String,
    #[serde(rename = "static")]
    pub static_folder: String,
    pub schools: HashSet<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub path_to_parent: String,
    pub path_to_index: String,
}

impl Metadata {
    pub fn new_wrapper<T>(self, t: T) -> MetadataWrapper<T> {
        MetadataWrapper {
            metadata: self,
            inner: t,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataWrapper<T> {
    pub metadata: Metadata,
    pub inner: T,
}

impl Index {
    pub fn build(&self) -> Result<HashMap<Cow<str>, String>, Box<dyn Error>> {
        let mut map: HashMap<Cow<str>, String> = HashMap::new();
        map.insert(
            "index.html".into(),
            engine::index::engine().render(
                &Metadata {
                    name: self.name.to_string(),
                    path_to_index: String::new(),
                    path_to_parent: String::new(),
                }
                .new_wrapper(self.clone()),
            )?,
        );
        map.insert(
            "spells/index.html".into(),
            engine::spells::engine().render(
                &Metadata {
                    name: self.name.to_string(),
                    path_to_index: "../index.html".to_string(),
                    path_to_parent: "../index.html".to_string(),
                }
                .new_wrapper(self.spells.clone()),
            )?,
        );
        for spell in &self.spells {
            map.insert(
                format!("spells/{}.html", spell.name).into(),
                engine::spell::engine().render(
                    &Metadata {
                        name: self.name.to_string(),
                        path_to_index: "../index.html".to_string(),
                        path_to_parent: "./index.html".to_string(),
                    }
                    .new_wrapper(spell.clone()),
                )?,
            );
        }
        for class in &self.classes {
            map.insert(
                format!("classes/{}.html", class.name).into(),
                engine::class::engine().render(
                    &Metadata {
                        name: self.name.to_string(),
                        path_to_index: "../index.html".to_string(),
                        path_to_parent: "../index.html".to_string(),
                    }
                    .new_wrapper(class.clone()),
                )?,
            );
        }
        Ok(map)
    }
}

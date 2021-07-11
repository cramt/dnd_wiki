use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::Deref;

use crate::out_model::crs::Crs;
use crate::text_utils::file_name_sanitize;
use crate::{handlebars_definitions, handlebars_engine as engine};
use dnd_wiki_markdown::referencer::Referencer;
use serde::{Deserialize, Serialize};

use super::class::Classes;
use super::featlikes::Featlikes;
use super::races::Races;
use super::spell::Spells;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    pub classes: Classes,
    pub spells: Spells,
    pub style: String,
    #[serde(rename = "static")]
    pub static_folder: String,
    pub schools: HashSet<String>,
    pub name: String,
    pub feats: Featlikes,
    pub races: Races,
}

impl Referencer for Index {
    fn prop(&self, prop: &str) -> Option<&dyn Referencer> {
        match prop {
            "class" => Some(&self.classes),
            "spells" => Some(&self.spells),
            "feats" => Some(b"./feats.html" as &dyn Referencer),
            _ => None,
        }
    }

    fn value(&self) -> Cow<str> {
        "".into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub path_to_parent: String,
    pub path_to_index: String,
    pub index: Crs<Index>,
    pub schools: HashSet<String>,
}

impl Metadata {
    pub fn parent<'a, S: Into<Cow<'a, str>>>(mut self, s: S) -> Self {
        let s: Cow<str> = s.into();
        self.path_to_parent = s.into_owned();
        self
    }
    pub fn index<'a, S: Into<Cow<'a, str>>>(mut self, s: S) -> Self {
        let s: Cow<str> = s.into();
        self.path_to_index = s.into_owned();
        self
    }
    pub fn index_parent<'a, S: Into<Cow<'a, str>>>(self, s: S) -> Self {
        let s: Cow<str> = s.into();
        self.index(s.deref()).parent(s)
    }
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
        let metadata = Metadata {
            index: Crs::new(&self),
            name: self.name.to_string(),
            path_to_index: "./".into(),
            path_to_parent: "./index.html".into(),
            schools: self.schools.clone(),
        };
        let mut map: HashMap<Cow<str>, String> = HashMap::new();
        map.insert(
            "index.html".into(),
            engine::index::engine().render(&metadata.clone().new_wrapper(self.clone()))?,
        );
        map.insert(
            "feats.html".into(),
            engine::feats::engine().render(
                &metadata
                    .clone()
                    .index("../")
                    .parent("../index.html")
                    .new_wrapper(self.feats.clone()),
            )?,
        );
        map.insert(
            "spells/index.html".into(),
            engine::spells::engine().render(
                &metadata
                    .clone()
                    .index("../")
                    .parent("../index.html")
                    .new_wrapper(self.spells.clone()),
            )?,
        );
        for spell in self.spells.deref() {
            map.insert(
                format!("spells/{}.html", file_name_sanitize(spell.name.as_str())).into(),
                engine::spell::engine()
                    .render(&metadata.clone().index("../").new_wrapper(spell.clone()))?,
            );
        }
        for class in self.classes.deref() {
            map.insert(
                format!(
                    "classes/{}/index.html",
                    file_name_sanitize(class.name.as_str())
                )
                .into(),
                engine::class::engine().render(
                    &metadata
                        .clone()
                        .index("../../")
                        .parent("../../index.html")
                        .new_wrapper(class.clone()),
                )?,
            );
            for subclass in &class.subclasses.entries {
                map.insert(
                    format!(
                        "classes/{}/{}.html",
                        file_name_sanitize(class.name.as_str()),
                        file_name_sanitize(subclass.name.as_str())
                    )
                    .into(),
                    engine::subclass::engine().render(
                        &metadata
                            .clone()
                            .parent("../../index.html")
                            .index("../../")
                            .new_wrapper(subclass.clone()),
                    )?,
                );
            }
        }
        map.insert(
            "races/index.html".into(),
            engine::races::engine().render(
                &metadata
                    .index("../")
                    .parent("../index.html")
                    .new_wrapper(self.races.clone()),
            )?,
        );
        map.insert("sw.js".into(), handlebars_definitions::sw().to_string());
        Ok(map)
    }
}

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ops::Deref;

use crate::out_model::class::Class;
use crate::out_model::crs::Crs;
use crate::out_model::spell::Spell;
use crate::text_utils::file_name_sanitize;
use crate::{handlebars_definitions, handlebars_engine as engine};
use serde::{Deserialize, Serialize};

use super::featlikes::Featlikes;
use super::races::Races;
use super::references::References;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    pub classes: Vec<Class>,
    pub spells: Vec<Spell>,
    pub style: String,
    #[serde(rename = "static")]
    pub static_folder: String,
    pub schools: HashSet<String>,
    pub name: String,
    pub feats: Featlikes,
    pub races: Races,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub path_to_parent: String,
    pub path_to_index: String,
    pub references: Crs<References>,
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
        let references = References::from(self);
        let r = Crs::new(&references);
        let metadata = Metadata {
            references: r,
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
        for spell in &self.spells {
            map.insert(
                format!("spells/{}.html", file_name_sanitize(spell.name.as_str())).into(),
                engine::spell::engine()
                    .render(&metadata.clone().index("../").new_wrapper(spell.clone()))?,
            );
        }
        for class in &self.classes {
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

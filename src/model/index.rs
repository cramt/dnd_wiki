use crate::handlebars_engine as engine;
use crate::model::class::Class;
use crate::model::path_or_struct::path_or_struct;
use crate::model::spell::Spell;
use crate::model::vec_of_path_or_struct::vec_of_path_or_struct;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    #[serde(deserialize_with = "vec_of_path_or_struct")]
    pub classes: Vec<Class>,
    #[serde(deserialize_with = "path_or_struct")]
    pub spells: Vec<Spell>,
    pub style: String,
    #[serde(rename = "static")]
    pub static_folder: String,
}

impl Index {
    pub fn build(&self) -> Result<HashMap<Cow<str>, String>, Box<dyn Error>> {
        let mut map: HashMap<Cow<str>, String> = HashMap::new();
        map.insert("index.html".into(), engine::index::engine().render(&self)?);
        map.insert(
            "spells/index.html".into(),
            engine::spells::engine().render(&self.spells)?,
        );
        for spell in &self.spells {
            map.insert(
                format!("spells/{}.html", spell.name).into(),
                engine::spell::engine().render(spell)?,
            );
        }
        for class in &self.classes {
            map.insert(
                format!("classes/{}.html", class.name).into(),
                engine::class::engine().render(class)?,
            );
        }
        Ok(map)
    }
}

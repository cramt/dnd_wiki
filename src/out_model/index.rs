use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::handlebars_engine as engine;
use crate::out_model::class::Class;
use crate::out_model::spell::Spell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    pub classes: Vec<Class>,
    pub spells: Vec<Spell>,
    pub style: String,
    #[serde(rename = "static")]
    pub static_folder: String,
    pub schools: HashSet<String>,
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

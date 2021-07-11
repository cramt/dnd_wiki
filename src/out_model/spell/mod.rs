pub mod components;

use std::ops::Deref;

use components::Components;
use dnd_wiki_markdown::referencer::Referencer;
use serde::{Deserialize, Serialize};

use crate::text_utils::file_name_sanitize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Spells(pub Vec<Spell>);

impl Deref for Spells {
    type Target = Vec<Spell>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Referencer for Spells {
    fn prop(&self, prop: &str) -> Option<&dyn Referencer> {
        self.iter()
            .find(|x| x.name == prop)
            .map(|x| x as &dyn Referencer)
    }

    fn value(&self) -> std::borrow::Cow<str> {
        "./spells/index.html".into()
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Spell {
    pub name: String,
    pub ritual: bool,
    pub spell_level: u8,
    pub school: String,
    pub casting_time: String,
    pub range: String,
    pub duration: String,
    pub body: String,
    pub components: Components,
    pub higher_levels: Option<String>,
    pub casters: Vec<String>,
}

impl Referencer for Spell {
    fn prop(&self, _: &str) -> Option<&dyn Referencer> {
        None
    }

    fn value(&self) -> std::borrow::Cow<str> {
        format!("./spells/{}.html", file_name_sanitize(&self.name)).into()
    }
}

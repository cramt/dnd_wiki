mod components;

use serde::{Deserialize, Serialize};
use crate::model::spell::components::Components;

#[derive(Debug, Deserialize, Serialize)]
pub struct Spell {
    pub name: String,
    #[serde(default)]
    pub ritual: bool,
    pub spell_level: u8,
    pub school: String,
    pub casting_time: String,
    pub range: String,
    pub duration: String,
    pub body: String,
    pub components: Components,
    #[serde(default)]
    pub higher_levels: Option<String>,
}

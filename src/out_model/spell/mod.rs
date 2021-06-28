mod components;

use components::Components;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
}

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
}

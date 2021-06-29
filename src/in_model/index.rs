use std::collections::HashSet;

use crate::in_model::class::Class;
use crate::in_model::path_or_struct::path_or_struct;
use crate::in_model::spell::Spell;
use crate::in_model::vec_of_path_or_struct::vec_of_path_or_struct;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Index {
    #[serde(deserialize_with = "vec_of_path_or_struct")]
    pub classes: Vec<Class>,
    #[serde(deserialize_with = "path_or_struct")]
    pub spells: Vec<Spell>,
    pub style: String,
    #[serde(rename = "static")]
    pub static_folder: String,
    #[serde(deserialize_with = "path_or_struct")]
    pub schools: HashSet<String>
}

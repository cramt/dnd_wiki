use super::subclass::Subclass;
use crate::in_model::vec_of_path_or_struct::vec_of_path_or_struct;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Subclasses {
    pub name: String,
    pub level: u8,
    pub prefix: String,
    pub postfix: String,
    #[serde(deserialize_with = "vec_of_path_or_struct")]
    pub entries: Vec<Subclass>,
    pub features: Vec<u8>,
}

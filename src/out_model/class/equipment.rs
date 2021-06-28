use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Equipment(Vec<Vec<String>>);

#[derive(Debug, Deserialize, Serialize)]
pub enum EquipmentEntry {
    Choice(Vec<String>),
    Specific(String),
}

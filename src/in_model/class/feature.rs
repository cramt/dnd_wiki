use serde::{Deserialize, Serialize};
use crate::in_model::vec_or_single_element::vec_or_single_element;

#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    #[serde(deserialize_with = "vec_or_single_element")]
    pub level: Vec<u8>,
    pub name: String,
    pub body: String,
    #[serde(default)]
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub name: String,
    pub body: String,
    #[serde(default)]
    pub sections: Vec<Section>,
}

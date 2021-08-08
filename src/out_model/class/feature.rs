use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Feature {
    pub initial_level: u8,
    pub relevant_levels: Vec<u8>,
    pub name: String,
    pub body: String,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Section {
    pub name: String,
    pub body: String,
    pub sections: Vec<Section>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Feature {
    pub level: u8,
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

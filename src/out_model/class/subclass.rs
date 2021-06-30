use serde::{Deserialize, Serialize};

use super::feature::Feature;

#[derive(Debug, Deserialize, Serialize)]
pub struct Subclass {
    pub name: String,
    pub flavour_text: String,
    pub features: Vec<Feature>,
}

use serde::Deserialize;

use super::feature::Feature;

#[derive(Debug, Deserialize)]
pub struct Subclass {
    pub name: String,
    pub flavour_text: String,
    pub features: Vec<Feature>,
}

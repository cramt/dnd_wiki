use serde::Deserialize;

use super::{caster_type::CasterType, feature::Feature};

#[derive(Debug, Deserialize)]
pub struct Subclass {
    pub name: String,
    #[serde(default)]
    pub caster_type: CasterType,
    pub flavour_text: String,
    pub features: Vec<Feature>,
}

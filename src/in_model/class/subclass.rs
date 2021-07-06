use std::collections::HashMap;

use serde::Deserialize;

use super::{caster_type::CasterType, feature::Feature, resource::Resource};

#[derive(Debug, Deserialize)]
pub struct Subclass {
    pub name: String,
    #[serde(default)]
    pub caster_type: CasterType,
    #[serde(default)]
    pub start_cantrips_known: Option<u8>,
    pub flavour_text: String,
    pub features: Vec<Feature>,
    #[serde(default)]
    pub class_resources: HashMap<String, Resource>,
}

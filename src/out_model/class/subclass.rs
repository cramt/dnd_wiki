use serde::{Deserialize, Serialize};

use super::{caster_type::CasterType, feature::Feature};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subclass {
    pub name: String,
    pub flavour_text: String,
    pub features: Vec<Feature>,
    pub caster_type: CasterType,
}

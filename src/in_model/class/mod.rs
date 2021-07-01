use serde::Deserialize;

use crate::in_model::path_or_struct::path_or_struct;
use caster_type::CasterType;
use equipment::Equipment;
use feature::Feature;
use multi_class_requirements::MultiClassRequirements;
use resource::Resource;
use starting_prof::StartingProf;
use std::collections::{HashMap, HashSet};

use self::subclasses::Subclasses;

pub mod caster_type;
pub mod equipment;
pub mod feature;
pub mod multi_class_requirements;
pub mod resource;
pub mod starting_prof;
pub mod subclass;
pub mod subclasses;

#[derive(Debug, Deserialize)]
pub struct Class {
    pub name: String,
    #[serde(default)]
    pub caster_type: CasterType,
    #[serde(deserialize_with = "path_or_struct")]
    #[serde(default)]
    pub spell_list: HashSet<String>,
    #[serde(default)]
    pub start_cantrips_known: Option<u8>,
    pub flavour_text: String,
    pub multi_class_requirements: MultiClassRequirements,
    #[serde(default)]
    pub class_resources: HashMap<String, Resource>,
    pub hit_die: u8,
    pub starting_prof: StartingProf,
    pub equipment: Equipment,
    pub subclasses: Subclasses,
    pub features: Vec<Feature>,
}

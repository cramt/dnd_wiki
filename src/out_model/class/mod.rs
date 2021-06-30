use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Class {
    pub name: String,
    pub caster_type: CasterType,
    pub spell_list: HashSet<String>,
    pub start_cantrips_known: Option<u8>,
    pub flavour_text: String,
    pub multi_class_requirements: MultiClassRequirements,
    pub class_resources: HashMap<String, Resource>,
    pub hit_die: u8,
    pub starting_prof: StartingProf,
    pub equipment: Equipment,
    pub features: Vec<Feature>,
    pub subclasses: Subclasses,
}

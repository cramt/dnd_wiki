use dnd_wiki_markdown::referencer::Referencer;
use serde::{Deserialize, Serialize};

use caster_type::CasterType;
use equipment::Equipment;
use feature::Feature;
use multi_class_requirements::MultiClassRequirements;
use resource::Resource;
use starting_prof::StartingProf;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    ops::Deref,
};

use crate::text_utils::file_name_sanitize;

use self::{generic_class::GenericClass, subclasses::Subclasses};

pub mod caster_type;
pub mod equipment;
pub mod feature;
pub mod generic_class;
pub mod multi_class_requirements;
pub mod resource;
pub mod starting_prof;
pub mod subclass;
pub mod subclasses;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Classes(pub Vec<Class>);

impl Deref for Classes {
    type Target = Vec<Class>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Referencer for Classes {
    fn prop(&self, prop: &str) -> Option<&dyn Referencer> {
        self.iter()
            .find(|x| x.name() == prop)
            .map(|x| x as &dyn Referencer)
    }

    fn value(&self) -> Cow<str> {
        "".into()
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    pub features: HashMap<u8, Vec<Feature>>,
    pub subclasses: Subclasses,
}

impl Referencer for Class {
    fn prop(&self, prop: &str) -> Option<&dyn Referencer> {
        

        self.subclasses
            .entries
            .iter()
            .find(|x| x.name() == prop)
            .map(|x| x as &dyn Referencer)
    }

    fn value(&self) -> Cow<str> {
        format!("./classes/{}/index.html", file_name_sanitize(self.name())).into()
    }
}

impl GenericClass for Class {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn flavour_text(&self) -> &str {
        self.flavour_text.as_str()
    }

    fn start_cantrips_known(&self) -> Option<u8> {
        self.start_cantrips_known
    }

    fn features(&self) -> &HashMap<u8, Vec<Feature>> {
        &self.features
    }

    fn caster_type(&self) -> CasterType {
        self.caster_type.clone()
    }

    fn class_resources(&self) -> &HashMap<String, Resource> {
        &self.class_resources
    }
}

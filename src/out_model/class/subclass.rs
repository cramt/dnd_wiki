use dnd_wiki_markdown::referencer::Referencer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::text_utils::file_name_sanitize;

use super::{
    caster_type::CasterType, feature::Feature, generic_class::GenericClass, resource::Resource,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Subclass {
    pub parent_class_name: String,
    pub name: String,
    pub level: u8,
    pub flavour_text: String,
    pub start_cantrips_known: Option<u8>,
    pub features: HashMap<u8, Vec<Feature>>,
    pub caster_type: CasterType,
    pub class_resources: HashMap<String, Resource>,
}

impl Referencer for Subclass {
    fn prop(&self, _: &str) -> Option<&dyn Referencer> {
        None
    }

    fn value(&self) -> std::borrow::Cow<str> {
        format!(
            "./classes/{}/{}.html",
            file_name_sanitize(&self.parent_class_name),
            file_name_sanitize(self.name())
        )
        .into()
    }
}

impl GenericClass for Subclass {
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

use std::collections::HashMap;

use crate::out_model::class::resource::Resource;

use super::{caster_type::CasterType, feature::Feature};

pub trait GenericClass {
    fn name(&self) -> &str;
    fn flavour_text(&self) -> &str;
    fn start_cantrips_known(&self) -> Option<u8>;
    fn features(&self) -> &HashMap<u8, Vec<Feature>>;
    fn caster_type(&self) -> CasterType;
    fn class_resources(&self) -> &HashMap<String, Resource>;

    fn cantrips_known_at_level(&self, level: u8) -> u8 {
        if let Some(start) = self.start_cantrips_known() {
            match self.caster_type() {
                CasterType::Full => {
                    if level > 9 {
                        start + 2
                    } else if level > 3 {
                        start + 1
                    } else {
                        start
                    }
                }

                CasterType::Half => 0,
                CasterType::Artificer => {
                    if level > 14 {
                        start + 2
                    } else if level > 9 {
                        start + 1
                    } else {
                        start
                    }
                }
                CasterType::Third => {
                    if level > 9 {
                        start + 1
                    } else if level > 2 {
                        start
                    } else {
                        0
                    }
                }
                CasterType::None => 0,
            }
        } else {
            0
        }
    }

    fn has_spells(&self) -> bool {
        self.caster_type() != CasterType::None
    }
}

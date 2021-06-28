pub mod caster_type;
pub mod equipment;
pub mod feature;
pub mod multi_class_requirements;
pub mod resource;
pub mod starting_prof;

use crate::model::class::Class as In;
use crate::out_model::class::Class as Out;

impl Into<Out> for In {
    fn into(self) -> Out {
        let Self {
            name,
            caster_type,
            spell_list,
            start_cantrips_known,
            flavour_text,
            multi_class_requirements,
            class_resources,
            hit_die,
            starting_prof,
            equipment,
            features,
        } = self;
        let caster_type = caster_type.into();
        let multi_class_requirements = multi_class_requirements.into();
        let class_resources = class_resources
            .into_iter()
            .map(|(a, b)| (a, b.into()))
            .collect();
        let starting_prof = starting_prof.into();
        let equipment = equipment.into();
        let features = features.into_iter().map(|x| x.into()).collect();
        Out {
            name,
            caster_type,
            spell_list,
            start_cantrips_known,
            flavour_text,
            multi_class_requirements,
            class_resources,
            hit_die,
            starting_prof,
            equipment,
            features,
        }
    }
}

impl Into<In> for Out {
    fn into(self) -> In {
        let Self {
            name,
            caster_type,
            spell_list,
            start_cantrips_known,
            flavour_text,
            multi_class_requirements,
            class_resources,
            hit_die,
            starting_prof,
            equipment,
            features,
        } = self;
        let caster_type = caster_type.into();
        let multi_class_requirements = multi_class_requirements.into();
        let class_resources = class_resources
            .into_iter()
            .map(|(a, b)| (a, b.into()))
            .collect();
        let starting_prof = starting_prof.into();
        let equipment = equipment.into();
        let features = features.into_iter().map(|x| x.into()).collect();
        In {
            name,
            caster_type,
            spell_list,
            start_cantrips_known,
            flavour_text,
            multi_class_requirements,
            class_resources,
            hit_die,
            starting_prof,
            equipment,
            features,
        }
    }
}

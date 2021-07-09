pub mod caster_type;
pub mod equipment;
pub mod feature;
pub mod multi_class_requirements;
pub mod resource;
pub mod starting_prof;
pub mod subclass;
pub mod subclasses;

use std::collections::HashMap;

use crate::in_model::class::Class as In;
use crate::out_model::class::Class as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
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
            subclasses,
        } = val;
        let caster_type = caster_type.into();
        let multi_class_requirements = multi_class_requirements.into();
        let class_resources = class_resources
            .into_iter()
            .map(|(a, b)| (a, b.into()))
            .collect();
        let starting_prof = starting_prof.into();
        let equipment = equipment.into();
        let mut features: Vec<crate::out_model::class::feature::Feature> =
            features.into_iter().map(|x| x.into()).collect();
        let (subclasses, subclass_feature) = subclasses.out(name.as_str());
        features.push(subclass_feature);
        let features = features.into_iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x.level).or_insert_with(Vec::new).push(x);
            acc
        });
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
            subclasses,
        }
    }
}

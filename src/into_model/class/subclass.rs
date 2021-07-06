use std::collections::HashMap;

use crate::in_model::class::subclass::Subclass as In;
use crate::out_model::class::feature::Feature;
use crate::out_model::class::subclass::Subclass as Out;

impl From<(In, u8)> for Out {
    fn from(val: (In, u8)) -> Self {
        let (
            In {
                name,
                flavour_text,
                features,
                caster_type,
                start_cantrips_known,
                class_resources,
            },
            level,
        ) = val;
        let features: Vec<Feature> = features.into_iter().map(|x| x.into()).collect();
        let class_resources = class_resources
            .into_iter()
            .map(|(a, b)| (a, b.into()))
            .collect();
        let caster_type = caster_type.into();
        let features = features.into_iter().fold(HashMap::new(), |mut acc, x| {
            if !acc.contains_key(&x.level) {
                acc.insert(x.level, Vec::new());
            }
            acc.get_mut(&x.level).unwrap().push(x);
            acc
        });
        Out {
            level,
            class_resources,
            name,
            flavour_text,
            features,
            caster_type,
            start_cantrips_known,
        }
    }
}

pub mod componenets;

use crate::in_model::spell::Spell as In;
use crate::out_model::spell::Spell as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
            name,
            ritual,
            spell_level,
            school,
            casting_time,
            range,
            duration,
            body,
            components,
            higher_levels,
        } = val;
        let components = components.into();
        Out {
            name,
            ritual,
            spell_level,
            school,
            casting_time,
            range,
            duration,
            body,
            components,
            higher_levels,
            casters: Vec::new(),
        }
    }
}

impl From<Out> for In {
    fn from(val: Out) -> Self {
        let Out {
            name,
            ritual,
            spell_level,
            school,
            casting_time,
            range,
            duration,
            body,
            components,
            higher_levels,
            ..
        } = val;
        let components = components.into();
        In {
            name,
            ritual,
            spell_level,
            school,
            casting_time,
            range,
            duration,
            body,
            components,
            higher_levels,
        }
    }
}

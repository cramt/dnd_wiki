pub mod componenets;

use crate::in_model::spell::Spell as In;
use crate::out_model::spell::Spell as Out;

impl Into<Out> for In {
    fn into(self) -> Out {
        let Self {
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
        } = self;
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

impl Into<In> for Out {
    fn into(self) -> In {
        let Self {
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
        } = self;
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

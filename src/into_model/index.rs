use crate::in_model::index::Index as In;
use crate::out_model;
use crate::out_model::index::Index as Out;

impl Into<Out> for In {
    fn into(self) -> Out {
        let Self {
            classes,
            spells,
            style,
            static_folder,
        } = self;
        let classes = classes.into_iter().map(|x| x.into()).collect::<Vec<out_model::class::Class>>();
        let spells = spells.into_iter().map(|x|  {
            let mut x: out_model::spell::Spell = x.into();
            for class in &classes {
                if class.spell_list.contains(&x.name) {
                    x.casters.push(class.name.clone());
                }
            }
            x
        }).collect();
        Out {
            classes,
            spells,
            style,
            static_folder,
        }
    }
}

impl Into<In> for Out {
    fn into(self) -> In {
        let Self {
            classes,
            spells,
            style,
            static_folder,
        } = self;
        let classes = classes.into_iter().map(|x| x.into()).collect();
        let spells = spells.into_iter().map(|x| x.into()).collect();
        In {
            classes,
            spells,
            style,
            static_folder,
        }
    }
}

use crate::in_model::index::Index as In;
use crate::out_model;
use crate::out_model::index::Index as Out;

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
            classes,
            spells,
            style,
            static_folder,
            schools,
        } = val;
        let classes = classes
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<out_model::class::Class>>();
        let spells = spells
            .into_iter()
            .map(|x| {
                let mut x: out_model::spell::Spell = x.into();
                if !schools.contains(&x.school) {
                    panic!("school {} does not exist", x.school)
                }
                for class in &classes {
                    if class.spell_list.contains(&x.name) {
                        x.casters.push(class.name.clone());
                    }
                }
                x
            })
            .collect();
        Out {
            classes,
            spells,
            style,
            static_folder,
            schools,
        }
    }
}

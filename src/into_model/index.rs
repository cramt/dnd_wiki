use crate::in_model::index::Index as In;
use crate::out_model;
use crate::out_model::class::Classes;
use crate::out_model::index::Index as Out;
use crate::out_model::spell::Spells;

impl From<In> for Out {
    fn from(val: In) -> Self {
        let In {
            classes,
            spells,
            style,
            static_folder,
            schools,
            name,
            feats,
            races,
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
        let spells = Spells(spells);
        let classes = Classes(classes);
        let feats = (feats, "feats".to_string()).into();
        let races = races.into();
        Out {
            classes,
            spells,
            style,
            static_folder,
            schools,
            name,
            feats,
            races,
        }
    }
}

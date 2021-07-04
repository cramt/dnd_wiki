use crate::engine_generator;
use crate::out_model::spell::Spell;

engine_generator!(
    Vec<Spell> >>
    spells >>
    sort_spells,
    classes_of_spell,
    casters
);

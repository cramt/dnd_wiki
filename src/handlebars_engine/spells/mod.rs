use crate::engine_generator;
use crate::out_model::spell::Spells;

engine_generator!(Spells >> spells >> sort_spells, classes_of_spell, casters);

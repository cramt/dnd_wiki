use crate::{engine_generator, out_model::class::subclass::Subclass};

engine_generator!(
    Subclass >> subclass >> is_caster,
    has_resources,
    max_spell_slot,
    cantrips_known,
    spell_slots
);

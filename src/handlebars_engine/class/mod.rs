use crate::engine_generator;
use crate::out_model::class::Class;

engine_generator!(
    Class >> class >> equipment_format,
    multi_class_req_gen,
    starting_prof_format,
    has_cantrips,
    cantrips_known,
    spell_slots,
    has_spells,
    max_spell_slot,
    features_at_level
);

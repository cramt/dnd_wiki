use crate::engine_generator;
use crate::model::class::Class;

engine_generator!(
    Class >> class >> avg_die,
    equipment_format,
    lowercase,
    markdown,
    multi_class_req_gen,
    starting_prof_format,
    has_cantrips,
    cantrips_known,
    spell_slots,
    has_spells,
    max_spell_slot,
    features_at_level
);

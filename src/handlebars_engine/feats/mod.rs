use crate::engine_generator;
use crate::out_model::featlikes::Featlikes;

engine_generator!(
    Featlikes >> featlikes >> has_prerequisites,
    format_prerequisites,
    sort_feats
);

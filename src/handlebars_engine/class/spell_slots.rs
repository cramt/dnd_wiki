use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde::de::IntoDeserializer;
use crate::model::class::Class;
use serde::Deserialize;
use crate::model::class::caster_type::CasterType;

pub fn spell_slots(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let class = Class::deserialize(h.param(0)
        .ok_or(RenderError::new("param not found"))?
        .value()
        .clone()
        .into_deserializer()).map_err(|x| RenderError::new(x.to_string()))?;
    let class_level = h.param(1)
        .ok_or(RenderError::new("param not found"))?
        .value().as_i64().ok_or_else(|| RenderError::new("not i64"))? as u8;
    let spell_level = h.param(2)
        .ok_or(RenderError::new("param not found"))?
        .value().as_i64().ok_or_else(|| RenderError::new("not i64"))? as u8;
    let slots = match class.caster_type {
        CasterType::Full => match (class_level, spell_level) {
            (1, 1) => 2,
            (2, 1) => 3,
            (3, 1) => 4,
            (3, 2) => 2,
            (4, 1) => 4,
            (4, 2) => 3,
            (5, 1) => 4,
            (5, 2) => 3,
            (5, 3) => 2,
            (6, 1) => 4,
            (6, 2) => 3,
            (6, 3) => 3,
            (7, 1) => 4,
            (7, 2) => 3,
            (7, 3) => 3,
            (7, 4) => 1,
            (8, 1) => 4,
            (8, 2) => 3,
            (8, 3) => 3,
            (8, 4) => 2,
            (9, 1) => 4,
            (9, 2) => 3,
            (9, 3) => 3,
            (9, 4) => 3,
            (9, 5) => 1,
            (10, 1) => 4,
            (10, 2) => 3,
            (10, 3) => 3,
            (10, 4) => 3,
            (10, 5) => 2,
            (11, 1) => 4,
            (11, 2) => 3,
            (11, 3) => 3,
            (11, 4) => 3,
            (11, 5) => 2,
            (11, 6) => 1,
            (12, 1) => 4,
            (12, 2) => 3,
            (12, 3) => 3,
            (12, 4) => 3,
            (12, 5) => 2,
            (12, 6) => 1,
            (13, 1) => 4,
            (13, 2) => 3,
            (13, 3) => 3,
            (13, 4) => 3,
            (13, 5) => 2,
            (13, 6) => 1,
            (13, 7) => 1,
            (14, 1) => 4,
            (14, 2) => 3,
            (14, 3) => 3,
            (14, 4) => 3,
            (14, 5) => 2,
            (14, 6) => 1,
            (14, 7) => 1,
            (15, 1) => 4,
            (15, 2) => 3,
            (15, 3) => 3,
            (15, 4) => 3,
            (15, 5) => 2,
            (15, 6) => 1,
            (15, 7) => 1,
            (15, 8) => 1,
            (16, 1) => 4,
            (16, 2) => 3,
            (16, 3) => 3,
            (16, 4) => 3,
            (16, 5) => 2,
            (16, 6) => 1,
            (16, 7) => 1,
            (16, 8) => 1,
            (17, 1) => 4,
            (17, 2) => 3,
            (17, 3) => 3,
            (17, 4) => 3,
            (17, 5) => 2,
            (17, 6) => 1,
            (17, 7) => 1,
            (17, 8) => 1,
            (17, 9) => 1,
            (18, 1) => 4,
            (18, 2) => 3,
            (18, 3) => 3,
            (18, 4) => 3,
            (18, 5) => 3,
            (18, 6) => 1,
            (18, 7) => 1,
            (18, 8) => 1,
            (18, 9) => 1,
            (19, 1) => 4,
            (19, 2) => 3,
            (19, 3) => 3,
            (19, 4) => 3,
            (19, 5) => 3,
            (19, 6) => 2,
            (19, 7) => 1,
            (19, 8) => 1,
            (19, 9) => 1,
            (20, 1) => 4,
            (20, 2) => 3,
            (20, 3) => 3,
            (20, 4) => 3,
            (20, 5) => 3,
            (20, 6) => 2,
            (20, 7) => 2,
            (20, 8) => 1,
            (20, 9) => 1,
            _ => 0,
        }
        CasterType::Half => match (class_level, spell_level) {
            (2, 1) => 2,
            (3, 1) => 3,
            (4, 1) => 3,
            (5, 1) => 4,
            (5, 2) => 2,
            (6, 1) => 4,
            (6, 2) => 2,
            (7, 1) => 4,
            (7, 2) => 3,
            (8, 1) => 4,
            (8, 2) => 3,
            (9, 1) => 4,
            (9, 2) => 3,
            (9, 3) => 2,
            (10, 1) => 4,
            (10, 2) => 3,
            (10, 3) => 2,
            (11, 1) => 4,
            (11, 2) => 3,
            (11, 3) => 3,
            (12, 1) => 4,
            (12, 2) => 3,
            (12, 3) => 3,
            (13, 1) => 4,
            (13, 2) => 3,
            (13, 3) => 3,
            (13, 4) => 1,
            (14, 1) => 4,
            (14, 2) => 3,
            (14, 3) => 3,
            (14, 4) => 1,
            (15, 1) => 4,
            (15, 2) => 3,
            (15, 3) => 3,
            (15, 4) => 2,
            (16, 1) => 4,
            (16, 2) => 3,
            (16, 3) => 3,
            (16, 4) => 2,
            (17, 1) => 4,
            (17, 2) => 3,
            (17, 3) => 3,
            (17, 4) => 3,
            (17, 5) => 1,
            (18, 1) => 4,
            (18, 2) => 3,
            (18, 3) => 3,
            (18, 4) => 3,
            (18, 5) => 1,
            (19, 1) => 4,
            (19, 2) => 3,
            (19, 3) => 3,
            (19, 4) => 3,
            (19, 5) => 2,
            (20, 1) => 4,
            (20, 2) => 3,
            (20, 3) => 3,
            (20, 4) => 3,
            (20, 5) => 2,
            _ => 0
        }
        CasterType::Artificer => match (class_level, spell_level) {
            (1, 1) => 2,
            (2, 1) => 2,
            (3, 1) => 3,
            (4, 1) => 3,
            (5, 1) => 4,
            (5, 2) => 2,
            (6, 1) => 4,
            (6, 2) => 2,
            (7, 1) => 4,
            (7, 2) => 3,
            (8, 1) => 4,
            (8, 2) => 3,
            (9, 1) => 4,
            (9, 2) => 3,
            (9, 3) => 2,
            (10, 1) => 4,
            (10, 2) => 3,
            (10, 3) => 2,
            (11, 1) => 4,
            (11, 2) => 3,
            (11, 3) => 3,
            (12, 1) => 4,
            (12, 2) => 3,
            (12, 3) => 3,
            (13, 1) => 4,
            (13, 2) => 3,
            (13, 3) => 3,
            (13, 4) => 1,
            (14, 1) => 4,
            (14, 2) => 3,
            (14, 3) => 3,
            (14, 4) => 1,
            (15, 1) => 4,
            (15, 2) => 3,
            (15, 3) => 3,
            (15, 4) => 2,
            (16, 1) => 4,
            (16, 2) => 3,
            (16, 3) => 3,
            (16, 4) => 2,
            (17, 1) => 4,
            (17, 2) => 3,
            (17, 3) => 3,
            (17, 4) => 3,
            (17, 5) => 1,
            (18, 1) => 4,
            (18, 2) => 3,
            (18, 3) => 3,
            (18, 4) => 3,
            (18, 5) => 1,
            (19, 1) => 4,
            (19, 2) => 3,
            (19, 3) => 3,
            (19, 4) => 3,
            (19, 5) => 2,
            (20, 1) => 4,
            (20, 2) => 3,
            (20, 3) => 3,
            (20, 4) => 3,
            (20, 5) => 2,
            _ => 0
        }
        CasterType::None => 0
    };
    let slots = match slots {
        0 => "-".to_string(),
        _x => _x.to_string()
    };
    out.write(slots.as_str())?;
    Ok(())
}

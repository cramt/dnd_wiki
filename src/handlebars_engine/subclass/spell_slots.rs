use crate::{
    handlebars_engine::deserialize_context::deserialize_context,
    out_model::class::subclass::Subclass,
};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn spell_slots(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let class = deserialize_context::<Subclass>(ctx)?.inner;
    let class_level = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_i64()
        .ok_or_else(|| RenderError::new("not i64"))? as u8;
    let spell_level = h
        .param(1)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_i64()
        .ok_or_else(|| RenderError::new("not i64"))? as u8;
    let slots = class.caster_type.slots_at(class_level, spell_level);
    let slots = match slots {
        0 => "-".to_string(),
        _x => _x.to_string(),
    };
    out.write(slots.as_str())?;
    Ok(())
}

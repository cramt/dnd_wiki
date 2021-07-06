use crate::out_model::class::subclass::Subclass;
use crate::{
    handlebars_engine::deserialize_context::deserialize_context,
    out_model::class::generic_class::GenericClass,
};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn cantrips_known(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let class = deserialize_context::<Subclass>(ctx)?.inner;
    let level = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_i64()
        .ok_or_else(|| RenderError::new("not i64"))? as u8;
    out.write(class.cantrips_known_at_level(level).to_string().as_str())?;
    Ok(())
}

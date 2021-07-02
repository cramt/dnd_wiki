use crate::handlebars_engine::deserialize_context::deserialize_context;
use crate::out_model::class::Class;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn cantrips_known(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let class = deserialize_context::<Class>(ctx)?.inner;
    let level = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_i64()
        .ok_or_else(|| RenderError::new("not i64"))? as u8;
    if let Some(mut i) = class.start_cantrips_known {
        if level > 3 {
            i += 1
        }
        if level > 9 {
            i += 1
        }
        out.write(i.to_string().as_str())?;
    }
    Ok(())
}

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

use crate::{handlebars_engine::deserialize_context::deserialize_metadata, markdown};

pub fn markdown(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let str = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_str()
        .ok_or_else(|| RenderError::new("param not found"))?;
    let metadata = deserialize_metadata(ctx)?;
    let str = markdown::markdown(str, &metadata.references);
    out.write(str.as_str())?;
    Ok(())
}

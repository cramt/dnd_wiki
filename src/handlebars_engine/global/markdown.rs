use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

use crate::markdown;

pub fn markdown(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let str = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_str()
        .ok_or_else(|| RenderError::new("param not found"))?;
    let str = markdown::markdown(str);
    out.write(str.as_str())?;
    Ok(())
}

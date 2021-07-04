
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

use crate::text_utils::class_name_sanitize;

pub fn class_name(
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
        .ok_or_else(|| RenderError::new("not string"))?;
    let str = class_name_sanitize(str);
    out.write(&str)?;
    Ok(())
}

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn proper_noun(
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
    let str = crate::text_utils::proper_noun(str);
    out.write(&str)?;
    Ok(())
}

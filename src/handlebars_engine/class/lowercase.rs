use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn lowercase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    out.write(
        h.param(0)
            .ok_or(RenderError::new("param not found"))?
            .value()
            .as_str()
            .ok_or(RenderError::new("not string"))?
            .to_lowercase()
            .as_str(),
    )?;
    Ok(())
}

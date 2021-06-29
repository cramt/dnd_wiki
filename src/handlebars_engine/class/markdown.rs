use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

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

    out.write(
        str.replace("\r\n", "<br>")
            .replace("\n", "<br>")
            .replace("\r", "<br>")
            .as_str(),
    )?;
    Ok(())
}

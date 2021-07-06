use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn avg_die(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let n = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_u64()
        .ok_or_else(|| RenderError::new("not number"))?;
    let n = (n / 2) + 1;
    out.write(n.to_string().as_str())?;
    Ok(())
}

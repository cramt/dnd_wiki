use handlebars::{Helper, Handlebars, RenderContext, Context, Output, HelperResult, RenderError};

pub fn ordinal(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let n = h
        .param(0)
        .ok_or(RenderError::new("param not found"))?
        .value()
        .as_i64()
        .ok_or(RenderError::new("param not found"))?;

    out.write(
        match n {
            1 => "1st".to_string(),
            2 => "2nd".to_string(),
            3 => "3rd".to_string(),
            _x=> format!("{}th", _x)
        }.as_str()
    )?;
    Ok(())
}

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn spell_level_name(
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
        .ok_or_else(|| RenderError::new("param not found"))?;

    out.write(
        match n {
            0 => "cantrip".to_string(),
            1 => "1st level".to_string(),
            2 => "2nd level".to_string(),
            3 => "3rd level".to_string(),
            _x => format!("{}th level", _x),
        }
        .as_str(),
    )?;
    Ok(())
}

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
    let str = str
        .split_inclusive(&['-', ' ', '/'][..])
        .map(|x| format!("{}{}", x[0..1].to_uppercase(), &x[1..]))
        .collect::<String>();
    out.write(&str)?;
    Ok(())
}

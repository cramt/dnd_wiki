use crate::text_utils::prepend_singular_definite_article;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

pub fn equipment_format(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let json = h
        .param(0)
        .ok_or_else(||RenderError::new("param not found"))?
        .value();

    let str: String = json.as_array()
        .ok_or_else(|| RenderError::new("not array"))?
        .iter().map(|x|x.as_str().unwrap()).map(prepend_singular_definite_article).collect::<Vec<String>>().join(" or ");
    out.write(str.as_str())?;
    Ok(())
}

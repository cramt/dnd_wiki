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
        .ok_or(RenderError::new("param not found"))?
        .value();

    out.write(
        match (json.as_str(), json.as_array()) {
            (Some(str), None) => prepend_singular_definite_article(str),
            (None, Some(arr)) => {
                let a = arr
                    .get(0)
                    .ok_or(RenderError::new("array not of min size 2"))?
                    .as_str()
                    .ok_or(RenderError::new("array index 0 not string"))?;
                let b = arr
                    .get(1)
                    .ok_or(RenderError::new("array not of min size 2"))?
                    .as_str()
                    .ok_or(RenderError::new("array index 0 not string"))?;
                format!(
                    "{} or {}",
                    prepend_singular_definite_article(a),
                    prepend_singular_definite_article(b)
                )
            }
            _ => return Err(RenderError::new("not string or array")),
        }
        .as_str(),
    )?;
    Ok(())
}

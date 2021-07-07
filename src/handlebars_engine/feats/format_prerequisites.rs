use crate::{
    handlebars_engine::render_err::render_err, out_model::featlikes::prerequisites::Prerequisites,
};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde::{de::IntoDeserializer, Deserialize};

pub fn format_prerequisites(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let prerequisite = Prerequisites::deserialize(
        h.param(0)
            .ok_or_else(|| RenderError::new("param not found"))?
            .value()
            .clone()
            .into_deserializer(),
    )
    .map_err(render_err)?;
    let str = prerequisite.to_string();
    out.write(str.as_str())?;
    Ok(())
}

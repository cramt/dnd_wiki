use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde::{de::IntoDeserializer, Deserialize};

use crate::out_model::spell::components::Components;

pub fn format_componenets(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let component = Components::deserialize(
        h.param(0)
            .ok_or_else(||RenderError::new("param not found"))?
            .value()
            .clone()
            .into_deserializer(),
    )
    .map_err(|x| RenderError::new(x.to_string()))?;
    out.write(component.to_string().as_str())?;
    Ok(())
}

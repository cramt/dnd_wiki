use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde::de::IntoDeserializer;
use crate::model::class::Class;
use serde::Deserialize;

pub fn feature_table_list(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let class = Class::deserialize(h.param(0)
        .ok_or(RenderError::new("param not found"))?
        .value()
        .clone()
        .into_deserializer()).map_err(|x| RenderError::new(x.to_string()))?;
    let level = h.param(1)
        .ok_or(RenderError::new("param not found"))?
        .value().as_i64().ok_or_else(|| RenderError::new("not i64"))? as u8;
    let str = class.features.iter().filter(|x| x.level == level).map(|x|x.name.to_string()).collect::<Vec<String>>().join(", ");
    out.write(str.as_str())?;
    Ok(())
}

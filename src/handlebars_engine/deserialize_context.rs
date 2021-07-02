use handlebars::{Context, RenderError};
use serde::{de::IntoDeserializer, Deserialize};

use crate::out_model::index::{Metadata, MetadataWrapper};

use super::render_err::render_err;

pub fn deserialize_context<'rc, 'de, T>(
    ctx: &'rc Context,
) -> Result<MetadataWrapper<T>, RenderError>
where
    T: Deserialize<'de>,
{
    MetadataWrapper::<T>::deserialize(ctx.data().clone().into_deserializer()).map_err(render_err)
}

pub fn deserialize_metadata<'rc, 'de>(ctx: &'rc Context) -> Result<Metadata, RenderError> {
    let data = ctx.data().get("metadata").unwrap().clone();
    Metadata::deserialize(data.into_deserializer()).map_err(render_err)
}

use std::ops::Deref;

use dnd_wiki_markdown::options::Options;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};

use crate::handlebars_engine::deserialize_context::deserialize_metadata;

pub fn markdown(
    h: &Helper,
    _: &Handlebars,
    ctx: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let str = h
        .param(0)
        .ok_or_else(|| RenderError::new("param not found"))?
        .value()
        .as_str()
        .ok_or_else(|| RenderError::new("param not found"))?;
    let metadata = deserialize_metadata(ctx)?;
    let str = dnd_wiki_markdown::markdown_opts(str, metadata.index.deref(), Options {
        link_prefix: metadata.path_to_index
    })
        .map_err(|e| RenderError::new(e.to_string()))?;
    out.write(str.as_str())?;
    Ok(())
}

use std::ops::Deref;

use dnd_wiki_markdown::options::Options;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::handlebars_engine::deserialize_context::deserialize_metadata;

static ROOT_TAG_STRIP: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^<[a-z]*>((?:.*|\s)*)</[a-z]*>$").unwrap());

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
    let inline = h
        .param(1)
        .map(|x| x.value().as_bool())
        .flatten()
        .unwrap_or(false);
    let metadata = deserialize_metadata(ctx)?;
    let str = dnd_wiki_markdown::markdown_opts(
        str,
        metadata.index.deref(),
        Options {
            link_prefix: metadata.path_to_index,
        },
    )
    .map_err(|e| RenderError::new(e.to_string()))?;
    let str = if inline {
        match ROOT_TAG_STRIP.captures(str.as_str()) {
            Some(x) => x[1].to_string(),
            None => str,
        }
    } else {
        str
    };
    out.write(str.as_str())?;
    Ok(())
}

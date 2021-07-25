mod markdown_error;
pub mod options;
pub mod referencer;
#[cfg(test)]
mod test;

use std::ops::Deref;

use markdown_error::MarkdownError;
use options::Options;
use percent_encoding::percent_decode_str;
use pulldown_cmark::{html, CowStr, Event, Parser, Tag};
use referencer::Referencer;

fn reference<R: Referencer>(r: &R, c: CowStr<'_>) -> Result<String, Vec<String>> {
    let path = percent_decode_str(c.deref()).decode_utf8_lossy();
    let path = path.trim().split(&['.', '/'][..]);
    r.traverse(path.clone())
        .map(|x| x.value().to_string())
        .ok_or_else(|| path.map(|x| x.to_string()).collect())
}

pub fn markdown_opts<S: AsRef<str>, R: Referencer>(
    s: S,
    r: R,
    options: Options,
) -> Result<String, MarkdownError> {
    let mut err = None;
    let parser = Parser::new_ext(s.as_ref(), pulldown_cmark::Options::empty()).map(|x| match x {
        Event::Start(Tag::Link(tt, link, alt)) => {
            let l = match reference(&r, link) {
                Ok(x) => x,
                Err(x) => {
                    err = Some(MarkdownError::UnknownReference {
                        name: alt.to_string(),
                        path: x,
                    });
                    String::new()
                }
            };
            Event::Start(Tag::Link(
                tt,
                format!("{}{}", &options.link_prefix, l).into(),
                alt,
            ))
        }
        _ => x,
    });
    let mut out = String::with_capacity(s.as_ref().len() * 2 / 3);
    html::push_html(&mut out, parser);
    match err {
        Some(x) => Err(x),
        None => Ok(out.trim().replace("\n", "\r\n")),
    }
}

pub fn markdown<S: AsRef<str>, R: Referencer>(s: S, r: R) -> Result<String, MarkdownError> {
    markdown_opts(s, r, Options::default())
}

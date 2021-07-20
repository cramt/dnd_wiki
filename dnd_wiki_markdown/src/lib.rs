mod markdown_error;
pub mod referencer;
#[cfg(test)]
mod test;
pub mod options;

use std::ops::Deref;

use markdown_error::MarkdownError;
use options::Options;
use percent_encoding::percent_decode_str;
use pulldown_cmark::{html, CowStr, Event, Parser, Tag};
use referencer::Referencer;

fn reference<R: Referencer>(r: &R, c: CowStr<'_>) -> Option<String> {
    r.traverse(percent_decode_str(c.deref()).decode_utf8_lossy().trim().split(&['.', '/'][..]))
        .map(|x| x.value().to_string())
}


pub fn markdown_opts<S: AsRef<str>, R: Referencer>(s: S, r: R, options: Options) -> Result<String, MarkdownError> { 
    let parser = Parser::new_ext(s.as_ref(), pulldown_cmark::Options::empty()).map(|x| match x {
        Event::Start(Tag::Link(tt, link, alt)) => {
            Event::Start(Tag::Link(tt, format!("{}{}", &options.link_prefix, reference(&r, link).unwrap()).into(), alt))
        },
        _ => x,
    });
    let mut out = String::with_capacity(s.as_ref().len() * 2 / 3);
    html::push_html(&mut out, parser);
    Ok(out.trim().replace("\n", "\r\n"))
}

pub fn markdown<S: AsRef<str>, R: Referencer>(s: S, r: R) -> Result<String, MarkdownError> {
    markdown_opts(s, r, Options::default())
}

use std::borrow::Cow;

use regex::Captures;

use crate::{markdown_error::MarkdownError, referencer::Referencer, regexs};

pub fn link<'a, R>(s: &'a str, references: &R) -> Result<Cow<'a, str>, MarkdownError>
where
    R: Referencer,
{
    let mut fail = None;
    let r = regexs::link().replace_all(s, |caps: &Captures| {
        let v = caps[2].trim().split(&['.', '/'][..]);
        if let Some(r) = references.traverse(v.clone()) {
            format!(
                r#"<a class="markdown" href="./{}">{}</a>"#,
                r.value(),
                &caps[1]
            )
        } else {
            fail = Some((caps[1].to_string(), v.map(|x|x.to_string()).collect()));
            String::new()
        }
    });
    match fail {
        Some(x) => Err(MarkdownError::UnknownReference {
            name: x.0,
            path: x.1,
        }),
        None => Ok(r)
    }
}

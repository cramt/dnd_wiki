use std::borrow::Cow;

use regex::Captures;

use super::regexs;

pub fn list(s: &str) -> Cow<str> {
    regexs::list().replace_all(s.as_ref(), |caps: &Captures| {
        format!(
            r#"<ul class="markdown">{}</ul>"#,
            regexs::list_entry().replace_all(
                caps.get(0).unwrap().as_str(),
                r#"<li class="markdown">$1</li>"#
            )
        )
    })
}

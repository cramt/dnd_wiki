use std::borrow::Cow;

use regex::Captures;

use crate::out_model::references::References;

use super::regexs;

pub fn link<'a, 'b>(s: &'a str, references: &'b References) -> Option<Cow<'a, str>> {
    let mut fail = false;
    let r = regexs::link().replace_all(s, |caps: &Captures| {
        let mut v = caps[1].trim().split(&['.', '/'][..]);
        let mut name = match v.next() {
            Some(x) => x,
            None => {
                fail = true;
                return String::new();
            }
        };
        let mut r = match references.get(name) {
            Some(x) => x,
            None => {
                fail = true;
                return String::new();
            }
        };
        for s in v {
            name = s;
            r = match r.children.get(s) {
                Some(x) => x,
                None => {
                    fail = true;
                    return String::new();
                }
            }
        }
        format!(r#"<a class="markdown" href="./{}">{}</a>"#, r.link, name)
    });
    match fail {
        false => Some(r),
        true => None,
    }
}

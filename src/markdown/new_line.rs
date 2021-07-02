use std::borrow::Cow;

use super::regexs;

pub fn new_line<'a>(s: &'a str) -> Cow<'a, str> {
    regexs::new_line().replace_all(s, "<br>")
}
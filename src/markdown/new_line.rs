use std::borrow::Cow;

use super::regexs;

pub fn new_line(s: &str) -> Cow<str> {
    regexs::new_line().replace_all(s, "<br>")
}
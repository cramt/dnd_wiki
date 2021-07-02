use std::ops::Deref;

use once_cell::sync::Lazy;
use regex::Regex;

pub fn bold_and_italic() -> &'static Regex {
    static BOLD_AND_ITALIC_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([^\*]|^)\*\*\*([^\*]+)\*\*\*([^\*]|$)").unwrap());
    BOLD_AND_ITALIC_REGEX.deref()
}

pub fn bold() -> &'static Regex {
    static BOLD_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([^\*]|^)\*\*([^\*]+)\*\*([^\*]|$)").unwrap());
    BOLD_REGEX.deref()
}

pub fn italic() -> &'static Regex {
    static BOLD_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([^\*]|^)\*([^\*]+)\*([^\*]|$)").unwrap());
    BOLD_REGEX.deref()
}

pub fn list() -> &'static Regex {
    static LIST_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?:(?:\r\n|\r|\n|^)\s*\.\s.*){1,}").unwrap());
    LIST_REGEX.deref()
}

pub fn new_line() -> &'static Regex {
    static NEW_LINE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\r\n|\r|\n").unwrap());
    NEW_LINE_REGEX.deref()
}

pub fn list_entry() -> &'static Regex {
    static LIST_ENTRY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^.]?\.\s(.*)").unwrap());
    LIST_ENTRY_REGEX.deref()
}

use std::borrow::Cow;

use std::path::PathBuf;

use once_cell::sync::Lazy;
use regex::Regex;

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

pub fn prepend_singular_definite_article<'a, C: Into<Cow<'a, str>>>(c: C) -> String {
    let c = c.into();
    if let Some(char) = c.as_ref().chars().next() {
        let article = if is_vowel(char) { "an" } else { "a" };
        format!("{} {}", article, c)
    } else {
        String::new()
    }
}

pub fn num_to_word(n: u8) -> String {
    num_to_words::integer_to_en_us(n as i64).unwrap()
}

pub fn path_normalize(path: PathBuf) -> PathBuf {
    #[cfg(target_os = "windows")]
    fn internal(s: &str) -> String {
        s[4..].replace("/", "\\")
    }
    #[cfg(not(target_os = "windows"))]
    fn internal(s: &str) -> String {
        s.replace("\\", "//")
    }

    let str = path.to_str().unwrap();
    internal(str).into()
}

pub fn file_name_sanitize<S: AsRef<str>>(s: S) -> String {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(?:/|<|>|:|"|\\|\||\?|\*)"#).unwrap());
    REGEX.replace_all(s.as_ref(), "_").into()
}

pub fn class_name_sanitize<S: AsRef<str>>(s: S) -> String{
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^a-zA-Z0-9]").unwrap());
    REGEX.replace_all(s.as_ref(), "").into()
}
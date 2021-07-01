use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

pub fn markdown<'a, S: Into<Cow<'a, str>>>(s: S) -> String {
    static BOLD_ITALIC_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([^\*]|^)(\*+)([^\*]+)(\*+)([^\*]|$)").unwrap());
    static LIST_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(?:(?:\r\n|\r|\n|^)\s*\.\s.*){2,}").unwrap());
    static NEW_LINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\r\n|\r|\n").unwrap());

    let s: Cow<str> = s.into();
    let s = s.trim();
    let s = LIST_REGEX.replace_all(s.as_ref(), |caps: &Captures| {
        static ENTRY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^.]?\.\s(.*)").unwrap());
        format!(
            r#"<ul class="markdown">{}</ul>"#,
            ENTRY_REGEX.replace_all(
                caps.get(0).unwrap().as_str(),
                r#"<li class="markdown">$1</li>"#
            )
        )
    });
    let s = NEW_LINE.replace_all(s.as_ref(), "<br>");
    BOLD_ITALIC_REGEX
        .replace_all(s.as_ref(), |caps: &Captures| {
            let class_name = match (caps[2].len(), caps[4].len()) {
                (1, 1) => "italic",
                (2, 2) => "bold",
                (3, 3) => "italic bold",
                (_a, _b) => panic!("markdown syntax between {}, has {} * on the left side and {} * on the right side", &caps[3], _a, _b),
            };
            format!(
                r#"{}<span class="{}">{}</span>{}"#,
                &caps[1], class_name, &caps[3], &caps[5]
            )
        })
        .into()
}

#[cfg(test)]
mod test {
    use shoulda::Shoulda;

    use super::markdown;

    #[test]
    fn bold() {
        markdown("**bold**".to_string())
            .should()
            .eq(r#"<span class="bold">bold</span>"#.to_string());

        markdown("aa**bold**aa".to_string())
            .should()
            .eq(r#"aa<span class="bold">bold</span>aa"#.to_string());
    }

    #[test]
    fn italic() {
        markdown("*aa*".to_string())
            .should()
            .eq(r#"<span class="italic">aa</span>"#.to_string());

        markdown("bb*aa*bb".to_string())
            .should()
            .eq(r#"bb<span class="italic">aa</span>bb"#.to_string());
    }

    #[test]
    fn both() {
        markdown("boring *cool italic* boring **cool bold** boring ***cool both*** boring".to_string()).should().eq("boring <span class=\"italic\">cool italic</span> boring <span class=\"bold\">cool bold</span> boring <span class=\"italic bold\">cool both</span> boring".to_string());
    }

    #[test]
    fn list() {
        markdown(
            r"
aaa
. first thing
. second thing
. third thing
aaa
        "
            .to_string(),
        )
        .should()
        .eq(
            "aaa<ul class=\"markdown\"><li class=\"markdown\">first thing</li><li class=\"markdown\">second thing</li><li class=\"markdown\">third thing</li></ul><br>aaa"
                .to_string(),
        );
    }

    #[test]
    fn list_more() {
        markdown(
            r"
hello there
aaa
. first thing
. second thing
. third thing
aaa
this is awesome
        "
            .to_string(),
        )
        .should()
        .eq(
            "hello there<br>aaa<ul class=\"markdown\"><li class=\"markdown\">first thing</li><li class=\"markdown\">second thing</li><li class=\"markdown\">third thing</li></ul><br>aaa<br>this is awesome"
                .to_string(),
        );
    }

    #[test]
    fn newlist() {
        markdown(
            r"
a
b
c
        "
            .to_string(),
        )
        .should()
        .eq("a<br>b<br>c".to_string());
    }
}

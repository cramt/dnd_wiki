use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

pub fn markdown<'a, S: Into<Cow<'a, str>>>(s: S) -> String {
    static BOLD_ITALIC_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([^\*]|^)(\*+)([^\*]+)(\*+)([^\*]|$)").unwrap());
    let s = s.into();
    let s = s
        .replace("\r\n", "<br>")
        .replace("\n", "<br>")
        .replace("\r", "<br>");
    BOLD_ITALIC_REGEX
        .replace_all(s.as_str(), |caps: &Captures| {
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
}

mod list;
mod new_line;
mod regexs;
mod style_formatting;

use std::borrow::Cow;

use self::{list::list, new_line::new_line, style_formatting::style_formatting};

pub fn markdown<'a, S: Into<Cow<'a, str>>>(s: S) -> String {
    let s: Cow<str> = s.into();
    let s = s.trim();
    let s = list(s);
    let s = new_line(s.as_ref());
    style_formatting(s.as_ref())
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

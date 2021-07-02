use super::regexs;

pub fn style_formatting<'a>(s: &'a str) -> String {
    let s = regexs::bold_and_italic().replace(s, r#"$1<span class="italic bold">$2</span>$3"#);
    let s = regexs::bold().replace(s.as_ref(), r#"$1<span class="bold">$2</span>$3"#);
    regexs::italic().replace(s.as_ref(), r#"$1<span class="italic">$2</span>$3"#).to_string()
}

use std::collections::HashMap;

use shoulda::Shoulda;

use crate::markdown;

use self::{empty_referencer::EmptyReferencer, hashmap_referencer::HashMapReferencer};

mod empty_referencer;
mod hashmap_referencer;

#[test]
fn all_text_bold() {
    markdown("**damn, this is bold**", EmptyReferencer)
        .should()
        .eq(Ok(
            "<span class=\"bold\">damn, this is bold</span>".to_string(),
        ));
}

#[test]
fn inner_text_bold() {
    markdown("yo **bold** it be", EmptyReferencer)
        .should()
        .eq(Ok(
            "yo <span class=\"bold\">bold</span> it be".to_string(),
        ));
}

#[test]
fn all_text_italic() {
    markdown("*damn, italy*", EmptyReferencer).should().eq(Ok(
        "<span class=\"italic\">damn, italy</span>".to_string(),
    ));
}

#[test]
fn inner_text_italic() {
    markdown("some *wild* shit", EmptyReferencer)
        .should()
        .eq(Ok(
            "some <span class=\"italic\">wild</span> shit".to_string(),
        ));
}

#[test]
fn all_text_italic_and_bold() {
    markdown("***AAA***", EmptyReferencer)
        .should()
        .eq(Ok("<span class=\"italic bold\">AAA</span>".to_string()));
}

#[test]
fn inner_text_italic_and_bold() {
    markdown("bruh, ***IDFK*** anymore", EmptyReferencer)
        .should()
        .eq(Ok(
            "bruh, <span class=\"italic bold\">IDFK</span> anymore".to_string(),
        ));
}

#[test]
fn all_text_list() {
    markdown(
        "
. First things first
. I'ma say all the words inside my head
. I'm fired up and tired of
. The way that things have been, oh-ooh
. The way that things have been, oh-ooh
    ",
        EmptyReferencer,
    )
    .should()
    .eq(Ok(
        "<ul class=\"markdown\"><li class=\"markdown\">First things first</li><li class=\"markdown\">I\'ma say all the words inside my head</li><li class=\"markdown\">I\'m fired up and tired of</li><li class=\"markdown\">The way that things have been, oh-ooh</li><li class=\"markdown\">The way that things have been, oh-ooh</li></ul>"
            .to_string(),
    ));
}

#[test]
fn inner_text_list() {
    markdown("
things that are cool
. you
. me
. other things
bye
    ", EmptyReferencer).should().eq(Ok("things that are cool<ul class=\"markdown\"><li class=\"markdown\">you</li><li class=\"markdown\">me</li><li class=\"markdown\">other things</li></ul><br>bye".to_string()));
}

#[test]
fn new_line() {
    markdown(
        "
yoo
new line
    ",
        EmptyReferencer,
    )
    .should()
    .eq(Ok("yoo<br>new line".to_string()));
}

#[test]
fn list_and_newline() {
    markdown("
yoo
hello
. list time
. aaa
    ", EmptyReferencer).should().eq(Ok("yoo<br>hello<ul class=\"markdown\"><li class=\"markdown\">list time</li><li class=\"markdown\">aaa</li></ul>".to_string()));
}

#[test]
fn references() {
    let r = HashMapReferencer {
        value: "".to_string(),
        children: vec![
            ("me".to_string(),
            HashMapReferencer {
                value: "things".to_string(),
                children: HashMap::new(),
            }),
        ]
        .into_iter()
        .collect(),
    };

    markdown("[[Me=me]]", r).should().be().eq(Ok("<a class=\"markdown\" href=\"./things\">Me</a>".to_string()));
}

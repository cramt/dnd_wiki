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
        .eq(Ok("<p><strong>damn, this is bold</strong></p>".to_string()));
}

#[test]
fn inner_text_bold() {
    markdown("yo **bold** it be", EmptyReferencer)
        .should()
        .eq(Ok(
            "<p>yo <strong>bold</strong> it be</p>".to_string()
        ));
}

#[test]
fn all_text_italic() {
    markdown("*damn, italy*", EmptyReferencer)
        .should()
        .eq(Ok("<p><em>damn, italy</em></p>".to_string()));
}

#[test]
fn inner_text_italic() {
    markdown("some *wild* shit", EmptyReferencer)
        .should()
        .eq(Ok(
            "<p>some <em>wild</em> shit</p>".to_string()
        ));
}

#[test]
fn all_text_italic_and_bold() {
    markdown("***AAA***", EmptyReferencer)
        .should()
        .eq(Ok("<p><em><strong>AAA</strong></em></p>".to_string()));
}

#[test]
fn inner_text_italic_and_bold() {
    markdown("bruh, ***IDFK*** anymore", EmptyReferencer)
        .should()
        .eq(Ok(
            "<p>bruh, <em><strong>IDFK</strong></em> anymore</p>".to_string()
        ));
}

#[test]
fn all_text_list() {
    markdown(
        "
- First things first
- I'ma say all the words inside my head
- I'm fired up and tired of
- The way that things have been, oh-ooh
- The way that things have been, oh-ooh
    ",
        EmptyReferencer,
    )
    .should()
    .eq(Ok(
        "<ul>\r\n<li>First things first</li>\r\n<li>I\'ma say all the words inside my head</li>\r\n<li>I\'m fired up and tired of</li>\r\n<li>The way that things have been, oh-ooh</li>\r\n<li>The way that things have been, oh-ooh</li>\r\n</ul>"
            .to_string(),
    ));
}

#[test]
fn inner_text_list() {
    markdown(
        "
things that are cool
- you
- me
- other things
bye
    ",
        EmptyReferencer,
    )
    .should()
    .eq(Ok(
        "<p>things that are cool</p>\r\n<ul>\r\n<li>you</li>\r\n<li>me</li>\r\n<li>other things\r\nbye</li>\r\n</ul>".to_string(),
    ));
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
    .eq(Ok("<p>yoo\r\nnew line</p>".to_string()));
}

#[test]
fn list_and_newline() {
    markdown(
        "
yoo
hello
- list time
- aaa
    ",
        EmptyReferencer,
    )
    .should()
    .eq(Ok("<p>yoo\r\nhello</p>\r\n<ul>\r\n<li>list time</li>\r\n<li>aaa</li>\r\n</ul>".to_string()));
}

#[test]
fn references() {
    let r = HashMapReferencer {
        value: "".to_string(),
        children: vec![(
            "me".to_string(),
            HashMapReferencer {
                value: "things".to_string(),
                children: HashMap::new(),
            },
        )]
        .into_iter()
        .collect(),
    };

    markdown("[Me](me)", r)
        .should()
        .be()
        .eq(Ok("<p><a href=\"things\">Me</a></p>".to_string()));
}

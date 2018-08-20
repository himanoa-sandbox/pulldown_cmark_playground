extern crate pulldown_cmark;

use pulldown_cmark::{html, Parser, Event, Tag};
use std::borrow::Cow::{Owned};

fn main() {
    let markdown_str = r#"# Hello
人間は愚かな生物。

[俺のブログ](https://blog.himanoa.net)
"#;
    let parser = Parser::new(markdown_str).map(|event| match event {
        Event::Start(Tag::Link(url, title)) => {
            let replaced_url = url.replace("https", "http");
            Event::Start(Tag::Link(Owned(replaced_url), title))
        },
        _ => event
    });
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("{}", html_buf);
}

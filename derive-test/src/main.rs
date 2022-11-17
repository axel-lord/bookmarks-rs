use bookmark_derive::Storeable;
use bookmark_storage::{content_string::ContentString, Field, ListField};

const TAGS_TOKEN: &str = "<tags>";
const INFO_TOKEN: &str = "<info>";

#[derive(Debug, Storeable)]
struct Test {
    #[line]
    ln: ContentString,

    #[string]
    #[token(INFO_TOKEN)]
    info: Field,

    #[composite(tag)]
    #[token(TAGS_TOKEN)]
    tags: ListField,
}

fn main() {
    println!("Hello, world!");
}

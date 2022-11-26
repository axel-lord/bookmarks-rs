use bookmark_derive::Storeable;
use bookmark_storage::{ContentString, Field, ListField};

const TAGS_TOKEN: &str = "<tags>";
const INFO_TOKEN: &str = "<info>";

#[derive(Debug, Storeable, Default)]
struct Test {
    #[line]
    ln: ContentString,

    #[string]
    #[title]
    #[token(INFO_TOKEN)]
    info: Field,

    #[composite(tag)]
    #[token(TAGS_TOKEN)]
    tags: ListField,
}

fn main() {
    println!("Hello, world!");
}

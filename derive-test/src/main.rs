use bookmark_derive::Storeable;
use bookmark_storage::content_string::ContentString;

#[derive(Debug, Storeable)]
struct Test {
    #[line]
    ln: ContentString,
}

fn main() {
    println!("Hello, world!");
}

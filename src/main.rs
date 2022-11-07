use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    bookmarks_rs::run()
}

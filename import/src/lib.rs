use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bookmark_library::{
    bookmark::Bookmark, command::CommandErr, reset::ResetValues, shared::BufferStorage,
};

#[derive(Debug)]
pub struct OnetabImport;

impl bookmark_library::CommandBuilder for OnetabImport {
    fn name(&self) -> &'static str {
        "onetab-import"
    }
    fn build(
        &mut self,
        BufferStorage(bookmarks, _, _): BufferStorage<Bookmark>,
        _categories: BufferStorage<bookmark_library::category::Category>,
        _infos: BufferStorage<bookmark_library::info::Info>,
        reset_values: ResetValues,
    ) -> Box<dyn bookmark_library::command::Command> {
        Box::new(move |args: &[String]| {
            if args.len() != 1 {
                return Err(CommandErr::Usage(
                    "onetab-import should be called with a single argument".into(),
                ));
            }

            let reader = BufReader::new(File::open(&args[0])?);

            let mut bookmarks = bookmarks.borrow_mut();
            for line in reader.lines() {
                let line = line?;
                let Some(url_size) = line.find(" | ") else {continue;};
                let desc_start = url_size + " | ".len();

                let url = &line[0..url_size];
                let desc = &line[desc_start..];

                bookmarks.push(Bookmark::new(url, desc, std::iter::empty::<&str>()))
            }

            reset_values.reset();

            Ok(())
        })
    }
}

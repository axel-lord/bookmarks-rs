use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    reset, shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct New {
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
    selected: shared::Selected,
}

impl Command for New {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 0 {
            return Err(CommandErr::Usage(
                "new should be called without any arguments".into(),
            ));
        }

        let index = self.bookmarks.len();

        self.bookmarks.borrow_mut().push(Bookmark::new(
            "no url",
            "no info",
            std::iter::empty::<&str>(),
        ));

        reset::reset(
            &mut self.buffer.borrow_mut(),
            &self.bookmarks.borrow(),
            &mut self.selected.borrow_mut(),
        );

        self.selected.replace(index);

        println!(
            "added and selected:\n{index}. {:#}",
            self.selected.get(&self.bookmarks.borrow_mut()).ok_or_else(
                || CommandErr::Execution("failed in using index of added bookmark".into())
            )?
        );

        Ok(())
    }
}

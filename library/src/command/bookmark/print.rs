use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Print {
    bookmarks: shared::Bookmarks,
    selected: shared::Selected,
}

impl Command for Print {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 0 {
            return Err(CommandErr::Usage(
                "print should be called without any arguments".into(),
            ));
        }

        // let Some(index) = self.selected.borrow().clone() else {
        //     return Err(CommandErr::Execution("no bookmark selected".into()));
        // };

        if self.selected.is_empty() {
            return Err(CommandErr::Execution("no bookmark selected".into()));
        }

        let bookmarks = self.bookmarks.borrow();

        let Some(selected) = self.selected.get(&bookmarks) else {
            return Err(CommandErr::Execution("selected bookmark does not exist".into()));
        };

        println!("{}. {:#}", self.selected.index().unwrap(), selected);

        Ok(())
    }
}

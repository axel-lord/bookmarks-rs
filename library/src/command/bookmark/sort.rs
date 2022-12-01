use crate::{
    command::{Command, CommandErr},
    reset::ResetValues,
    shared,
};

pub fn build(bookmarks: shared::Bookmarks, reset_values: ResetValues) -> Box<dyn Command> {
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "sort should be called without any arguments".into(),
            ));
        }

        bookmarks.sort_by(|a, b| a.url().partial_cmp(b.url()).unwrap());
        reset_values.reset();

        Ok(())
    })
}

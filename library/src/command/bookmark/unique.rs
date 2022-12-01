use crate::{
    command::{Command, CommandErr},
    reset::ResetValues,
    shared,
};

pub fn build(bookmarks: shared::Bookmarks, reset_values: ResetValues) -> Box<dyn Command> {
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "unique should be called without any arguments".into(),
            ));
        }

        let mut removed_count = 0usize;
        bookmarks
            .sort_by(|a, b| a.url().partial_cmp(b.url()).unwrap())
            .dedup_by(|a, b| {
                if a.url().eq_ignore_ascii_case(b.url()) {
                    removed_count += 1;
                    true
                } else {
                    false
                }
            });
        reset_values.reset();

        println!("remvoved {removed_count} bookmarks");

        Ok(())
    })
}

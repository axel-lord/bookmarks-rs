use bookmark_storage::Listed;

use crate::{
    command::{Command, CommandErr},
    shared,
};

pub fn wrap_if_negative(number: isize, max: usize) -> Result<usize, CommandErr> {
    if number.unsigned_abs() > max {
        return Err(CommandErr::Execution(format!(
            "number {number} larger than max value {max}"
        )));
    }

    Ok(if number >= 0 {
        number as usize
    } else {
        max - number.unsigned_abs()
    })
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct List<T>
where
    T: Listed + std::fmt::Display,
{
    storage: shared::Storage<T>,
    buffer: shared::Buffer,
}

impl<T> Command for List<T>
where
    T: Listed + std::fmt::Display,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        let items = self.storage.borrow();

        let count = args
            .get(0)
            .map(|arg| arg.parse())
            .unwrap_or_else(|| Ok(self.buffer.count().unwrap_or_else(|| self.storage.len())))
            .map_err(|_| {
                CommandErr::Execution(format!(
                    "could not parse {} as a positive integer",
                    &args[0]
                ))
            })?;

        let from = args
            .get(1)
            .map(|arg| arg.parse())
            .unwrap_or(Ok(0))
            .map_err(|_| {
                CommandErr::Execution(format!("could not parse {} as an integer", &args[1]))
            })
            .map(|from| {
                wrap_if_negative(from, self.buffer.count().unwrap_or(self.storage.len()))
            })??;

        for (index, item) in self
            .buffer
            .iter()
            .map(|i| (i, items.get(i)))
            .take_while(|(_, t)| t.is_some())
            .map(|(i, t)| (i, t.unwrap()))
            .skip(from)
            .take(count)
        {
            println!("{}. {:#}", index, item);
        }

        Ok(())
    }
}

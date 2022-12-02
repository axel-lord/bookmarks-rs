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
    buffer_storage: shared::BufferStorage<T>,
}

impl<T> Command for List<T>
where
    T: Listed + std::fmt::Display,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        let items = self.buffer_storage.storage.read().unwrap();
        let buffer = self.buffer_storage.buffer.read().unwrap();

        let count = args
            .get(0)
            .map(|arg| arg.parse())
            .unwrap_or_else(|| Ok(buffer.count().unwrap_or(items.len())))
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
            .map(|from| wrap_if_negative(from, buffer.count().unwrap_or(items.len())))??;

        for (index, item) in buffer
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

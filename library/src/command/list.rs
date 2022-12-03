use crate::{
    command::{Command, CommandErr},
    shared,
};
use bookmark_command::Command;
use bookmark_storage::Listed;

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

#[derive(Debug, Command)]
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
        let buffer_storage = self.buffer_storage.read().unwrap();

        let count = args
            .get(0)
            .map(|arg| arg.parse())
            .unwrap_or_else(|| {
                Ok(buffer_storage
                    .buffer
                    .count()
                    .unwrap_or(buffer_storage.storage.len()))
            })
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
                wrap_if_negative(
                    from,
                    buffer_storage
                        .buffer
                        .count()
                        .unwrap_or(buffer_storage.storage.len()),
                )
            })??;

        for (index, item) in buffer_storage
            .buffer
            .iter()
            .map(|i| (i, buffer_storage.storage.get(i)))
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

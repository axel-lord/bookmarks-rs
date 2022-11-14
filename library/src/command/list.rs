use crate::command::CommandErr;

pub fn wrap_if_negative(number: isize, max: usize) -> Result<usize, CommandErr> {
    if number.abs() as usize > max {
        return Err(CommandErr::Execution(format!(
            "number {number} larger than max value {max}"
        )));
    }

    Ok(if number >= 0 {
        number as usize
    } else {
        max - number.abs() as usize
    })
}

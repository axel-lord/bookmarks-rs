use crate::{
    command::{Command, CommandErr},
    reset::ResetValues,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Reset {
    reset_values: ResetValues,
}

impl Command for Reset {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Execution(
                "reset should be used without any arguments".into(),
            ));
        }

        self.reset_values.reset();

        Ok(())
    }
}

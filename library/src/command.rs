pub mod bookmark;
pub mod category;
pub mod count;
pub mod info;
pub mod list;
pub mod load;
pub mod print;
pub mod push;
pub mod reset;
pub mod save;
pub mod select;
pub mod set;

pub fn command_debug(args: &[String]) -> Result<(), bookmark_command::CommandErr> {
    println!("{:#?}", args);
    Ok(())
}
